#!/usr/bin/env python3
# pylint: disable=maybe-no-member
#
# This file is part of Cynthion.
#
# Copyright (c) 2024 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-3-Clause

from amaranth import Elaboratable, Module, Signal, Mux

from luna.gateware.usb.usb2            import USBSpeed
from .speeds                           import USBAnalyzerSpeed


class USBAnalyzerSpeedDetector(Elaboratable):
    """ Gateware that detects reset signaling on the USB bus.

    Attributes
    ----------
    vbus_connected: Signal(), input
        Indicates that the device is connected to VBUS. When this is de-asserted, the device will
        be held in perpetual bus reset, and reset handshaking will be disabled.
    line_state: Signal(2), input
        The UTMI linestate signals; used to read the current state of the USB D+ and D- lines.

    bus_reset: Signal(), output
        Strobe; pulses high for one cycle when a bus reset is detected. This signal indicates that the
         device should return to unaddressed, unconfigured, and should not longer be in High Speed mode.
    suspended: Signal(), output
        Held high while the USB device should be in suspend. This technically indicates that the device should
        drop down to consuming suspend current (<= 2.5mA), but very few devices are compliant with this requirement.
        Either way, a polite device might reduce its power consumption while in suspend.

    phy_speed: Signal(2), output
        A USBSpeed value used to drive our PHY's speed selection.

    detected_speed: Signal(2), output
        A USBAnalyzer speed value used to indicate detected speed.

    speed_changing: Signal(), output
        A strobe that indicates that a speed change condition has been detected and detected_speed is about to change.

    next_speed: Signal(2), output
        A USBAnalyzerSpeed value that indicates the new speed when speed_changing is set.
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
    _CYCLES_1_MICROSECOND      = _CYCLES_500_NANOSECONDS  * 2
    _CYCLES_2P5_MICROSECONDS   = _CYCLES_500_NANOSECONDS  * 5
    _CYCLES_5_MICROSECONDS     = _CYCLES_1_MICROSECOND    * 5
    _CYCLES_200_MICROSECONDS   = _CYCLES_1_MICROSECOND    * 200
    _CYCLES_1_MILLISECONDS     = _CYCLES_1_MICROSECOND    * 1000
    _CYCLES_2_MILLISECONDS     = _CYCLES_1_MILLISECONDS   * 2
    _CYCLES_2P5_MILLISECONDS   = _CYCLES_2P5_MICROSECONDS * 1000
    _CYCLES_3_MILLISECONDS     = _CYCLES_1_MILLISECONDS   * 3
    _CYCLES_7_MILLISECONDS     = _CYCLES_1_MILLISECONDS   * 7


    def __init__(self):

        #
        # I/O port
        #
        self.reset              = Signal()
        self.vbus_connected     = Signal()
        self.line_state         = Signal(2)
        self.usb_diff           = Signal()

        self.bus_reset          = Signal()
        self.suspended          = Signal()

        self.phy_speed          = Signal(2, reset=USBSpeed.FULL)
        self.detected_speed     = Signal(2, reset=USBAnalyzerSpeed.AUTO)

        self.speed_changing     = Signal()
        self.next_speed         = Signal(2)


    def elaborate(self, platform):
        m = Module()

        # Event timer: keeps track of the timing of each of the individual event phases.
        timer = Signal(range(0, self._CYCLES_7_MILLISECONDS + 1))

        # Line state timer: keeps track of how long we've seen a line-state of interest;
        # other than a reset SE0. Used to track chirp and idle times.
        line_state_time = Signal(range(0, self._CYCLES_7_MILLISECONDS + 1))

        # Valid pairs: keeps track of how make Chirp K / Chirp J sequences we've
        # seen, thus far.
        valid_pairs = Signal(range(0, 4))

        # Tracks whether we were at high speed when we entered a suspend state.
        was_hs_pre_suspend = Signal()

        # By default, always count forward in time.
        # We'll reset the timer below when appropriate.
        m.d.usb += timer.eq(timer + 1)
        m.d.usb += line_state_time.eq(line_state_time + 1)

        # Signal that indicates when the bus is idle.
        # Our bus's IDLE condition depends on our active speed.
        bus_idle = Signal()

        chirp_j = self.usb_diff
        chirp_k = ~self.usb_diff

        # High speed busses present SE0 (which we see as SQUELCH'd) when idle [USB2.0: 7.1.1.3].
        with m.If(self.phy_speed == USBSpeed.HIGH):
            m.d.comb += bus_idle.eq(self.line_state == self._LINE_STATE_SQUELCH)

        # Full and low-speed busses see a 'J' state when idle, due to the device pull-up restistors.
        # (The line_state values for these are flipped between speeds.) [USB2.0: 7.1.7.4.1; USB2.0: Table 7-2].
        with m.Elif(self.phy_speed == USBSpeed.FULL):
            m.d.comb += bus_idle.eq(self.line_state == self._LINE_STATE_FS_HS_J)
        with m.Else():
            m.d.comb += bus_idle.eq(self.line_state == self._LINE_STATE_LS_J)


        chirp_seen = Signal()

        #
        # Core reset sequences.
        #
        with m.FSM(domain='usb', reset='INITIALIZE') as fsm:

            # INITIALIZE -- we're immediately post-reset; we'll perform some minor setup
            with m.State('INITIALIZE'):
                m.next = 'DISCONNECT'
                m.d.usb += [
                    timer.eq(0),
                    line_state_time.eq(0),
                    self.phy_speed.eq(USBSpeed.FULL),
                ]

            # DISCONNECT -- the device disconnected and now we're waiting to see a bus idle
            # state to indicate a connection.
            #
            # In this state, the PHY is set to full-speed mode as we're only looking at line_state.
            with m.State('DISCONNECT'):
                # If we see full-speed J-state, go into full-speed mode.
                with m.If(self.line_state == self._LINE_STATE_FS_HS_J):
                    with m.If(line_state_time == self._CYCLES_2P5_MICROSECONDS):
                        m.next = 'FS_NON_RESET'

                # If we see FS K-state, it's equivalent to LS J-state, go into low-speed mode.
                with m.Elif(self.line_state == self._LINE_STATE_FS_HS_K):
                    with m.If(line_state_time == self._CYCLES_2P5_MICROSECONDS):
                        m.d.usb += self.phy_speed.eq(USBSpeed.LOW)
                        m.next = 'LS_NON_RESET'
                        self.detect_speed(m, USBAnalyzerSpeed.LOW)

                with m.Else():
                    m.d.usb += line_state_time.eq(0)

            # LS_NON_RESET -- we're currently operating at LS and waiting for a reset;
            # the device could be active or inactive, but we haven't yet seen a reset condition.
            with m.State('LS_NON_RESET'):

                # If we're seeing a state other than SE0 (D+ / D- at zero), this isn't yet a
                # potential reset. Keep our timer at zero.
                with m.If(self.line_state != self._LINE_STATE_SE0):
                    m.d.usb += timer.eq(0)


                # If VBUS isn't connected, don't go through the whole reset process;
                # but also consider ourselves permanently in reset. This ensures we
                # don't progress through the reset FSM; but also ensures the device
                # state starts fresh with each plug.
                with m.If(~self.vbus_connected):
                    m.d.usb  += [
                        timer.eq(0),
                        line_state_time.eq(0),
                        self.phy_speed.eq(USBSpeed.FULL),
                    ]
                    self.detect_speed(m, USBAnalyzerSpeed.AUTO)
                    m.next = 'DISCONNECT'


                # If we're seeing a state other than IDLE, clear our suspend timer.
                with m.If(~bus_idle):
                    m.d.usb += line_state_time.eq(0)

                # If we see 3ms of consecutive line idle, we're being put into USB suspend.
                # We'll enter our suspended state, directly. [USB2.0: 7.1.7.6]
                #with m.If(line_state_time == self._CYCLES_3_MILLISECONDS):
                #    m.d.usb += was_hs_pre_suspend.eq(0)
                #    m.next = 'SUSPENDED'

            # FS_NON_RESET -- we're currently operating at FS and waiting for a reset;
            # the device could be active or inactive, but we haven't yet seen a reset condition.
            with m.State('FS_NON_RESET'):

                # If we're seeing a state other than SE0 (D+ / D- at zero), this isn't yet a
                # potential reset. Keep our timer at zero.
                with m.If(self.line_state != self._LINE_STATE_SE0):
                    m.d.usb += timer.eq(0)


                # If VBUS isn't connected, don't go through the whole reset process;
                # but also consider ourselves permanently in reset. This ensures we
                # don't progress through the reset FSM; but also ensures the device
                # state starts fresh with each plug.
                with m.If(~self.vbus_connected):
                    m.d.usb  += [
                        timer.eq(0),
                        line_state_time.eq(0),
                        self.phy_speed.eq(USBSpeed.FULL),
                    ]
                    self.detect_speed(m, USBAnalyzerSpeed.AUTO)
                    m.next = 'DISCONNECT'

                # If we see an SE0 for >2.5uS; < 3ms, this a bus reset.
                # We'll trigger a reset after 5uS; providing a little bit of timing flexibility.
                # [USB2.0: 7.1.7.5; ULPI 3.8.5.1].
                with m.If(timer == self._CYCLES_5_MICROSECONDS):
                    m.d.comb += self.bus_reset.eq(1)
                    m.next = 'START_HS_DETECTION'


                # If we're seeing a state other than IDLE, clear our suspend timer.
                with m.If(~bus_idle):
                    m.d.usb += line_state_time.eq(0)

                # If we see 3ms of consecutive line idle, we're being put into USB suspend.
                # We'll enter our suspended state, directly. [USB2.0: 7.1.7.6]
                with m.If(line_state_time == self._CYCLES_3_MILLISECONDS):
                    m.d.usb += was_hs_pre_suspend.eq(0)
                    m.next = 'SUSPENDED'




            # HS_NON_RESET -- we're currently operating at high speed and waiting for a reset or
            # suspend; the device could be active or inactive.
            with m.State('HS_NON_RESET'):

                # If we're seeing a state other than SE0 (D+ / D- at zero), this isn't yet a
                # potential reset. Keep our timer at zero.
                with m.If(self.line_state != self._LINE_STATE_SE0):
                    m.d.usb += timer.eq(0)

                # If VBUS isn't connected, our device/host relationship is effectively
                # a blank state. We'll want to present our detection pull-up to the host,
                # so we'll drop out of high speed.
                with m.If(~self.vbus_connected):
                    m.d.comb += self.bus_reset.eq(1)
                    m.next = 'IS_FULL_SPEED'


                # High speed signaling presents IDLE and RESET the same way: with the host
                # driving SE0; and us seeing SQUELCH. [USB2.0: 7.1.1.3; USB2.0: 7.1.7.6].
                # Either way, our next step is the same: we'll drop down to full-speed. [USB2.0: 7.1.7.6]
                # Afterwards, we'll take steps to differentiate a reset from a suspend.
                with m.If(timer == self._CYCLES_3_MILLISECONDS):
                    m.d.usb += [
                        timer.eq(0),
                        self.phy_speed.eq(USBSpeed.FULL),
                    ]
                    m.next = 'DETECT_HS_SUSPEND'


            # START_HS_DETECTION -- entry state for high-speed detection
            with m.State('START_HS_DETECTION'):
                m.d.usb += [
                    timer              .eq(0),
                    line_state_time    .eq(0),
                    chirp_seen         .eq(0),
                    # Switch into High-speed mode
                    self.phy_speed.eq(USBSpeed.HIGH),
                ]
                m.next = 'AWAIT_DEVICE_CHIRP'


            # AWAIT_DEVICE_CHIRP -- the device produces a 'chirp' K,
            # which advertises to the host that it's high speed capable.
            with m.State('AWAIT_DEVICE_CHIRP'):

                # In HS mode, the linestate signals the assertion and de-assertion of squelch.
                with m.If(~chirp_k):
                    # If HS squelch asserted, we're still seeing SE0 so reset the timer.
                    m.d.usb += line_state_time.eq(0)

                    # If we've already seen the device chirp, then the return to SE0 signals
                    # the end of the chirp and we should await the host chirp.
                    with m.If(chirp_seen):
                        m.d.usb += [
                            timer           .eq(0),
                            line_state_time .eq(0),
                            chirp_seen      .eq(0),
                        ]
                        m.next = 'AWAIT_HOST_K'

                # The host must detect the device chirp after it has seen assertion
                # of the Chirp K for no less than 2.5us [USB2.0: 7.1.7.5]
                with m.If(line_state_time == self._CYCLES_2P5_MICROSECONDS):
                    m.d.usb += chirp_seen.eq(1)

                with m.If(timer == self._CYCLES_7_MILLISECONDS):
                    m.next = 'IS_FULL_SPEED'



            # AWAIT_HOST_K -- we've now completed the device chirp; and are waiting to see if the host
            # will respond with an alternating sequence of K's and J's.
            with m.State('AWAIT_HOST_K'):

                # If we don't see our response within 2.5ms, this isn't a compliant HS host. [USB2.0: 7.1.7.5].
                # This is thus a full-speed host, and we'll act as a full-speed device.
                with m.If(timer == self._CYCLES_2P5_MILLISECONDS):
                    m.next = 'IS_FULL_SPEED'

                # Once we've seen our K, we're good to start observing J/K toggles.
                with m.If(chirp_k):
                    m.next = 'IN_HOST_K'
                    m.d.usb += line_state_time.eq(0)


            # IN_HOST_K: we're seeing a host Chirp K as part of our handshake; we'll
            # time it and see how long it lasts
            with m.State('IN_HOST_K'):

                # If we've exceeded our minimum chirp time, consider this a valid pattern
                # bit, # and advance in the pattern.
                with m.If(line_state_time == self._CYCLES_2P5_MICROSECONDS):
                    m.next = 'AWAIT_HOST_J'

                # If our input has become something other than a K, then
                # we haven't finished our sequence. We'll go back to expecting a K.
                with m.If(~chirp_k):
                    m.next = 'AWAIT_HOST_K'

                # Time out if we exceed our maximum allowed duration.
                with m.If(timer == self._CYCLES_2P5_MILLISECONDS):
                    m.next = 'IS_FULL_SPEED'


            # AWAIT_HOST_J -- we're waiting for the next Chirp J in the host chirp sequence
            with m.State('AWAIT_HOST_J'):

                # If we've exceeded our maximum wait, this isn't a high speed host.
                with m.If(timer == self._CYCLES_2P5_MILLISECONDS):
                    m.next = 'IS_FULL_SPEED'

                # Once we've seen our J, start timing its duration.
                with m.If(chirp_j):
                    m.next = 'IN_HOST_J'
                    m.d.usb += line_state_time.eq(0)


            # IN_HOST_J: we're seeing a host Chirp K as part of our handshake; we'll
            # time it and see how long it lasts
            with m.State('IN_HOST_J'):

                # If we've exceeded our minimum chirp time, consider this a valid pattern
                # bit, and advance in the pattern.
                with m.If(line_state_time == self._CYCLES_2P5_MICROSECONDS):

                    # If this would complete our third pair, this completes a handshake,
                    # and we've identified a high speed host!
                    with m.If(valid_pairs == 2):
                        m.next = 'IS_HIGH_SPEED'

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
                    m.next = 'IS_FULL_SPEED'


            # IS_HIGH_SPEED -- we've completed a high speed handshake, and are ready to
            # switch to high speed signaling
            with m.State('IS_HIGH_SPEED'):

                # Switch to high speed.
                m.d.usb += [
                    timer                    .eq(0),
                    line_state_time          .eq(0),
                    self.phy_speed           .eq(USBSpeed.HIGH),
                ]

                # Signal HS detection.
                self.detect_speed(m, USBAnalyzerSpeed.HIGH)

                m.next = 'HS_NON_RESET'


            # IS_LOW_OR_FULL_SPEED -- we've decided the device is low/full speed (typically
            # because it didn't) complete our high-speed handshake; set it up accordingly.
            with m.State('IS_FULL_SPEED'):
                m.d.usb += self.phy_speed.eq(USBSpeed.FULL)

                # Signal FS detection.
                self.detect_speed(m, USBAnalyzerSpeed.FULL)

                # Once we know that our reset is complete, move back to our normal, non-reset state.
                with m.If(self.line_state != self._LINE_STATE_SE0):
                    m.next = 'FS_NON_RESET'
                    m.d.usb += [
                        timer.eq(0),
                        line_state_time.eq(0)
                    ]


            # DETECT_HS_SUSPEND -- we were operating at high speed, and just detected an event
            # which is either a reset or a suspend event; we'll now detect which.
            with m.State('DETECT_HS_SUSPEND'):

                # We've just switch from HS signaling to FS signaling.
                # We'll wait a little while for the bus to settle, and then
                # check to see if it's settled to FS idle; or if we still see SE0.
                with m.If(timer == self._CYCLES_200_MICROSECONDS):
                    m.d.usb += timer.eq(0)

                    # If we've resume IDLE, this is suspend. Move to HS suspend.
                    with m.If(self.line_state == self._LINE_STATE_FS_HS_J):
                        m.d.usb += was_hs_pre_suspend.eq(1)
                        m.next = 'SUSPENDED'

                    # Otherwise, this is a reset (or, if K/SE1, we're very confused, and
                    # should re-initialize anyway). Move to the HS reset detect sequence.
                    with m.Else():
                        m.d.comb += self.bus_reset.eq(1)
                        m.next = 'START_HS_DETECTION'


            # SUSPEND -- our device has entered USB suspend; we'll now wait for either a
            # resume or a reset
            with m.State('SUSPENDED'):
                m.d.comb += self.suspended.eq(1)

                # If we see a K state, then we're being resumed.
                is_fs_k = (self.line_state == self._LINE_STATE_FS_HS_K)
                with m.If(is_fs_k):
                    m.d.usb  += timer.eq(0)

                    # If we were in high-speed pre-suspend, then resume being in HS.
                    with m.If(was_hs_pre_suspend):
                        m.next = 'IS_HIGH_SPEED'

                    # Otherwise, just resume.
                    with m.Else():
                        m.next = 'FS_NON_RESET'
                        m.d.usb += [
                            timer.eq(0),
                            line_state_time.eq(0)
                        ]


                # If this isn't an SE0, we're not receiving a reset request.
                # Keep our reset counter at zero.
                with m.If(self.line_state != self._LINE_STATE_SE0):
                    m.d.usb += timer.eq(0)


                # If we see an SE0 for > 2.5uS, this is a reset request. [USB 2.0: 7.1.7.5]
                # We'll handle it directly from suspend.
                with m.If(timer == self._CYCLES_2P5_MICROSECONDS):
                    m.d.comb += self.bus_reset.eq(1)
                    m.d.usb  += timer.eq(0)

                    # Otherwise, this could be a high-speed device; enter its reset.
                    m.next = 'START_HS_DETECTION'

        return m

    def detect_speed(self, m, new_speed):
        with m.If(new_speed != self.detected_speed):
            m.d.comb += [
                self.speed_changing.eq(1),
                self.next_speed.eq(new_speed),
            ]
            m.d.usb += self.detected_speed.eq(new_speed)
