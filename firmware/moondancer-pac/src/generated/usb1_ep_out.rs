#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: CONTROL,
    endpoint: ENDPOINT,
    enable: ENABLE,
    prime: PRIME,
    stall: STALL,
    pid: PID,
    status: STATUS,
    reset: RESET,
    data: DATA,
    _reserved9: [u8; 0x16],
    ev_enable: EV_ENABLE,
    ev_pending: EV_PENDING,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register address: Controls the current device's USB address. Should be written after a SET_ADDRESS request is received. Automatically resets back to zero on a USB reset."]
    #[inline(always)]
    pub const fn control(&self) -> &CONTROL {
        &self.control
    }
    #[doc = "0x01 - Endpoint register number: Selects the endpoint number to prime. This interface allows priming multiple endpoints at once. That is, multiple endpoints can be ready to receive data at a time. See the `prime` and `enable` bits for usage."]
    #[inline(always)]
    pub const fn endpoint(&self) -> &ENDPOINT {
        &self.endpoint
    }
    #[doc = "0x02 - Enable register enabled: Controls whether any data can be received on any primed OUT endpoint. This bit is automatically cleared on receive in order to give the controller time to read data from the FIFO. It must be re-enabled once the FIFO has been emptied."]
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    #[doc = "0x03 - Prime register primed: Controls \"priming\" an out endpoint. To receive data on any endpoint, the CPU must first select the endpoint with the `epno` register; and then write a '1' into the prime and enable register. This prepares our FIFO to receive data; and the next OUT transaction will be captured into the FIFO. When a transaction is complete, the `enable` bit is reset; the `prime` is not. This effectively means that `enable` controls receiving on _any_ of the primed endpoints; while `prime` can be used to build a collection of endpoints willing to participate in receipt. Note that this does not apply to the control endpoint. Once the control endpoint has received a packet it will be un-primed and need to be re-primed before it can receive again. This is to ensure that we can establish an order on the receipt of the setup packet and any associated data. Only one transaction / data packet is captured per `enable` write; repeated enabling is necessary to capture multiple packets."]
    #[inline(always)]
    pub const fn prime(&self) -> &PRIME {
        &self.prime
    }
    #[doc = "0x04 - Stall register stalled: Controls STALL'ing the active endpoint. Setting or clearing this bit will set or clear STALL on the provided endpoint. Endpoint STALLs persist even after `epno` is changed; so multiple endpoints can be stalled at once by writing their respective endpoint numbers into `epno` register and then setting their `stall` bits."]
    #[inline(always)]
    pub const fn stall(&self) -> &STALL {
        &self.stall
    }
    #[doc = "0x05 - Pid register toggle: Sets the current PID toggle bit for the given endpoint."]
    #[inline(always)]
    pub const fn pid(&self) -> &PID {
        &self.pid
    }
    #[doc = "0x06 - Status register epno: Contains the endpoint number associated with the data in the FIFO -- that is, the endpoint number on which the relevant data was received. have: `1` iff data is available in the FIFO. pid: Contains the current PID toggle bit for the given endpoint."]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08 - Reset register fifo: Local reset for the OUT handler; clears the out FIFO."]
    #[inline(always)]
    pub const fn reset(&self) -> &RESET {
        &self.reset
    }
    #[doc = "0x09 - Data register Read-only register. A FIFO that returns the bytes from the most recently captured OUT transaction. Reading a byte from this register advances the FIFO. byte: Contains the most recently received byte."]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x20 - A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations &lt;python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable."]
    #[inline(always)]
    pub const fn ev_enable(&self) -> &EV_ENABLE {
        &self.ev_enable
    }
    #[doc = "0x21 - A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations &lt;python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable."]
    #[inline(always)]
    pub const fn ev_pending(&self) -> &EV_PENDING {
        &self.ev_pending
    }
}
#[doc = "control (rw) register accessor: Control register address: Controls the current device's USB address. Should be written after a SET_ADDRESS request is received. Automatically resets back to zero on a USB reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "control")]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Control register address: Controls the current device's USB address. Should be written after a SET_ADDRESS request is received. Automatically resets back to zero on a USB reset."]
pub mod control;
#[doc = "endpoint (rw) register accessor: Endpoint register number: Selects the endpoint number to prime. This interface allows priming multiple endpoints at once. That is, multiple endpoints can be ready to receive data at a time. See the `prime` and `enable` bits for usage.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endpoint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endpoint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endpoint`]
module"]
#[doc(alias = "endpoint")]
pub type ENDPOINT = crate::Reg<endpoint::ENDPOINT_SPEC>;
#[doc = "Endpoint register number: Selects the endpoint number to prime. This interface allows priming multiple endpoints at once. That is, multiple endpoints can be ready to receive data at a time. See the `prime` and `enable` bits for usage."]
pub mod endpoint;
#[doc = "enable (rw) register accessor: Enable register enabled: Controls whether any data can be received on any primed OUT endpoint. This bit is automatically cleared on receive in order to give the controller time to read data from the FIFO. It must be re-enabled once the FIFO has been emptied.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "enable")]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable register enabled: Controls whether any data can be received on any primed OUT endpoint. This bit is automatically cleared on receive in order to give the controller time to read data from the FIFO. It must be re-enabled once the FIFO has been emptied."]
pub mod enable;
#[doc = "prime (rw) register accessor: Prime register primed: Controls \"priming\" an out endpoint. To receive data on any endpoint, the CPU must first select the endpoint with the `epno` register; and then write a '1' into the prime and enable register. This prepares our FIFO to receive data; and the next OUT transaction will be captured into the FIFO. When a transaction is complete, the `enable` bit is reset; the `prime` is not. This effectively means that `enable` controls receiving on _any_ of the primed endpoints; while `prime` can be used to build a collection of endpoints willing to participate in receipt. Note that this does not apply to the control endpoint. Once the control endpoint has received a packet it will be un-primed and need to be re-primed before it can receive again. This is to ensure that we can establish an order on the receipt of the setup packet and any associated data. Only one transaction / data packet is captured per `enable` write; repeated enabling is necessary to capture multiple packets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prime`]
module"]
#[doc(alias = "prime")]
pub type PRIME = crate::Reg<prime::PRIME_SPEC>;
#[doc = "Prime register primed: Controls \"priming\" an out endpoint. To receive data on any endpoint, the CPU must first select the endpoint with the `epno` register; and then write a '1' into the prime and enable register. This prepares our FIFO to receive data; and the next OUT transaction will be captured into the FIFO. When a transaction is complete, the `enable` bit is reset; the `prime` is not. This effectively means that `enable` controls receiving on _any_ of the primed endpoints; while `prime` can be used to build a collection of endpoints willing to participate in receipt. Note that this does not apply to the control endpoint. Once the control endpoint has received a packet it will be un-primed and need to be re-primed before it can receive again. This is to ensure that we can establish an order on the receipt of the setup packet and any associated data. Only one transaction / data packet is captured per `enable` write; repeated enabling is necessary to capture multiple packets."]
pub mod prime;
#[doc = "stall (rw) register accessor: Stall register stalled: Controls STALL'ing the active endpoint. Setting or clearing this bit will set or clear STALL on the provided endpoint. Endpoint STALLs persist even after `epno` is changed; so multiple endpoints can be stalled at once by writing their respective endpoint numbers into `epno` register and then setting their `stall` bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stall`]
module"]
#[doc(alias = "stall")]
pub type STALL = crate::Reg<stall::STALL_SPEC>;
#[doc = "Stall register stalled: Controls STALL'ing the active endpoint. Setting or clearing this bit will set or clear STALL on the provided endpoint. Endpoint STALLs persist even after `epno` is changed; so multiple endpoints can be stalled at once by writing their respective endpoint numbers into `epno` register and then setting their `stall` bits."]
pub mod stall;
#[doc = "pid (rw) register accessor: Pid register toggle: Sets the current PID toggle bit for the given endpoint.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid`]
module"]
#[doc(alias = "pid")]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "Pid register toggle: Sets the current PID toggle bit for the given endpoint."]
pub mod pid;
#[doc = "status (rw) register accessor: Status register epno: Contains the endpoint number associated with the data in the FIFO -- that is, the endpoint number on which the relevant data was received. have: `1` iff data is available in the FIFO. pid: Contains the current PID toggle bit for the given endpoint.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register epno: Contains the endpoint number associated with the data in the FIFO -- that is, the endpoint number on which the relevant data was received. have: `1` iff data is available in the FIFO. pid: Contains the current PID toggle bit for the given endpoint."]
pub mod status;
#[doc = "reset (rw) register accessor: Reset register fifo: Local reset for the OUT handler; clears the out FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset`]
module"]
#[doc(alias = "reset")]
pub type RESET = crate::Reg<reset::RESET_SPEC>;
#[doc = "Reset register fifo: Local reset for the OUT handler; clears the out FIFO."]
pub mod reset;
#[doc = "data (rw) register accessor: Data register Read-only register. A FIFO that returns the bytes from the most recently captured OUT transaction. Reading a byte from this register advances the FIFO. byte: Contains the most recently received byte.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "data")]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data register Read-only register. A FIFO that returns the bytes from the most recently captured OUT transaction. Reading a byte from this register advances the FIFO. byte: Contains the most recently received byte."]
pub mod data;
#[doc = "ev_enable (rw) register accessor: A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations &lt;python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_enable`]
module"]
#[doc(alias = "ev_enable")]
pub type EV_ENABLE = crate::Reg<ev_enable::EV_ENABLE_SPEC>;
#[doc = "A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations &lt;python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable."]
pub mod ev_enable;
#[doc = "ev_pending (rw) register accessor: A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations &lt;python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_pending::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_pending::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_pending`]
module"]
#[doc(alias = "ev_pending")]
pub type EV_PENDING = crate::Reg<ev_pending::EV_PENDING_SPEC>;
#[doc = "A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations &lt;python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable."]
pub mod ev_pending;
