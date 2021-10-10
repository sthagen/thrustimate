extern crate rand;

use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

const TOTAL_TRIALS: i32 = 100_000; // too low for trials but good 'nuff for development
const TOTAL_TICKETS: i32 = 12 * 20 * 9; // months x work days x actor(s)
const PERCENT_DOMAIN: i32 = 100;

const SPRINT_DAILY_TPS: [i32; 10] = [4, 9, 10, 13, 9, 6, 8, 18, 8, 8];

fn main() {
    let mut rng = thread_rng();
    let throughput = Uniform::from(0..SPRINT_DAILY_TPS.len());

    let working_days_of_year = 12 * 20;
    let mut successes = 0;

    for _ in 0..TOTAL_TRIALS {
        let mut tickets_completed = 0;

        for _ in 0..working_days_of_year + 1 {
            let random_index = throughput.sample(&mut rng);
            tickets_completed += SPRINT_DAILY_TPS[random_index];
        }

        if tickets_completed > TOTAL_TICKETS {
            successes += 1
        }
    }

    let prob_perc = f64::from(successes) / f64::from(TOTAL_TRIALS) * f64::from(PERCENT_DOMAIN);

    println!("Total simulations: {}", TOTAL_TRIALS);
    println!("Successes: {}", successes);
    println!("Probability of succeeding: {:.2}%", prob_perc);
}
