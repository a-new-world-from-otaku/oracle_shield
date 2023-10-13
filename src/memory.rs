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
    pub b: Arc<RwLock<Vec<u8>>>,
}

pub fn memory(gib: i32) -> Result<Vec<GiBObject>, Box<dyn std::error::Error>> {
    let now: DateTime<Local> = SystemTime::now().into();
    println!("{} Started using memory", now.format("%Y-%m-%d %H:%M:%S").to_string());

    let mut buffers = Vec::with_capacity(gib as usize);

    let (tx, rx) = mpsc::channel();

    (0..gib).into_par_iter().for_each_with(tx, |tx, _| {
        let mut v = Vec::with_capacity(GIB);
        v.resize(GIB, 0);
        v.par_chunks_mut(MIB).for_each(|chunk| {
            let mut rng = SmallRng::from_entropy();
            for byte in chunk {
                *byte = rng.gen::<u8>();
            }
        });
        let v = Arc::new(RwLock::new(v));
        let o = GiBObject { b: v.clone() };
        if let Err(e) = tx.send(o) {
            eprintln!("Failed to send memory buffer: {}", e);
        }
    });

    for o in rx {
        buffers.push(o);
    }

    Ok(buffers)
}
