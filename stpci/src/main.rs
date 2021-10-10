extern crate rand;

use chrono::TimeZone;
use chrono::Local;
use chrono::DateTime;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use std::collections::HashMap;
use chrono::Duration;

const TOTAL_TRIALS: i32 = 100_000; // too low for trials but good 'nuff for development
const TOTAL_TICKETS: i32 = 12 * 20 * 9; // months x work days x actor(s)
const SPRINT_DAILY_TPS: [i32; 10] = [4, 9, 10, 13, 9, 6, 8, 18, 8, 8];

fn calculate_percentile(data: &HashMap<i64, i32>, percentile: i32) {
    let total_sims: i32 = data.values().sum();

    let p_qtd: i32 = total_sims - (total_sims as f64 / (100_f64 / percentile as f64)).ceil() as i32;

    let mut hash_vec: Vec<(&i64, &i32)> = data.iter().collect();
    hash_vec.sort_by(|a, b| b.0.cmp(a.0));

    let mut sum = 0;
    let mut percentile_timestamp: i64 = 0;

    for (timestamp, freq) in hash_vec {
        sum += freq;

        if sum >= p_qtd {
            percentile_timestamp = *timestamp;
            break;
        }
    }

    println!(
        "Percentile {}: {} ({}/{})",
        percentile,
        Local
            .timestamp(percentile_timestamp, 0)
            .format("%Y/%m/%d")
            .to_string(),
        sum,
        total_sims
    );
}

fn main() {
    let mut rng = thread_rng();
    let throughput = Uniform::from(0..SPRINT_DAILY_TPS.len());

    let mut outcomes: HashMap<i64, i32> = HashMap::new();

    let one_day = Duration::days(1);

    let start_date: DateTime<Local> = Local::now();

    for _ in 0..TOTAL_TRIALS {
        let mut current_date = start_date;
        let mut tickets_completed = 0;

        while tickets_completed < TOTAL_TICKETS {
            let random_index = throughput.sample(&mut rng);
            tickets_completed += SPRINT_DAILY_TPS[random_index];
            current_date = current_date + one_day;
        }

        let count = outcomes.entry(current_date.timestamp()).or_insert(0);
        *count += 1;
    }

    println!("Total simulations: {}", TOTAL_TRIALS);
    calculate_percentile(&outcomes, 2);
    calculate_percentile(&outcomes, 5);
    calculate_percentile(&outcomes, 15);
    calculate_percentile(&outcomes, 25);
    calculate_percentile(&outcomes, 50);
    calculate_percentile(&outcomes, 75);
    calculate_percentile(&outcomes, 85);
    calculate_percentile(&outcomes, 95);
    calculate_percentile(&outcomes, 98);
}
