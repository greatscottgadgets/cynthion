#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: CONTROL,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register enable : Set this bit to '1' to start ApolloAdvertiser and disconnect the Cynthion USB control port from Apollo."]
    #[inline(always)]
    pub const fn control(&self) -> &CONTROL {
        &self.control
    }
}
#[doc = "control (rw) register accessor: Control register enable : Set this bit to '1' to start ApolloAdvertiser and disconnect the Cynthion USB control port from Apollo.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "control")]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Control register enable : Set this bit to '1' to start ApolloAdvertiser and disconnect the Cynthion USB control port from Apollo."]
pub mod control;
