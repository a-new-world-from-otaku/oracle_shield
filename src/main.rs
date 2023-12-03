mod args;
mod cpu;
mod memory;
mod signal;

use args::{parse_and_validate_args, Args};
use signal::set_signal_handler;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{Duration, Instant, sleep};

const CALCULATION_INTERVAL_FACTOR: u64 = 30 * 24 * 60 * 60;
const CALCULATION_DURATION: u64 = 5 * 60;

#[tokio::main]
async fn main() {
    let Args { memory, frequency } = parse_and_validate_args();
    let memory_buffers = match memory {
        Some(gib) => match memory::allocate_memory(gib) {
            Ok(buffers) => Some(buffers),
            Err(e) => {
                eprintln!("Failed to allocate memory: {}", e);
                None
            }
        },
        None => None,
    };

    let interval = frequency
        .map(|f| Duration::from_secs(CALCULATION_INTERVAL_FACTOR / f as u64))
        .unwrap_or(Duration::from_secs(u64::MAX));

    let is_running = Arc::new(AtomicBool::new(true));
    set_signal_handler(is_running.clone()).await;

    let mut last_calculation = Instant::now();
    let mut calculation_start_time = None;

    while is_running.load(Ordering::SeqCst) {
        if let Some(buffers) = &memory_buffers {
            for buffer in buffers {
                let _ = buffer.buffer[0];
            }
        }
        if last_calculation.elapsed() >= interval {
            calculation_start_time = Some(Instant::now());
            last_calculation = Instant::now();
        }

        if let Some(start_time) = calculation_start_time {
            if start_time.elapsed() < Duration::from_secs(CALCULATION_DURATION) {
                cpu::calculate_pi(Duration::from_secs(CALCULATION_DURATION)).await;
            } else {
                calculation_start_time = None;
            }
        }

        sleep(Duration::from_millis(100)).await;
    }
}
