#![no_main]
#![no_std]

use panic_rtt_target as _;

#[rtic::app(device = stm32f3xx_hal::pac, dispatchers = [TIM20_BRK, TIM20_UP, TIM20_TRG_COM])]
mod app {
    use dwt_systick_monotonic::DwtSystick;
    use rtic::time::duration::*;
    use rtt_target::{rprintln, rtt_init_print};
    use stm32f3xx_hal::prelude::*;

    #[monotonic(binds = SysTick, default = true)]
    type DwtMono = DwtSystick<8_000_000>;

    #[init]
    fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {
        let mut flash = cx.device.FLASH.constrain();
        let rcc = cx.device.RCC.constrain();
        let mut dcb = cx.core.DCB;
        let dwt = cx.core.DWT;
        let systick = cx.core.SYST;

        rtt_init_print!(NoBlockSkip, 4096);

        rprintln!("pre init");

        //
        // Initialize the clocks
        //
        let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);

        // Setup the monotonic timer
        let mono2 = DwtSystick::new(&mut dcb, dwt, systick, clocks.sysclk().0);

        rprintln!("init");

        printer::spawn(1).unwrap();
        printer::spawn_after(Milliseconds(5_000_u32), 6).unwrap();
        printer::spawn_after(Milliseconds(6_000_u32), 7).unwrap();
        printer::spawn_after(Milliseconds(7_000_u32), 8).unwrap();
        printer::spawn_after(Milliseconds(8_000_u32), 9).unwrap();
        printer::spawn_after(Milliseconds(4_000_u32), 5).unwrap();
        printer::spawn_after(Milliseconds(3_000_u32), 4).unwrap();
        printer::spawn_after(Milliseconds(2_000_u32), 3).unwrap();
        printer::spawn_after(Milliseconds(1_000_u32), 2).unwrap();

        let _now = mono2.try_now();

        // (init::LateResources {}, init::Monotonics(mono2))
        (init::LateResources {}, init::Monotonics(mono2))
    }

    use core::convert::TryInto;

    pub type TEST = u32;

    #[task(capacity = 16)]
    fn printer(_cx: printer::Context, val: TEST) {
        let now: Milliseconds = monotonics::DwtMono::now().duration_since_epoch().try_into().unwrap();
        rprintln!("Val: {} at {} ms", val, now.integer());
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        rprintln!("idle");

        loop {
            cortex_m::asm::nop();
        }
    }
}
