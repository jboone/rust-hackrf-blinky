#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate lpc43xx;

use rtfm::{app, Threshold};

app! {
	device: lpc43xx,

	resources: {
		static ON: bool = false;
	},

	tasks: {
		SYS_TICK: {
			path: sys_tick,
			resources: [SCU, CREG, GPIO_PORT, ON],
		},
	}
}

fn init(p: init::Peripherals, _r: init::Resources) {
	p.SCU.sfsp4_1.write(|w| w
		.mode().function_0_default()
		.epd().disable_pull_down()
		.epun().disable_pull_up()
		.ehs().slow_low_noise_with_medium_speed()
		.ezi().disable_input_buffer()
		.zif().enable_input_glitch()
	);
	p.SCU.sfsp4_2.write(|w| w
		.mode().function_0_default()
		.epd().disable_pull_down()
		.epun().disable_pull_up()
		.ehs().slow_low_noise_with_medium_speed()
		.ezi().disable_input_buffer()
		.zif().enable_input_glitch()
	);
	p.SCU.sfsp6_12.write(|w| w
		.mode().function_0_default()
		.epd().disable_pull_down()
		.epun().disable_pull_up()
		.ehs().slow_low_noise_with_medium_speed()
		.ezi().disable_input_buffer()
		.zif().enable_input_glitch()
	);

	p.GPIO_PORT.dir2.modify(|_, w| w
		.dirp1().set_bit()
		.dirp2().set_bit()
		.dirp8().set_bit()
	);

    // p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(12_000_000); // 1s
    p.SYST.enable_interrupt();
	p.SYST.enable_counter();
}

fn idle() -> ! {
	loop {
		rtfm::wfi();
	}
}

fn sys_tick(_t: &mut Threshold, r: SYS_TICK::Resources) {
/*
	match r.CREG.m4memmap.read().m4map().bits() {
		0x10000 => r.GPIO_PORT.set2.write(|w| w.setp1().set_bit()),
		0x20000 => r.GPIO_PORT.set2.write(|w| w.setp2().set_bit()),
		0x80000 => r.GPIO_PORT.set2.write(|w| w.setp8().set_bit()),
		_ => {}
	}
*/
	r.GPIO_PORT.not2.write(|w| w
		.notp1().set_bit()
		.notp2().set_bit()
		.notp8().set_bit()	
	);
}
