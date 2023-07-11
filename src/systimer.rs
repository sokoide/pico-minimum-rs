use cortex_m_rt::exception;
use volatile_register::{RO, RW};

pub static mut COUNTER: u32 = 0;

pub const TIMER_PERIOD: u32 = 10;
const TMCLK_KHZ: u32 = 125 * 1000;

// const SYST_COUNTER_MASK: u32 = 0x00ff_ffff;
const SYST_CSR_ENABLE: u32 = 1 << 0;
const SYST_CSR_TICKINT: u32 = 1 << 1;
const SYST_CSR_CLKSOURCE: u32 = 1 << 2;
const SYST_CSR_COUNTFLAG: u32 = 1 << 16;

pub struct SystemTimer {
    p: &'static mut RegisterBlock,
}

#[repr(C)]
struct RegisterBlock {
    pub csr: RW<u32>,
    pub rvr: RW<u32>,
    pub cvr: RW<u32>,
    pub calib: RO<u32>,
}

impl Default for SystemTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemTimer {
    pub fn new() -> SystemTimer {
        SystemTimer {
            p: unsafe { &mut *(0xE000_E010 as *mut RegisterBlock) },
        }
    }

    pub fn init(&mut self) {
        unsafe {
            // Stop SysTick
            self.p.csr.write(SYST_CSR_CLKSOURCE | SYST_CSR_TICKINT);
            // Set reload
            self.p.rvr.write(TIMER_PERIOD * TMCLK_KHZ - 1);
            // Set counter
            self.p.cvr.write(TIMER_PERIOD * TMCLK_KHZ - 1);
            // Start SysTick
            self.p
                .csr
                .write(SYST_CSR_CLKSOURCE | SYST_CSR_TICKINT | SYST_CSR_ENABLE);
        }
    }

    #[inline]
    pub fn has_wrapped(&mut self) -> bool {
        self.p.csr.read() & SYST_CSR_COUNTFLAG != 0
    }

    pub fn delay_ms(&mut self, ms: u32) {
        let mut counter = ms / TIMER_PERIOD;
        let mut st = SystemTimer::new();
        while counter > 0 {
            if st.has_wrapped() {
                counter -= 1;
            }
        }
    }
}

#[exception]
fn SysTick() {
    unsafe {
        if COUNTER == 0xFFFF_FFFF {
            COUNTER = 0;
        } else {
            COUNTER += 1;
        }
    }
}
