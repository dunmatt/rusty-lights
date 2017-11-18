#![feature(lang_items)]
#![feature(proc_macro)] // <- IMPORTANT! Feature gate for procedural macros
#![feature(used)]
#![no_std]

// #[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_rtfm as rtfm;
extern crate stm32f40x;

use cortex_m::asm;
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


// NOTE: the below hackery is due to not including std

#[lang="panic_fmt"]
extern fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
  loop {}
}

// As we are not using interrupts, we just register a dummy catch all handler
#[allow(dead_code)]
#[used]
#[link_section = ".vector_table.interrupts"]
static INTERRUPTS: [extern "C" fn(); 97] = [default_handler; 97];
// static INTERRUPTS: [extern "C" fn(); 24] = [default_handler; 24];
// static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
