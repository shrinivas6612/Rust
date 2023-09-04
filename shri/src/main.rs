#![no_std]
#![no_main]
#[allow(warnings)]
#[allow(unused_imports)]


// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a 

extern crate stm32f4;
extern crate panic_halt;
extern crate cortex_m_rt;
extern crate cortex_m;


use cortex_m_rt::entry;
use stm32f4::stm32f407;

#[entry]
fn main() -> ! {
    // get handles to the hardware
    let peripherals = stm32f407::Peripherals::take().unwrap();
    let gpiod= &peripherals.GPIOD;
    let rcc = &peripherals.RCC;

    // enable the GPIO clock for IO port C
    rcc.ahb1enr.write(|w| w.gpioden().set_bit());
    gpiod.moder.write(|w| 
        w.moder15().bits(0b01)
    );
 
    loop{

        // gpiod.bsrr.write(|w| w.bs14().set_bit());
        // cortex_m::asm::delay(2000);
        // gpiod.bsrr.write(|w| w.br14().set_bit());
        // cortex_m::asm::delay(2000);
        gpiod.odr.write(|w| {
            w.odr15().bit(gpiod.odr.read().odr15().bit_is_clear())
        });
        for _i in 0..1000 {
            cortex_m::asm::nop()
        }
    }
}
