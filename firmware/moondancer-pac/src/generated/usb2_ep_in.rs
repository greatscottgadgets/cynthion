#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    endpoint: ENDPOINT,
    stall: STALL,
    pid: PID,
    _reserved3: [u8; 0x01],
    status: STATUS,
    reset: RESET,
    data: DATA,
    _reserved6: [u8; 0x06],
    ev_enable: EV_ENABLE,
    ev_pending: EV_PENDING,
}
impl RegisterBlock {
    #[doc = "0x00 - Endpoint register number: Contains the endpoint the enqueued packet is to be transmitted on. Writing to this field marks the relevant packet as ready to transmit; and thus should only be written after a full packet has been written into the FIFO. If no data has been placed into the DATA FIFO, a zero-length packet is generated. Note that any IN requests that do not match the endpoint number are automatically NAK'd."]
    #[inline(always)]
    pub const fn endpoint(&self) -> &ENDPOINT {
        &self.endpoint
    }
    #[doc = "0x01 - Stall register stalled: When this field contains '1', any IN tokens targeting `epno` will be responded to with a STALL token, rather than DATA or a NAK. For EP0, this field will automatically be cleared when a new SETUP token is received."]
    #[inline(always)]
    pub const fn stall(&self) -> &STALL {
        &self.stall
    }
    #[doc = "0x02 - Pid register toggle: Sets the current PID toggle bit for the given endpoint."]
    #[inline(always)]
    pub const fn pid(&self) -> &PID {
        &self.pid
    }
    #[doc = "0x04 - Status register nak: Contains a bitmask of endpoints that have responded with a NAK since the last read of this register. epno: Contains the endpoint being transmitted on. idle: This value is `1` if no packet is actively being transmitted. have: This value is `1` if data is present in the transmit FIFO. pid: Contains the current PID toggle bit for the given endpoint."]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x08 - Reset register fifo: A write to this field Clears the FIFO without transmitting."]
    #[inline(always)]
    pub const fn reset(&self) -> &RESET {
        &self.reset
    }
    #[doc = "0x09 - Data register Each write enqueues a byte to be transmitted; gradually building a single packet to be transmitted. This queue should only ever contain a single packet; it is the software's responsibility to handle breaking requests down into packets."]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x10 - A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations <python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable."]
    #[inline(always)]
    pub const fn ev_enable(&self) -> &EV_ENABLE {
        &self.ev_enable
    }
    #[doc = "0x11 - A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations <python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable."]
    #[inline(always)]
    pub const fn ev_pending(&self) -> &EV_PENDING {
        &self.ev_pending
    }
}
#[doc = "endpoint (rw) register accessor: Endpoint register number: Contains the endpoint the enqueued packet is to be transmitted on. Writing to this field marks the relevant packet as ready to transmit; and thus should only be written after a full packet has been written into the FIFO. If no data has been placed into the DATA FIFO, a zero-length packet is generated. Note that any IN requests that do not match the endpoint number are automatically NAK'd.\n\nYou can [`read`](crate::Reg::read) this register and get [`endpoint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endpoint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endpoint`]
module"]
#[doc(alias = "endpoint")]
pub type ENDPOINT = crate::Reg<endpoint::ENDPOINT_SPEC>;
#[doc = "Endpoint register number: Contains the endpoint the enqueued packet is to be transmitted on. Writing to this field marks the relevant packet as ready to transmit; and thus should only be written after a full packet has been written into the FIFO. If no data has been placed into the DATA FIFO, a zero-length packet is generated. Note that any IN requests that do not match the endpoint number are automatically NAK'd."]
pub mod endpoint;
#[doc = "stall (rw) register accessor: Stall register stalled: When this field contains '1', any IN tokens targeting `epno` will be responded to with a STALL token, rather than DATA or a NAK. For EP0, this field will automatically be cleared when a new SETUP token is received.\n\nYou can [`read`](crate::Reg::read) this register and get [`stall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stall`]
module"]
#[doc(alias = "stall")]
pub type STALL = crate::Reg<stall::STALL_SPEC>;
#[doc = "Stall register stalled: When this field contains '1', any IN tokens targeting `epno` will be responded to with a STALL token, rather than DATA or a NAK. For EP0, this field will automatically be cleared when a new SETUP token is received."]
pub mod stall;
#[doc = "pid (rw) register accessor: Pid register toggle: Sets the current PID toggle bit for the given endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`pid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pid`]
module"]
#[doc(alias = "pid")]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "Pid register toggle: Sets the current PID toggle bit for the given endpoint."]
pub mod pid;
#[doc = "status (rw) register accessor: Status register nak: Contains a bitmask of endpoints that have responded with a NAK since the last read of this register. epno: Contains the endpoint being transmitted on. idle: This value is `1` if no packet is actively being transmitted. have: This value is `1` if data is present in the transmit FIFO. pid: Contains the current PID toggle bit for the given endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register nak: Contains a bitmask of endpoints that have responded with a NAK since the last read of this register. epno: Contains the endpoint being transmitted on. idle: This value is `1` if no packet is actively being transmitted. have: This value is `1` if data is present in the transmit FIFO. pid: Contains the current PID toggle bit for the given endpoint."]
pub mod status;
#[doc = "reset (rw) register accessor: Reset register fifo: A write to this field Clears the FIFO without transmitting.\n\nYou can [`read`](crate::Reg::read) this register and get [`reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset`]
module"]
#[doc(alias = "reset")]
pub type RESET = crate::Reg<reset::RESET_SPEC>;
#[doc = "Reset register fifo: A write to this field Clears the FIFO without transmitting."]
pub mod reset;
#[doc = "data (rw) register accessor: Data register Each write enqueues a byte to be transmitted; gradually building a single packet to be transmitted. This queue should only ever contain a single packet; it is the software's responsibility to handle breaking requests down into packets.\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "data")]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data register Each write enqueues a byte to be transmitted; gradually building a single packet to be transmitted. This queue should only ever contain a single packet; it is the software's responsibility to handle breaking requests down into packets."]
pub mod data;
#[doc = "ev_enable (rw) register accessor: A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations <python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable.\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_enable`]
module"]
#[doc(alias = "ev_enable")]
pub type EV_ENABLE = crate::Reg<ev_enable::EV_ENABLE_SPEC>;
#[doc = "A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations <python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable."]
pub mod ev_enable;
#[doc = "ev_pending (rw) register accessor: A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations <python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable.\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_pending::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_pending::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_pending`]
module"]
#[doc(alias = "ev_pending")]
pub type EV_PENDING = crate::Reg<ev_pending::EV_PENDING_SPEC>;
#[doc = "A CSR register. Parameters ---------- fields : :class:`dict` or :class:`list` or :class:`Field` Collection of register fields. If ``None`` (default), a dict is populated from Python :term:`variable annotations <python:variable annotations>`. ``fields`` is used to create a :class:`FieldActionMap`, :class:`FieldActionArray`, or :class:`FieldAction`, depending on its type (dict, list, or Field). Interface attributes -------------------- element : :class:`Element` Interface between this register and a CSR bus primitive. Attributes ---------- field : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Collection of field instances. f : :class:`FieldActionMap` or :class:`FieldActionArray` or :class:`FieldAction` Shorthand for :attr:`Register.field`. Raises ------ :exc:`TypeError` If ``fields`` is neither ``None``, a :class:`dict`, a :class:`list`, or a :class:`Field`. :exc:`ValueError` If ``fields`` is not ``None`` and at least one variable annotation is a :class:`Field`. :exc:`ValueError` If ``element.access`` is not readable and at least one field is readable. :exc:`ValueError` If ``element.access`` is not writable and at least one field is writable."]
pub mod ev_pending;
