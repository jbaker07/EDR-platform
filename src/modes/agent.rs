// edr-agent/src/agent.rs

use std::thread;
use std::time::Duration;

use crate::modes::{minimal, forensic};
use crate::runtime::telemetry_writer::submit_batch_telemetry;
use crate::runtime::policy_validator::get_current_mode;
use crate::runtime::mode_switcher::check_for_mode_escalation;

pub struct Agent {
    pub mode: Mode,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Minimal,
    Forensic,
}

impl Agent {
    pub fn new(initial_mode: Mode) -> Self {
        Agent { mode: initial_mode }
    }

    pub fn start(&mut self) {
        match self.mode {
            Mode::Minimal => self.start_minimal_mode(),
            Mode::Forensic => self.start_forensic_mode(),
        }
    }

    fn start_minimal_mode(&mut self) {
        println!("⚡ [Agent] Starting in Minimal Mode");
        minimal::start_minimal_mode();

        // Optional: Monitor for escalation trigger
        thread::spawn({
            let current_mode = self.mode;
            move || loop {
                if check_for_mode_escalation() {
                    println!("🔁 Escalating to Forensic Mode");
                    // In real usage, you'd notify main.rs to restart agent in forensic mode
                }
                thread::sleep(Duration::from_secs(30));
            }
        });
    }

    fn start_forensic_mode(&self) {
        println!("🕵️ [Agent] Starting in Forensic Mode");
        forensic::start_forensic_engine();

        thread::spawn(|| loop {
            let telemetry_batch = forensic::gather_all_forensic_telemetry(); // You must implement this
            submit_batch_telemetry(telemetry_batch); // Send to backend or replay buffer
            thread::sleep(Duration::from_secs(60));
        });
    }
}
