 #!/usr/bin/env python3
# pylint: disable=maybe-no-member
#
# This file is part of Cynthion.
#
# Copyright (c) 2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from amaranth import Elaboratable, Module, Signal, Mux, Cat, Array

from luna.gateware.usb.usb2            import USBSpeed
from .speeds                           import USBAnalyzerSpeed
from .events                           import USBAnalyzerEvent


class USBAnalyzerSpeedDetector(Elaboratable):
    """ Gateware that detects reset signaling on the USB bus.

    Attributes
    ----------
    vbus_connected: Signal(), input
        Indicates that the device is connected to VBUS. When this is de-asserted, the device will
        be held in perpetual bus reset, and reset handshaking will be disabled.
    line_state: Signal(2), input
        The UTMI linestate signals; used to read the current state of the USB D+ and D- lines.

    phy_speed: Signal(2), output
        A USBSpeed value used to drive our PHY's speed selection.

    detected_speed: Signal(2), output
        A USBAnalyzer speed value used to indicate detected speed.

    event_strobe: Signal(), output
        A strobe that indicates that an event has been detected.

    event_code: Signal(8), output
        A USBAnalyzerEvent value indicating the detected event.
    """

    # Constants for our line states at various speeds.
    _LINE_STATE_SE0       = 0b00
    _LINE_STATE_SQUELCH   = 0b00
    _LINE_STATE_FS_HS_K   = 0b10
    _LINE_STATE_FS_HS_J   = 0b01
    _LINE_STATE_LS_K      = 0b01
    _LINE_STATE_LS_J      = 0b10

    # Reset time constants.
    # Eventually, if we support clocks other than 60MHz (48 MHz)?
    # We should provide the ability to scale these.
    _CYCLES_500_NANOSECONDS    = 30
    _CYCLES_666_NANOSECONDS    = 40
    _CYCLES_1_MICROSECOND      = _CYCLES_500_NANOSECONDS  * 2
    _CYCLES_1500_NANOSECONDS   = _CYCLES_500_NANOSECONDS  * 3
    _CYCLES_2P5_MICROSECONDS   = _CYCLES_500_NANOSECONDS  * 5
    _CYCLES_5_MICROSECONDS     = _CYCLES_1_MICROSECOND    * 5
    _CYCLES_50_MICROSECONDS    = _CYCLES_1_MICROSECOND    * 50
    _CYCLES_200_MICROSECONDS   = _CYCLES_1_MICROSECOND    * 200
    _CYCLES_1_MILLISECONDS     = _CYCLES_1_MICROSECOND    * 1000
    _CYCLES_2_MILLISECONDS     = _CYCLES_1_MILLISECONDS   * 2
    _CYCLES_2P5_MILLISECONDS   = _CYCLES_2P5_MICROSECONDS * 1000
    _CYCLES_3_MILLISECONDS     = _CYCLES_1_MILLISECONDS   * 3
    _CYCLES_7_MILLISECONDS     = _CYCLES_1_MILLISECONDS   * 7

    _VBUS_DEBOUNCE_TIME        = _CYCLES_50_MICROSECONDS


    def __init__(self):

        #
        # I/O port
        #
        self.reset              = Signal()
        self.vbus_connected     = Signal()
        self.line_state         = Signal(2)
        self.usb_dp             = Signal()
        self.usb_dm             = Signal()

        self.phy_speed          = Signal(2, reset=USBSpeed.FULL)
        self.detected_speed     = Signal(2, reset=USBAnalyzerSpeed.AUTO)

        self.event_strobe       = Signal()
        self.event_code         = Signal(8)


    def elaborate(self, platform):
        m = Module()

        # Event timer: keeps track of the timing of each of the individual event phases.
        timer = Signal(range(0, self._CYCLES_7_MILLISECONDS + 1))

        # Valid pairs: keeps track of how make Chirp K / Chirp J sequences we've
        # seen, thus far.
        valid_pairs = Signal(range(0, 4))

        # Tracks whether we were at high speed when we entered a suspend state.
        was_hs_pre_suspend = Signal()

        # By default, always count forward in time.
        # We'll reset the timer below when appropriate.
        m.d.usb += timer.eq(timer + 1)

        # Helper functions for detecting events and speed changes.

        def detect_event(code):
            m.d.comb += [
                self.event_strobe.eq(1),
                self.event_code.eq(code),
            ]

        def detect_speed(new_speed):
            with m.If(new_speed != self.detected_speed):
                m.d.usb += self.detected_speed.eq(new_speed)
                detect_event(USBAnalyzerEvent.SPEED_DETECT_BASE | new_speed)

        # Line states detected at HS chirp signal levels.
        chirp_j =  self.usb_dp & ~self.usb_dm
        chirp_k = ~self.usb_dp &  self.usb_dm

        # Whether to generate chirp events.
        chirp_events = Signal()

        # Track chirp state changes, and generate events when enabled.
        chirp_state = Cat(self.usb_dp, self.usb_dm)
        last_chirp_state = Signal.like(chirp_state)
        chirp_state_time = Signal.like(timer)
        m.d.usb += last_chirp_state.eq(chirp_state)
        with m.If(chirp_state != last_chirp_state):
            m.d.usb += chirp_state_time.eq(0)
            with m.If(chirp_events):
                detect_event(USBAnalyzerEvent.LINESTATE_BASE + chirp_state)
        with m.Else():
            m.d.usb += chirp_state_time.eq(chirp_state_time + 1)

        # Whether to generate line state events.
        line_state_events = Signal()
        # Whether to interpret line states as Low Speed ones.
        low_speed_line_states = Signal()

        # Track line state changes, and generate events when enabled.
        last_line_state = Signal.like(self.line_state)
        line_state_time = Signal.like(timer)
        m.d.usb += last_line_state.eq(self.line_state)
        with m.If(self.line_state != last_line_state):
            m.d.usb += line_state_time.eq(0)
            with m.If(line_state_events):
                with m.If(low_speed_line_states):
                    mapping = Array([
                        USBAnalyzerEvent.LINESTATE_SE0,
                        USBAnalyzerEvent.LINESTATE_LS_K,
                        USBAnalyzerEvent.LINESTATE_LS_J,
                        USBAnalyzerEvent.LINESTATE_SE1,
                    ])
                    detect_event(mapping[self.line_state])
                with m.Else():
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

        #
        # Core reset sequences.
        #
        with m.FSM(domain='usb', reset='INITIALIZE') as fsm:

            # Helper functions for state transitions.

            def enter_fs_reset():
                """
                Helper to enter the FS reset sequence.
                """
                m.next = 'AWAIT_DEVICE_CHIRP_START'
                m.d.usb += [
                    self.phy_speed.eq(USBSpeed.FULL),
                    timer.eq(0),
                    valid_pairs.eq(0),
                ]
                detect_event(USBAnalyzerEvent.BUS_RESET)

            def handle_ls_connect():
                """
                Helper to be called from FS reset states,
                i.e PHY is in FS mode and line state was previously SE0.
                """
                # If we see FS K-state, it's equivalent to LS J-state, this SE0 was a
                # disconnect rather than a reset, and an LS device has just connected.
                with m.If(self.line_state == self._LINE_STATE_FS_HS_K):
                    m.d.comb += low_speed_line_states.eq(1)
                    with m.If(line_state_time == self._CYCLES_2P5_MICROSECONDS):
                        m.d.usb += self.phy_speed.eq(USBSpeed.LOW)
                        m.next = 'LS_NON_RESET'
                        detect_event(USBAnalyzerEvent.LS_ATTACH)

            def handle_vbus_disconnect():
                """
                Helper to handle VBUS disconnect.
                """
                with m.If(~self.vbus_connected):
                    m.d.usb += self.phy_speed.eq(USBSpeed.FULL),
                    m.next = 'VBUS_INVALID'
                    detect_event(USBAnalyzerEvent.VBUS_INVALID)

            # INITIALIZE -- we're immediately post-reset; we'll perform some minor setup
            with m.State('INITIALIZE'):
                m.d.usb += self.phy_speed.eq(USBSpeed.FULL)
                with m.If(self.vbus_connected):
                    m.next = 'DISCONNECT'
                with m.Else():
                    m.next = 'VBUS_INVALID'

            # VBUS_INVALID -- there's no valid VBUS, so await that before anything else.
            with m.State('VBUS_INVALID'):

                # Speed is unknown for now.
                detect_speed(USBAnalyzerSpeed.AUTO)

                # Accept VBUS as valid when it's stayed high for 50us.
                with m.If(vbus_high_time == self._VBUS_DEBOUNCE_TIME):
                    m.next = 'DISCONNECT'
                    detect_event(USBAnalyzerEvent.VBUS_VALID)


            # DISCONNECT -- the device disconnected and now we're waiting to see a bus idle
            # state to indicate a connection.
            #
            # In this state, the PHY is set to full-speed mode as we're only looking at line_state.
            with m.State('DISCONNECT'):
                m.d.comb += line_state_events.eq(1)

                # Speed is unknown for now.
                detect_speed(USBAnalyzerSpeed.AUTO)

                # If we see full-speed J-state, go into full-speed mode.
                with m.If(self.line_state == self._LINE_STATE_FS_HS_J):
                    with m.If(line_state_time == self._CYCLES_2P5_MICROSECONDS):
                        m.next = 'FS_NON_RESET'
                        detect_event(USBAnalyzerEvent.FS_ATTACH)

                # If we see FS K-state, it's equivalent to LS J-state, go into low-speed mode.
                with m.Elif(self.line_state == self._LINE_STATE_FS_HS_K):
                    m.d.comb += low_speed_line_states.eq(1)
                    with m.If(line_state_time == self._CYCLES_2P5_MICROSECONDS):
                        m.d.usb += self.phy_speed.eq(USBSpeed.LOW)
                        m.next = 'LS_NON_RESET'
                        detect_event(USBAnalyzerEvent.LS_ATTACH)

                handle_vbus_disconnect()


            # LS_NON_RESET -- we're currently operating at LS and waiting for a reset;
            # the device could be active or inactive, but we haven't yet seen a reset condition.
            with m.State('LS_NON_RESET'):

                detect_speed(USBAnalyzerSpeed.LOW)

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
                m.d.comb += [
                    line_state_events.eq(1),
                    low_speed_line_states.eq(1),
                ]

                # If we come out of SE0 into the LS K state (same as HS J), then this was a
                # disconnect and there's a new FS or HS device connecting.
                with m.If(self.line_state == self._LINE_STATE_LS_K):
                    m.d.comb += low_speed_line_states.eq(0)
                    with m.If(line_state_time == self._CYCLES_2P5_MICROSECONDS):
                        m.d.usb += self.phy_speed.eq(USBSpeed.FULL)
                        m.next = 'FS_NON_RESET'
                        detect_event(USBAnalyzerEvent.FS_ATTACH)

                # If we come out of SE0 into any other line state, revert to LS_NON_RESET state.
                with m.Elif(self.line_state != self._LINE_STATE_SE0):
                    m.next = 'LS_NON_RESET'

                handle_vbus_disconnect()


            # FS_NON_RESET -- we're currently operating at FS and waiting for a reset;
            # the device could be active or inactive, but we haven't yet seen a reset condition.
            with m.State('FS_NON_RESET'):

                detect_speed(USBAnalyzerSpeed.FULL)

                # If we see SE0, generate a line state event, but it's not a reset yet.
                with m.If(self.line_state == self._LINE_STATE_SE0):
                    with m.If(last_line_state != self._LINE_STATE_SE0):
                        detect_event(USBAnalyzerEvent.LINESTATE_SE0)

                    # If we see an SE0 for >2.5us, this a bus reset. [USB2.0: 7.1.7.5; ULPI 3.8.5.1].
                    with m.If(line_state_time == self._CYCLES_2P5_MICROSECONDS):
                        enter_fs_reset()

                # If we see 3ms of consecutive line idle, we're being put into USB suspend.
                # We'll enter our suspended state, directly. [USB2.0: 7.1.7.6]
                with m.If((self.line_state == self._LINE_STATE_FS_HS_J) &
                          (line_state_time == self._CYCLES_3_MILLISECONDS)):
                    m.d.usb += was_hs_pre_suspend.eq(0)
                    m.next = 'FS_SUSPEND'
                    detect_event(USBAnalyzerEvent.SUSPEND)

                handle_vbus_disconnect()


            # HS_NON_RESET -- we're currently operating at high speed and waiting for a reset or
            # suspend; the device could be active or inactive.
            with m.State('HS_NON_RESET'):

                # High speed signaling presents IDLE and RESET the same way: with the host
                # driving SE0; and us seeing SQUELCH. [USB2.0: 7.1.1.3; USB2.0: 7.1.7.6].
                # Either way, our next step is the same: we'll drop down to full-speed. [USB2.0: 7.1.7.6]
                # Afterwards, we'll take steps to differentiate a reset from a suspend.
                with m.If((self.line_state == self._LINE_STATE_SQUELCH) &
                          (line_state_time == self._CYCLES_3_MILLISECONDS)):
                    m.d.usb += [
                        timer.eq(0),
                        self.phy_speed.eq(USBSpeed.FULL),
                    ]
                    m.next = 'DETECT_HS_SUSPEND'

                handle_vbus_disconnect()


            # AWAIT_DEVICE_CHIRP_START -- the device may produce a 'chirp' K,
            # which advertises to the host that it's high speed capable.
            with m.State('AWAIT_DEVICE_CHIRP_START'):
                m.d.comb += chirp_events.eq(1)
                m.d.comb += line_state_events.eq(1)

                with m.If(chirp_k & (chirp_state_time == self._CYCLES_2P5_MICROSECONDS)):
                    # The host must detect the device chirp after it has seen
                    # assertion of the Chirp K for no less than 2.5us
                    # [USB2.0: 7.1.7.5]
                    m.next = 'AWAIT_DEVICE_CHIRP_END'
                    detect_event(USBAnalyzerEvent.DEVICE_CHIRP_VALID)

                with m.If(timer == self._CYCLES_7_MILLISECONDS):
                    m.next = 'IS_LOW_OR_FULL_SPEED'

                handle_ls_connect()
                handle_vbus_disconnect()


            # AWAIT_DEVICE_CHIRP_END -- we've seen the device chirp and are
            # waiting for it to end.
            with m.State('AWAIT_DEVICE_CHIRP_END'):
                m.d.comb += chirp_events.eq(1)
                m.d.comb += line_state_events.eq(1)

                with m.If(~chirp_k):
                    # The return to SE0 signals the end of the chirp and
                    # we should await the host chirp.
                    m.d.usb += timer.eq(0)
                    m.next = 'AWAIT_HOST_K'

                with m.If(timer == self._CYCLES_7_MILLISECONDS):
                    m.next = 'IS_LOW_OR_FULL_SPEED'

                handle_ls_connect()
                handle_vbus_disconnect()


            # AWAIT_HOST_K -- we've now completed the device chirp; and are waiting to see if the host
            # will respond with an alternating sequence of K's and J's.
            with m.State('AWAIT_HOST_K'):
                m.d.comb += chirp_events.eq(1)
                m.d.comb += line_state_events.eq(1)

                # If we don't see our response within 2.5ms, this isn't a compliant HS host. [USB2.0: 7.1.7.5].
                # This is thus a full-speed host, and we'll act as a full-speed device.
                with m.If(timer == self._CYCLES_2P5_MILLISECONDS):
                    m.next = 'IS_LOW_OR_FULL_SPEED'

                # Once we've seen our K, we're good to start observing J/K toggles.
                with m.If(chirp_k):
                    m.next = 'IN_HOST_K'

                handle_ls_connect()
                handle_vbus_disconnect()


            # IN_HOST_K: we're seeing a host Chirp K as part of our handshake; we'll
            # time it and see how long it lasts
            with m.State('IN_HOST_K'):
                m.d.comb += chirp_events.eq(1)
                m.d.comb += line_state_events.eq(1)

                # If we've exceeded our minimum chirp time, consider this a valid pattern
                # bit, # and advance in the pattern.
                with m.If(chirp_state_time == self._CYCLES_2P5_MICROSECONDS):
                    m.next = 'AWAIT_HOST_J'

                # If our input has become something other than a K, then
                # we haven't finished our sequence. We'll go back to expecting a K.
                with m.If(~chirp_k):
                    m.next = 'AWAIT_HOST_K'

                # Time out if we exceed our maximum allowed duration.
                with m.If(timer == self._CYCLES_2P5_MILLISECONDS):
                    m.next = 'IS_LOW_OR_FULL_SPEED'

                handle_ls_connect()
                handle_vbus_disconnect()


            # AWAIT_HOST_J -- we're waiting for the next Chirp J in the host chirp sequence
            with m.State('AWAIT_HOST_J'):
                m.d.comb += chirp_events.eq(1)
                m.d.comb += line_state_events.eq(1)

                # If we've exceeded our maximum wait, this isn't a high speed host.
                with m.If(timer == self._CYCLES_2P5_MILLISECONDS):
                    m.next = 'IS_LOW_OR_FULL_SPEED'

                # Once we've seen our J, start timing its duration.
                with m.If(chirp_j):
                    m.next = 'IN_HOST_J'

                handle_ls_connect()
                handle_vbus_disconnect()


            # IN_HOST_J: we're seeing a host Chirp K as part of our handshake; we'll
            # time it and see how long it lasts
            with m.State('IN_HOST_J'):
                m.d.comb += chirp_events.eq(1)
                m.d.comb += line_state_events.eq(1)

                # If we've exceeded our minimum chirp time, consider this a valid pattern
                # bit, and advance in the pattern.
                with m.If(chirp_state_time == self._CYCLES_2P5_MICROSECONDS):

                    # If this would complete our third pair, this completes a handshake,
                    # and we've identified a high speed host!
                    with m.If(valid_pairs == 2):
                        m.next = 'IS_HIGH_SPEED'
                        detect_event(USBAnalyzerEvent.HOST_CHIRP_VALID)

                    # Otherwise, count the pair as valid, and wait for the next K.
                    with m.Else():
                        m.d.usb += valid_pairs.eq(valid_pairs + 1)
                        m.next = 'AWAIT_HOST_K'

                # If our input has become something other than a K, then
                # we haven't finished our sequence. We'll go back to expecting a K.
                with m.If(~chirp_j):
                    m.next = 'AWAIT_HOST_J'

                # Time out if we exceed our maximum allowed duration.
                with m.If(timer == self._CYCLES_2P5_MILLISECONDS):
                    m.next = 'IS_LOW_OR_FULL_SPEED'

                handle_ls_connect()
                handle_vbus_disconnect()


            # IS_HIGH_SPEED -- we've completed a high speed handshake, and are ready to
            # switch to high speed signaling
            with m.State('IS_HIGH_SPEED'):

                # Switch to high speed.
                m.d.usb += self.phy_speed.eq(USBSpeed.HIGH)

                # Signal HS detection.
                detect_speed(USBAnalyzerSpeed.HIGH)

                m.next = 'HS_NON_RESET'


            # IS_LOW_OR_FULL_SPEED -- we've decided the device is low/full speed (typically
            # because it didn't) complete our high-speed handshake; set it up accordingly.
            with m.State('IS_LOW_OR_FULL_SPEED'):
                m.d.comb += line_state_events.eq(1)

                # If we see a return to FS idle, FS operation is now confirmed.
                with m.If(self.line_state == self._LINE_STATE_FS_HS_J):
                    m.next = 'FS_NON_RESET'

                handle_ls_connect()
                handle_vbus_disconnect()


            # DETECT_HS_SUSPEND -- we were operating at high speed, and just detected an event
            # which is either a reset or a suspend event; we'll now detect which.
            with m.State('DETECT_HS_SUSPEND'):

                # We've just switch from HS signaling to FS signaling.
                # We'll wait a little while for the bus to settle, and then
                # check to see if it's settled to FS idle; or if we still see SE0.
                with m.If(timer == self._CYCLES_200_MICROSECONDS):

                    # If we've resume IDLE, this is suspend. Move to HS suspend.
                    with m.If(self.line_state == self._LINE_STATE_FS_HS_J):
                        m.d.usb += was_hs_pre_suspend.eq(1)
                        m.next = 'FS_SUSPEND'
                        detect_event(USBAnalyzerEvent.SUSPEND)

                    # Otherwise, this is a reset (or, if K/SE1, we're very confused, and
                    # should re-initialize anyway). Move to the HS reset detect sequence.
                    with m.Else():
                        enter_fs_reset()

                handle_vbus_disconnect()


            # FS SUSPEND -- our device has entered FS suspend; we'll now wait for either a
            # resume or a reset
            with m.State('FS_SUSPEND'):
                m.d.comb += line_state_events.eq(1)

                # If we see a K state, then we're being resumed.
                is_fs_k = (self.line_state == self._LINE_STATE_FS_HS_K)
                with m.If(is_fs_k):

                    # If we were in high-speed pre-suspend, then resume being in HS.
                    with m.If(was_hs_pre_suspend):
                        m.next = 'IS_HIGH_SPEED'

                    # Otherwise, just resume.
                    with m.Else():
                        m.next = 'FS_NON_RESET'

                    detect_event(USBAnalyzerEvent.RESUME)

                # If we see an SE0 for > 2.5uS, this is a reset request. [USB 2.0: 7.1.7.5]
                with m.If((self.line_state == self._LINE_STATE_SE0) &
                          (line_state_time == self._CYCLES_2P5_MICROSECONDS)):
                    enter_fs_reset()

                handle_vbus_disconnect()


            # LS SUSPEND -- our device has entered LS suspend; we'll now wait for either a
            # resume or a reset
            with m.State('LS_SUSPEND'):
                m.d.comb += [
                    line_state_events.eq(1),
                    low_speed_line_states.eq(1),
                ]

                # If we see a K state, then we're being resumed.
                is_ls_k = (self.line_state == self._LINE_STATE_LS_K)
                with m.If(is_ls_k):
                    m.next = 'LS_NON_RESET'

                    detect_event(USBAnalyzerEvent.RESUME)

                # If we see an SE0 for > 2.5uS, this is a reset request. [USB 2.0: 7.1.7.5]
                with m.If((self.line_state == self._LINE_STATE_SE0) &
                          (line_state_time == self._CYCLES_2P5_MICROSECONDS)):
                    enter_fs_reset()

                handle_vbus_disconnect()

        return m
