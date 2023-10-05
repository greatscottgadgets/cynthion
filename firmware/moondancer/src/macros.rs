#[macro_export]
macro_rules! profile {
    ($($token:tt)+) => {
        {
            let t1 = riscv::register::mcycle::read();
            let _result = {
                $($token)+
            };
            let t2 = riscv::register::mcycle::read();
            (_result, t2 - t1)
        }
    }
}
