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
            let mut rng = SmallRng::from_entropy();
            for _ in 0..(GIB / MIB) {
                let mut chunk = vec![0u8; MIB];
                rng.fill(chunk.as_mut_slice());
                rng_buffer.extend(chunk);
            }
            GiBObject { buffer: rng_buffer }
        })
        .collect();

    Ok(memory_buffers)
}
