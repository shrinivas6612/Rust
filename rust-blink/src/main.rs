
#![no_std]
#![no_main]
#[allow(warnings)]
#[allow(unused_imports)]


// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a 

extern crate stm32l5;
extern crate panic_halt;
extern crate cortex_m_rt;
extern crate cortex_m;


use cortex_m_rt::entry;
use stm32l5::stm32l562;

#[entry]
fn main() -> ! {
    // get handles to the hardware
    let peripherals = stm32l562::Peripherals::take().unwrap();
    let gpiog= &peripherals.GPIOG;
    let rcc = &peripherals.RCC;

    // enable the GPIO clock for IO port C
    rcc.ahb2enr.write(|w| w.gpiogen().set_bit());
    gpiog.moder.write(|w| 
        w.moder12().bits(0b01)
    );
 
    loop{

        // gpiog.bsrr.write(|w| w.bs12().set_bit());
        // cortex_m::asm::delay(2000);
        // gpiog.brr.write(|w| w.br12().set_bit());
        // cortex_m::asm::delay(2000);
        gpiog.odr.write(|w| {
            w.odr12().bit(gpiog.odr.read().odr12().bit_is_clear())
        });
        for _i in 0..1000 {
            cortex_m::asm::nop()
        }
    }
}
