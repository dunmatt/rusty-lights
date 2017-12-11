#![feature(proc_macro)] // <- IMPORTANT! Feature gate for procedural macros
#![no_std]

// #[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f40x;

use cortex_m::peripheral::SystClkSource;
use rtfm::{app, Threshold};

app! {
  device: stm32f40x,
  tasks: {
    SYS_TICK: {
      path: toggle,
      resources: [GPIOC],
    },
  },
}

fn init(p: init::Peripherals) {
  // TODO (robomancer): try taking the below line out to see if it's required
  p.RCC.ahb1enr.write(|w| w.gpiocen().enabled());
  p.GPIOC.moder.write(|w| w.moder13().output());
  p.GPIOC.otyper.write(|w| w.ot13().clear_bit());

  p.SYST.set_clock_source(SystClkSource::Core);
  p.SYST.set_reload(16_000_000);  // 2s
  p.SYST.enable_interrupt();
  p.SYST.enable_counter();
}

fn idle() -> ! {
  loop {
    rtfm::wfi();
  }
}

fn toggle(_t: &mut Threshold, r: SYS_TICK::Resources) {
  r.GPIOC.odr.modify(|r, w| w.odr13().bit(!r.odr13().bit()));
}
