mod args;
mod cpu;
mod memory;
mod signal;

use args::{parse_and_validate_args, Args};
use signal::set_signal_handler;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{Duration, sleep};
use cgroups_rs::{cgroup_builder::CgroupBuilder, CgroupPid, hierarchies::auto};

#[tokio::main]
async fn main() {
    let Args { memory, percent } = parse_and_validate_args();

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

    let is_running = Arc::new(AtomicBool::new(true));
    set_signal_handler(is_running.clone()).await;

    // Create cgroup
    let cg = if let Some(cpu_percent) = percent {
        let hier = auto();
        let cgroup = CgroupBuilder::new("oracle_shield")
            .cpu()
            .shares(cpu_percent as u64)
            .done()
            .build(hier)
            .expect("Failed to create cgroup");
        
        let pid = CgroupPid::from(std::process::id() as u64);
        cgroup.add_task(pid).expect("Failed to add task to cgroup");

        Some(cgroup)
    } else {
        None
    };

    while is_running.load(Ordering::SeqCst) {
        cpu::calculate_pi(Duration::from_secs(u64::MAX)).await;

        if let Some(buffers) = &memory_buffers {
            for buffer in buffers {
                let _ = buffer.buffer[0];
            }
        }
        
        sleep(Duration::from_millis(100)).await;
    }

    if let Some(cgroup) = cg {
        cgroup.delete().expect("Failed to delete cgroup");
    }
}
