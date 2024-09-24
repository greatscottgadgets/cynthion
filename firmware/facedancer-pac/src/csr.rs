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

    /// Return the current value of the CPU's Machines IRQ Mask register.
    #[must_use]
    pub fn reg_mask() -> usize {
        register::mim::read()
    }

    /// Return the current bit value of the CPU's Machines IRQ Pending register.
    #[must_use]
    pub fn bits_pending() -> usize {
        register::mip::read()
    }

    /// Check if the given `Interrupt` is pending in the CPU's Machines IRQ Pending register.
    #[must_use]
    pub fn is_pending(interrupt: Interrupt) -> bool {
        let pending = register::mip::read();
        (pending & (1 << interrupt as usize)) != 0
    }

    /// Returns the current `Interrupt` pending in the CPU's Machines IRQ Pending register.
    ///
    /// If there is no interrupt pending or an unknown interrupt
    /// pending it returns an `Err` containing the current bit value
    /// of the register.
    pub fn pending() -> Result<Interrupt, usize> {
        let bit = register::mip::read();
        if bit == 0 {
            return Err(0);
        }
        let pending = bit.ilog2();
        if let Ok(interrupt) = Interrupt::try_from(pending as u8) {
            Ok(interrupt)
        } else {
            Err(bit)
        }
    }
}
