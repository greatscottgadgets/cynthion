pub mod interrupt {
    //! CSR access methods.

    use crate::register;
    use crate::Interrupt;

    pub unsafe fn enable(interrupt: Interrupt) {
        let mask = register::mim::read();
        let mask = mask | (1 << interrupt as usize);
        register::mim::write(mask);
        while register::mim::read() != mask {}
    }

    pub unsafe fn disable(interrupt: Interrupt) {
        let mask = register::mim::read();
        let mask = mask & !(1 << interrupt as usize);
        register::mim::write(mask);
        while register::mim::read() != mask {}
    }

    pub fn reg_mask() -> usize {
        register::mim::read()
    }

    // TODO decide on params - basically what we need to know is
    // whether minerva gets ratty inbetween calls to read the csr
    // registers
    //pub fn is_pending(pending: usize, interrupt: Interrupt) -> bool {
    //    (pending & (1 << interrupt as usize)) != 0
    //}
    pub fn pending(interrupt: Interrupt) -> bool {
        let pending = register::mip::read();
        (pending & (1 << interrupt as usize)) != 0
    }

    pub fn reg_pending() -> usize {
        register::mip::read()
    }
}
