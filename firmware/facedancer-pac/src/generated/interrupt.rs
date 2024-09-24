#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - timer"]
    TIMER = 0,
    #[doc = "1 - uart"]
    UART = 1,
    #[doc = "2 - gpioa"]
    GPIOA = 2,
    #[doc = "3 - gpiob"]
    GPIOB = 3,
    #[doc = "4 - usb0"]
    USB0 = 4,
    #[doc = "5 - usb0_ep_control"]
    USB0_EP_CONTROL = 5,
    #[doc = "6 - usb0_ep_in"]
    USB0_EP_IN = 6,
    #[doc = "7 - usb0_ep_out"]
    USB0_EP_OUT = 7,
    #[doc = "8 - usb1"]
    USB1 = 8,
    #[doc = "9 - usb1_ep_control"]
    USB1_EP_CONTROL = 9,
    #[doc = "10 - usb1_ep_in"]
    USB1_EP_IN = 10,
    #[doc = "11 - usb1_ep_out"]
    USB1_EP_OUT = 11,
    #[doc = "12 - usb2"]
    USB2 = 12,
    #[doc = "13 - usb2_ep_control"]
    USB2_EP_CONTROL = 13,
    #[doc = "14 - usb2_ep_in"]
    USB2_EP_IN = 14,
    #[doc = "15 - usb2_ep_out"]
    USB2_EP_OUT = 15,
    #[doc = "16 - uart1"]
    UART1 = 16,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            0 => Ok(Interrupt::TIMER),
            1 => Ok(Interrupt::UART),
            2 => Ok(Interrupt::GPIOA),
            3 => Ok(Interrupt::GPIOB),
            4 => Ok(Interrupt::USB0),
            5 => Ok(Interrupt::USB0_EP_CONTROL),
            6 => Ok(Interrupt::USB0_EP_IN),
            7 => Ok(Interrupt::USB0_EP_OUT),
            8 => Ok(Interrupt::USB1),
            9 => Ok(Interrupt::USB1_EP_CONTROL),
            10 => Ok(Interrupt::USB1_EP_IN),
            11 => Ok(Interrupt::USB1_EP_OUT),
            12 => Ok(Interrupt::USB2),
            13 => Ok(Interrupt::USB2_EP_CONTROL),
            14 => Ok(Interrupt::USB2_EP_IN),
            15 => Ok(Interrupt::USB2_EP_OUT),
            16 => Ok(Interrupt::UART1),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ($ NAME : ident , $ path : path , locals : { $ ($ lvar : ident : $ lty : ty = $ lval : expr ;) * }) => { # [allow (non_snake_case)] mod $ NAME { pub struct Locals { $ (pub $ lvar : $ lty ,) * } } # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ($ lvar : $ lval ,) * } ; let f : fn (& mut self :: $ NAME :: Locals) = $ path ; f (unsafe { & mut LOCALS }) ; } } ; ($ NAME : ident , $ path : path) => { # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn () = $ path ; f () ; } } }
