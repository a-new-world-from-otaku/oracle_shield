use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use std::sync::mpsc;

const KIB: usize = 1024;
const MIB: usize = 1024 * KIB;
const GIB: usize = 1024 * MIB;

pub struct GiBObject {
    pub buffer: Arc<RwLock<Vec<u8>>>,
}

pub fn allocate_memory(gib: i32) -> Result<Vec<GiBObject>, Box<dyn std::error::Error>> {
    let current_time: DateTime<Local> = SystemTime::now().into();
    println!("{} Started using memory", current_time.format("%Y-%m-%d %H:%M:%S").to_string());

    let mut memory_buffers = Vec::with_capacity(gib as usize);

    let (sender, receiver) = mpsc::channel();

    (0..gib).into_par_iter().for_each_with(sender, |sender, _| {
        let mut rng_buffer = Vec::with_capacity(GIB);
        rng_buffer.resize(GIB, 0);
        rng_buffer.par_chunks_mut(MIB).for_each(|chunk| {
            let mut rng = SmallRng::from_entropy();
            for byte in chunk {
                *byte = rng.gen::<u8>();
            }
        });
        let rng_buffer = Arc::new(RwLock::new(rng_buffer));
        let memory_object = GiBObject { buffer: rng_buffer.clone() };
        if let Err(e) = sender.send(memory_object) {
            eprintln!("Failed to send memory buffer: {}", e);
        }
    });

    for memory_object in receiver {
        memory_buffers.push(memory_object);
    }

    Ok(memory_buffers)
}
