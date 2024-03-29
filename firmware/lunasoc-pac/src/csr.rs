pub mod interrupt {
    //! CSR access methods.

    use crate::register;
    use crate::Interrupt;

    /// Unmask the given [`Interrupt`] in the CPU's Machines IRQ Mask register.
    ///
    /// # Safety
    ///
    /// Passing incorrect value can cause undefined behaviour. See CPU reference manual.
    pub unsafe fn enable(interrupt: Interrupt) {
        let mask = register::mim::read();
        let mask = mask | (1 << interrupt as usize);
        register::mim::write(mask);
        while register::mim::read() != mask {}
    }

    /// Mask the given [`Interrupt`] in the CPU's Machines IRQ Mask register.
    ///
    /// # Safety
    ///
    /// Passing incorrect value can cause undefined behaviour. See CPU reference manual.
    pub unsafe fn disable(interrupt: Interrupt) {
        let mask = register::mim::read();
        let mask = mask & !(1 << interrupt as usize);
        register::mim::write(mask);
        while register::mim::read() != mask {}
    }

    #[must_use]
    pub fn reg_mask() -> usize {
        register::mim::read()
    }

    #[must_use]
    pub fn pending(interrupt: Interrupt) -> bool {
        let pending = register::mip::read();
        (pending & (1 << interrupt as usize)) != 0
    }

    #[must_use]
    pub fn reg_pending() -> usize {
        register::mip::read()
    }
}
