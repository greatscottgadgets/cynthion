target extended-remote :3333

set print asm-demangle on
set backtrace limit 8

break DefaultHandler
break HardFault
break rust_begin_unwind

info mem
load
continue
