use tokio::time::{Duration, Instant, sleep_until};
use std::time::SystemTime;
use chrono::{DateTime, Local};

pub async fn calculate_pi(duration: Duration) -> f64 {
    let now: DateTime<Local> = SystemTime::now().into();
    println!("{} Started using CPU", now.format("%Y-%m-%d %H:%M:%S").to_string());

    let mut pi = 0.0;
    let mut sign = 1.0;
    let start_time = Instant::now();
    let end_time = start_time + duration;

    for k in 0.. {
        pi += sign * (4.0 / (8.0 * k as f64 + 1.0) - 2.0 / (8.0 * k as f64 + 4.0) - 1.0 / (8.0 * k as f64 + 5.0) - 1.0 / (8.0 * k as f64 + 6.0)) / (16.0_f64).powf(k as f64);
        sign *= -1.0;

        if start_time.elapsed() >= duration {
            break;
        }

        sleep_until(end_time).await;
    }

    pi
}
