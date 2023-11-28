use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::Rng;
use rayon::prelude::*;
use std::time::SystemTime;
use chrono::{DateTime, Local};

const KIB: usize = 1024;
const MIB: usize = 1024 * KIB;
const GIB: usize = 1024 * MIB;

pub struct GiBObject {
    pub buffer: Vec<u8>,
}

pub fn allocate_memory(gib: i32) -> Result<Vec<GiBObject>, Box<dyn std::error::Error>> {
    let current_time: DateTime<Local> = SystemTime::now().into();
    println!("{} Started using memory", current_time.format("%Y-%m-%d %H:%M:%S").to_string());

    let memory_buffers: Vec<GiBObject> = (0..gib)
        .into_par_iter()
        .map(|_| {
            let mut rng_buffer = Vec::with_capacity(GIB);
            rng_buffer.resize(GIB, 0);

            let mut rng = SmallRng::from_entropy();
            rng_buffer
                .chunks_mut(MIB)
                .for_each(|chunk| {
                    for byte in chunk {
                        *byte = rng.gen::<u8>();
                    }
                });

            GiBObject { buffer: rng_buffer }
        })
        .collect();

    Ok(memory_buffers)
}

pub fn simulate_memory_usage(buffers: &[GiBObject]) {
    let mut rng = rand::thread_rng();

    for buffer in buffers {
        if rng.gen_bool(0.5) {
            for byte in &buffer.buffer {
                let _ = *byte;
            }
        } else {
            let buffer_len = buffer.buffer.len();
            for _ in 0..100 {
                let index = rng.gen_range(0..buffer_len);
                let _ = buffer.buffer[index];
            }
        }
    }
}
