#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: CONTROL,
    _reserved1: [u8; 0x01],
    status: STATUS,
    reset: RESET,
    data: DATA,
    _reserved4: [u8; 0x0a],
    ev_enable: EV_ENABLE,
    ev_pending: EV_PENDING,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register address: Controls the current device's USB address. Should be written after a SET_ADDRESS request is received. Automatically resets back to zero on a USB reset."]
    #[inline(always)]
    pub const fn control(&self) -> &CONTROL {
        &self.control
    }
    #[doc = "0x02 - Status register address: Holds the current device's active USB address. epno: The endpoint number associated with the most recently captured SETUP packet. have: `1` iff data is available in the FIFO."]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x04 - Reset register fifo: Local reset control for the SETUP handler; writing a '1' to this register clears the handler state."]
    #[inline(always)]
    pub const fn reset(&self) -> &RESET {
        &self.reset
    }
    #[doc = "0x05 - Data register A FIFO that returns the bytes from the most recently captured SETUP packet. Reading a byte from this register advances the FIFO. The first eight bytes read from this contain the core SETUP packet."]
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
#[doc = "control (rw) register accessor: Control register address: Controls the current device's USB address. Should be written after a SET_ADDRESS request is received. Automatically resets back to zero on a USB reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "control")]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Control register address: Controls the current device's USB address. Should be written after a SET_ADDRESS request is received. Automatically resets back to zero on a USB reset."]
pub mod control;
#[doc = "status (rw) register accessor: Status register address: Holds the current device's active USB address. epno: The endpoint number associated with the most recently captured SETUP packet. have: `1` iff data is available in the FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register address: Holds the current device's active USB address. epno: The endpoint number associated with the most recently captured SETUP packet. have: `1` iff data is available in the FIFO."]
pub mod status;
#[doc = "reset (rw) register accessor: Reset register fifo: Local reset control for the SETUP handler; writing a '1' to this register clears the handler state.\n\nYou can [`read`](crate::Reg::read) this register and get [`reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset`]
module"]
#[doc(alias = "reset")]
pub type RESET = crate::Reg<reset::RESET_SPEC>;
#[doc = "Reset register fifo: Local reset control for the SETUP handler; writing a '1' to this register clears the handler state."]
pub mod reset;
#[doc = "data (rw) register accessor: Data register A FIFO that returns the bytes from the most recently captured SETUP packet. Reading a byte from this register advances the FIFO. The first eight bytes read from this contain the core SETUP packet.\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "data")]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data register A FIFO that returns the bytes from the most recently captured SETUP packet. Reading a byte from this register advances the FIFO. The first eight bytes read from this contain the core SETUP packet."]
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
