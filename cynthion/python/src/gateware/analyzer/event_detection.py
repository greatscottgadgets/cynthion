 #!/usr/bin/env python3
# pylint: disable=maybe-no-member
#
# This file is part of Cynthion.
#
# Copyright (c) 2025 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from amaranth import Elaboratable, Module, Signal, Mux, Cat, Array

from luna.gateware.usb.usb2            import USBSpeed
from .speeds                           import USBAnalyzerSpeed
from .events                           import USBAnalyzerEvent


class USBFullSpeedEventDetector(Elaboratable):
    """ Gateware that detects events on the USB bus at Full Speed.

    Attributes
    ----------
    vbus_connected: Signal(), input
        Indicates that the device is connected to VBUS.
        be held in perpetual bus reset, and reset handshaking will be disabled.

    line_state: Signal(2), input
        The UTMI linestate signals; used to read the current state of the USB D+ and D- lines.

    event_strobe: Signal(), output
        A strobe that indicates that an event has been detected.

    event_code: Signal(8), output
        A USBAnalyzerEvent value indicating the detected event.
    """

    # Constants for our line states.
    _LINE_STATE_SE0       = 0b00
    _LINE_STATE_FS_K      = 0b10
    _LINE_STATE_FS_J      = 0b01

    # Time constants.
    _CYCLES_500_NANOSECONDS    = 30
    _CYCLES_666_NANOSECONDS    = 40
    _CYCLES_1_MICROSECOND      = _CYCLES_500_NANOSECONDS  * 2
    _CYCLES_1500_NANOSECONDS   = _CYCLES_500_NANOSECONDS  * 3
    _CYCLES_2P5_MICROSECONDS   = _CYCLES_500_NANOSECONDS  * 5
    _CYCLES_50_MICROSECONDS    = _CYCLES_1_MICROSECOND    * 50
    _CYCLES_1_MILLISECONDS     = _CYCLES_1_MICROSECOND    * 1000
    _CYCLES_2P5_MILLISECONDS   = _CYCLES_2P5_MICROSECONDS * 1000
    _CYCLES_3_MILLISECONDS     = _CYCLES_1_MILLISECONDS   * 3
    _CYCLES_20_MILLISECONDS    = _CYCLES_1_MILLISECONDS   * 20

    _VBUS_DEBOUNCE_TIME        = _CYCLES_50_MICROSECONDS


    def __init__(self):
        self.reset              = Signal()
        self.vbus_connected     = Signal()
        self.line_state         = Signal(2)
        self.event_strobe       = Signal()
        self.event_code         = Signal(8)


    def elaborate(self, platform):
        m = Module()

        # Helper function for detecting events.
        def detect_event(code):
            m.d.comb += [
                self.event_strobe.eq(1),
                self.event_code.eq(code),
            ]

        # Whether to generate line state events.
        line_state_events = Signal()

        # Track line state changes, and generate events when enabled.
        last_line_state = Signal.like(self.line_state)
        line_state_time = Signal(range(0, self._CYCLES_20_MILLISECONDS + 1))
        m.d.usb += last_line_state.eq(self.line_state)
        with m.If(self.line_state != last_line_state):
            m.d.usb += line_state_time.eq(0)
            with m.If(line_state_events):
                mapping = Array([
                    USBAnalyzerEvent.LINESTATE_SE0,
                    USBAnalyzerEvent.LINESTATE_FS_J,
                    USBAnalyzerEvent.LINESTATE_FS_K,
                    USBAnalyzerEvent.LINESTATE_SE1,
                ])
                detect_event(mapping[self.line_state])
        with m.Else():
            m.d.usb += line_state_time.eq(line_state_time + 1)

        # Timer for debouncing VBUS going high.
        vbus_high_time = Signal(range(self._VBUS_DEBOUNCE_TIME + 1))
        with m.If(self.vbus_connected):
            m.d.usb += vbus_high_time.eq(vbus_high_time + 1)
        with m.Else():
            m.d.usb += vbus_high_time.eq(0)

        with m.FSM(domain='usb', reset='INITIALIZE') as fsm:

            def handle_vbus_disconnect():
                """
                Helper to handle VBUS disconnect.
                """
                with m.If(~self.vbus_connected):
                    m.next = 'VBUS_INVALID'
                    detect_event(USBAnalyzerEvent.VBUS_INVALID)

            # INITIALIZE -- we're immediately post-reset; we'll perform some minor setup
            with m.State('INITIALIZE'):
                with m.If(self.vbus_connected):
                    m.next = 'DISCONNECT'
                with m.Else():
                    m.next = 'VBUS_INVALID'

            # VBUS_INVALID -- there's no valid VBUS, so await that before anything else.
            with m.State('VBUS_INVALID'):

                # Accept VBUS as valid when it's stayed high for 50us.
                with m.If(vbus_high_time == self._VBUS_DEBOUNCE_TIME):
                    m.next = 'DISCONNECT'
                    detect_event(USBAnalyzerEvent.VBUS_VALID)


            # DISCONNECT -- the device disconnected and now we're waiting to see a bus idle
            # state to indicate a connection.
            with m.State('DISCONNECT'):
                m.d.comb += line_state_events.eq(1)

                # If we see full-speed J-state, go into full-speed mode.
                with m.If((self.line_state == self._LINE_STATE_FS_J) &
                          (line_state_time == self._CYCLES_2P5_MICROSECONDS)):
                    m.next = 'FS_NON_RESET'
                    detect_event(USBAnalyzerEvent.FS_ATTACH)

                handle_vbus_disconnect()


            # FS_NON_RESET -- we're currently operating at FS and waiting for a reset;
            # the device could be active or inactive, but we haven't yet seen a reset condition.
            with m.State('FS_NON_RESET'):

                # If we see SE0, generate a line state event, but it's not a reset yet.
                with m.If(self.line_state == self._LINE_STATE_SE0):
                    with m.If(last_line_state != self._LINE_STATE_SE0):
                        detect_event(USBAnalyzerEvent.LINESTATE_SE0)

                    # If we see an SE0 for >2.5us, this a bus reset. [USB2.0: 7.1.7.5; ULPI 3.8.5.1].
                    with m.If(line_state_time == self._CYCLES_2P5_MICROSECONDS):
                        m.next = 'FS_RESET'
                        detect_event(USBAnalyzerEvent.BUS_RESET)

                with m.If(self.line_state == self._LINE_STATE_FS_J):
                    with m.If(last_line_state == self._LINE_STATE_SE0):
                        detect_event(USBAnalyzerEvent.LINESTATE_FS_J)

                    # If we see 3ms of consecutive line idle, we're being put into USB suspend.
                    # We'll enter our suspended state, directly. [USB2.0: 7.1.7.6]
                    with m.If(line_state_time == self._CYCLES_3_MILLISECONDS):
                        m.next = 'FS_SUSPEND'
                        detect_event(USBAnalyzerEvent.SUSPEND)

                handle_vbus_disconnect()


            # FS_RESET -- we're in an FS bus reset, but it could also be a disconnect.
            with m.State('FS_RESET'):
                m.d.comb += line_state_events.eq(1)

                # If we see a return to FS idle, revert to FS_NON_RESET state.
                with m.If(self.line_state == self._LINE_STATE_FS_J):
                    m.next = 'FS_NON_RESET'

                handle_vbus_disconnect()


            # FS SUSPEND -- our device has entered FS suspend; we'll now wait for either a
            # resume or a reset
            with m.State('FS_SUSPEND'):
                m.d.comb += line_state_events.eq(1)

                # If we see a K state for 20ms, then we're being resumed.
                with m.If((self.line_state == self._LINE_STATE_FS_K) &
                          (line_state_time == self._CYCLES_20_MILLISECONDS)):
                    m.next = 'FS_RESUME'
                    detect_event(USBAnalyzerEvent.RESUME)

                # If we see an SE0 for > 2.5uS, this is a reset request. [USB 2.0: 7.1.7.5]
                with m.If((self.line_state == self._LINE_STATE_SE0) &
                          (line_state_time == self._CYCLES_2P5_MICROSECONDS)):
                    m.next = 'FS_RESET'
                    detect_event(USBAnalyzerEvent.BUS_RESET)

                handle_vbus_disconnect()


            # FS RESUME -- we've detected a resume from FS suspend
            with m.State('FS_RESUME'):
                m.d.comb += line_state_events.eq(1)

                # Return to normal operation when we see end of resume.
                with m.If((self.line_state == self._LINE_STATE_FS_J) &
                          (last_line_state == self._LINE_STATE_SE0) &
                          (line_state_time >= self._CYCLES_666_NANOSECONDS) &
                          (line_state_time <= self._CYCLES_1500_NANOSECONDS)):
                    m.next = 'FS_NON_RESET'

                # If we see an SE0 for > 2.5uS, this is a reset request. [USB 2.0: 7.1.7.5]
                with m.If((self.line_state == self._LINE_STATE_SE0) &
                          (line_state_time == self._CYCLES_2P5_MICROSECONDS)):
                    m.next = 'FS_RESET'
                    detect_event(USBAnalyzerEvent.BUS_RESET)

                handle_vbus_disconnect()


        return m


class USBLowSpeedEventDetector(Elaboratable):
    """ Gateware that detects events on the USB bus at Low Speed.

    Attributes
    ----------
    vbus_connected: Signal(), input
        Indicates that the device is connected to VBUS.
        be held in perpetual bus reset, and reset handshaking will be disabled.

    line_state: Signal(2), input
        The UTMI linestate signals; used to read the current state of the USB D+ and D- lines.

    event_strobe: Signal(), output
        A strobe that indicates that an event has been detected.

    event_code: Signal(8), output
        A USBAnalyzerEvent value indicating the detected event.
    """

    # Constants for our line states.
    _LINE_STATE_SE0       = 0b00
    _LINE_STATE_LS_K      = 0b01
    _LINE_STATE_LS_J      = 0b10

    # Time constants.
    _CYCLES_500_NANOSECONDS    = 30
    _CYCLES_666_NANOSECONDS    = 40
    _CYCLES_1_MICROSECOND      = _CYCLES_500_NANOSECONDS  * 2
    _CYCLES_1500_NANOSECONDS   = _CYCLES_500_NANOSECONDS  * 3
    _CYCLES_2P5_MICROSECONDS   = _CYCLES_500_NANOSECONDS  * 5
    _CYCLES_50_MICROSECONDS    = _CYCLES_1_MICROSECOND    * 50
    _CYCLES_1_MILLISECONDS     = _CYCLES_1_MICROSECOND    * 1000
    _CYCLES_2P5_MILLISECONDS   = _CYCLES_2P5_MICROSECONDS * 1000
    _CYCLES_3_MILLISECONDS     = _CYCLES_1_MILLISECONDS   * 3
    _CYCLES_20_MILLISECONDS    = _CYCLES_1_MILLISECONDS   * 20

    _VBUS_DEBOUNCE_TIME        = _CYCLES_50_MICROSECONDS


    def __init__(self):
        self.reset              = Signal()
        self.vbus_connected     = Signal()
        self.line_state         = Signal(2)
        self.event_strobe       = Signal()
        self.event_code         = Signal(8)


    def elaborate(self, platform):
        m = Module()

        # Helper function for detecting events.
        def detect_event(code):
            m.d.comb += [
                self.event_strobe.eq(1),
                self.event_code.eq(code),
            ]

        # Whether to generate line state events.
        line_state_events = Signal()

        # Track line state changes, and generate events when enabled.
        last_line_state = Signal.like(self.line_state)
        line_state_time = Signal(range(0, self._CYCLES_20_MILLISECONDS + 1))
        m.d.usb += last_line_state.eq(self.line_state)
        with m.If(self.line_state != last_line_state):
            m.d.usb += line_state_time.eq(0)
            with m.If(line_state_events):
                mapping = Array([
                    USBAnalyzerEvent.LINESTATE_SE0,
                    USBAnalyzerEvent.LINESTATE_LS_K,
                    USBAnalyzerEvent.LINESTATE_LS_J,
                    USBAnalyzerEvent.LINESTATE_SE1,
                ])
                detect_event(mapping[self.line_state])
        with m.Else():
            m.d.usb += line_state_time.eq(line_state_time + 1)

        # Timer for debouncing VBUS going high.
        vbus_high_time = Signal(range(self._VBUS_DEBOUNCE_TIME + 1))
        with m.If(self.vbus_connected):
            m.d.usb += vbus_high_time.eq(vbus_high_time + 1)
        with m.Else():
            m.d.usb += vbus_high_time.eq(0)

        with m.FSM(domain='usb', reset='INITIALIZE') as fsm:

            def handle_vbus_disconnect():
                """
                Helper to handle VBUS disconnect.
                """
                with m.If(~self.vbus_connected):
                    m.next = 'VBUS_INVALID'
                    detect_event(USBAnalyzerEvent.VBUS_INVALID)

            # INITIALIZE -- we're immediately post-reset; we'll perform some minor setup
            with m.State('INITIALIZE'):
                with m.If(self.vbus_connected):
                    m.next = 'DISCONNECT'
                with m.Else():
                    m.next = 'VBUS_INVALID'


            # VBUS_INVALID -- there's no valid VBUS, so await that before anything else.
            with m.State('VBUS_INVALID'):

                # Accept VBUS as valid when it's stayed high for 50us.
                with m.If(vbus_high_time == self._VBUS_DEBOUNCE_TIME):
                    m.next = 'DISCONNECT'
                    detect_event(USBAnalyzerEvent.VBUS_VALID)


            # DISCONNECT -- the device disconnected and now we're waiting to see a bus idle
            # state to indicate a connection.
            with m.State('DISCONNECT'):
                m.d.comb += line_state_events.eq(1)

                # If we see LS J-state, go into low-speed mode.
                with m.If((self.line_state == self._LINE_STATE_LS_J) &
                            (line_state_time == self._CYCLES_2P5_MICROSECONDS)):
                    m.next = 'LS_NON_RESET'
                    detect_event(USBAnalyzerEvent.LS_ATTACH)

                handle_vbus_disconnect()


            # LS_NON_RESET -- we're currently operating at LS and waiting for a reset;
            # the device could be active or inactive, but we haven't yet seen a reset condition.
            with m.State('LS_NON_RESET'):

                # If we see an SE0 that lasts between 670 and 1500 ns, this is an LS keepalive.
                with m.If((self.line_state == self._LINE_STATE_LS_J) &
                          (last_line_state == self._LINE_STATE_SE0) &
                          (line_state_time >= self._CYCLES_666_NANOSECONDS) &
                          (line_state_time <= self._CYCLES_1500_NANOSECONDS)):
                    detect_event(USBAnalyzerEvent.LS_KEEPALIVE)

                # If we see an SE0 for >2.5us, this a bus reset. [USB2.0: 7.1.7.5; ULPI 3.8.5.1].
                with m.If((self.line_state == self._LINE_STATE_SE0) &
                          (line_state_time == self._CYCLES_2P5_MICROSECONDS)):
                    m.next = 'LS_RESET'
                    detect_event(USBAnalyzerEvent.BUS_RESET)

                # If we see 3ms of consecutive line idle, we're being put into USB suspend.
                # We'll enter our suspended state, directly. [USB2.0: 7.1.7.6]
                with m.If((self.line_state == self._LINE_STATE_LS_J) &
                          (line_state_time == self._CYCLES_3_MILLISECONDS)):
                    m.next = 'LS_SUSPEND'
                    detect_event(USBAnalyzerEvent.SUSPEND)

                handle_vbus_disconnect()


            # LS_RESET -- we're in an LS bus reset, but it could also be a disconnect.
            with m.State('LS_RESET'):
                m.d.comb += line_state_events.eq(1)

                # If we see a return to LS idle, revert to LS_NON_RESET state.
                with m.If(self.line_state == self._LINE_STATE_LS_J):
                    m.next = 'LS_NON_RESET'

                handle_vbus_disconnect()


            # LS SUSPEND -- our device has entered LS suspend; we'll now wait for either a
            # resume or a reset
            with m.State('LS_SUSPEND'):
                m.d.comb += line_state_events.eq(1)

                # If we see a K state for 20ms, then we're being resumed.
                with m.If((self.line_state == self._LINE_STATE_LS_K) &
                          (line_state_time == self._CYCLES_20_MILLISECONDS)):
                    m.next = 'LS_RESUME'
                    detect_event(USBAnalyzerEvent.RESUME)

                # If we see an SE0 for > 2.5uS, this is a reset request. [USB 2.0: 7.1.7.5]
                with m.If((self.line_state == self._LINE_STATE_SE0) &
                          (line_state_time == self._CYCLES_2P5_MICROSECONDS)):
                    m.next = 'LS_RESET'
                    detect_event(USBAnalyzerEvent.BUS_RESET)

                handle_vbus_disconnect()


            # LS RESUME -- we've detected a resume from LS suspend
            with m.State('LS_RESUME'):
                m.d.comb += line_state_events.eq(1)

                # Return to LS when we see end of resume.
                with m.If((self.line_state == self._LINE_STATE_LS_J) &
                          (last_line_state == self._LINE_STATE_SE0) &
                          (line_state_time >= self._CYCLES_666_NANOSECONDS) &
                          (line_state_time <= self._CYCLES_1500_NANOSECONDS)):
                    m.next = 'LS_NON_RESET'

                # If we see an SE0 for > 2.5uS, this is a reset request. [USB 2.0: 7.1.7.5]
                with m.If((self.line_state == self._LINE_STATE_SE0) &
                          (line_state_time == self._CYCLES_2P5_MICROSECONDS)):
                    m.next = 'LS_RESET'
                    detect_event(USBAnalyzerEvent.BUS_RESET)

                handle_vbus_disconnect()


        return m


class USBHighSpeedEventDetector(Elaboratable):
    """ Gateware that detects events on the USB bus at High Speed.

    Attributes
    ----------
    vbus_connected: Signal(), input
        Indicates that the device is connected to VBUS.
        be held in perpetual bus reset, and reset handshaking will be disabled.

    event_strobe: Signal(), output
        A strobe that indicates that an event has been detected.

    event_code: Signal(8), output
        A USBAnalyzerEvent value indicating the detected event.
    """

    # Time constants.
    _CYCLES_500_NANOSECONDS    = 30
    _CYCLES_1_MICROSECOND      = _CYCLES_500_NANOSECONDS  * 2
    _CYCLES_50_MICROSECONDS    = _CYCLES_1_MICROSECOND    * 50

    _VBUS_DEBOUNCE_TIME        = _CYCLES_50_MICROSECONDS


    def __init__(self):
        self.reset              = Signal()
        self.vbus_connected     = Signal()
        self.event_strobe       = Signal()
        self.event_code         = Signal(8)


    def elaborate(self, platform):
        m = Module()

        # Helper function for detecting events.
        def detect_event(code):
            m.d.comb += [
                self.event_strobe.eq(1),
                self.event_code.eq(code),
            ]

        # Timer for debouncing VBUS going high.
        vbus_high_time = Signal(range(self._VBUS_DEBOUNCE_TIME + 1))
        with m.If(self.vbus_connected):
            m.d.usb += vbus_high_time.eq(vbus_high_time + 1)
        with m.Else():
            m.d.usb += vbus_high_time.eq(0)

        with m.FSM(domain='usb', reset='INITIALIZE') as fsm:

            # INITIALIZE -- we're immediately post-reset; we'll perform some minor setup
            with m.State('INITIALIZE'):
                with m.If(self.vbus_connected):
                    m.next = 'HS_LISTEN'
                with m.Else():
                    m.next = 'VBUS_INVALID'


            # VBUS_INVALID -- there's no valid VBUS, so await that before anything else.
            with m.State('VBUS_INVALID'):

                # Accept VBUS as valid when it's stayed high for 50us.
                with m.If(vbus_high_time == self._VBUS_DEBOUNCE_TIME):
                    m.next = 'HS_LISTEN'
                    detect_event(USBAnalyzerEvent.VBUS_VALID)


            # HS_LISTEN -- we're listening for HS traffic.
            with m.State('HS_LISTEN'):

                with m.If(~self.vbus_connected):
                    m.next = 'VBUS_INVALID'
                    detect_event(USBAnalyzerEvent.VBUS_INVALID)


        return m
