//! Support for various vendor defined softcore extensions.

pub mod minerva {

    pub mod register {
        //! Micro-architecture specific CSR extensions for the Minerva RISC-V
        //! soft processor.
        //!
        //! See: [ISA definition](https://github.com/minerva-cpu/minerva/blob/master/minerva/isa.py)
        //!
        //! These are somewhat weird because peripheral irq enable (0x330)
        //! overlaps with the Machine Counter Setup `mhpmevent16`
        //! performance-monitoring event selector.
        //!
        //! See: [Chapter 2 - Control and Status Registers](https://riscv.org/wp-content/uploads/2017/05/riscv-privileged-v1.10.pdf)

        /// Machine IRQ Mask
        pub mod mim {
            crate::macros::read_csr_as_usize!(0x330);
            crate::macros::write_csr_as_usize!(0x330);
        }

        /// Machine IRQ Pending
        pub mod mip {
            crate::macros::read_csr_as_usize!(0x360);
        }
    }
}

pub mod vexriscv {

    #[inline(always)]
    pub fn flush_icache() {
        unsafe {
            core::arch::asm!(".word(0x100f)", "nop", "nop", "nop", "nop", "nop",);
        }
    }
    #[inline(always)]
    pub fn flush_dcache() {
        unsafe {
            core::arch::asm!(".word(0x500f)");
        }
    }

    pub mod register {
        //! Micro-architecture specific CSR extensions for the `VexRiscv` RISC-V
        //! soft processor.
        //!
        //! See: [ExternalInterruptArrayPlugin.scala](https://github.com/SpinalHDL/VexRiscv/blob/master/src/main/scala/vexriscv/plugin/ExternalInterruptArrayPlugin.scala)

        /// Machine IRQ Mask
        pub mod mim {
            crate::macros::read_csr_as_usize!(0xBC0);
            crate::macros::write_csr_as_usize!(0xBC0);
        }

        /// Machine IRQ Pending
        pub mod mip {
            crate::macros::read_csr_as_usize!(0xFC0);
        }

        /// Supervisor IRQ Mask
        pub mod sim {
            crate::macros::read_csr_as_usize!(0x9C0);
            crate::macros::write_csr_as_usize!(0x9C0);
        }

        /// Supervisor IRQ Pending
        pub mod sip {
            crate::macros::read_csr_as_usize!(0xDC0);
        }

        /// Data Cache Info
        pub mod dci {
            crate::macros::read_csr_as_usize!(0xCC0);
        }
    }
}
