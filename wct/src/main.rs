extern crate rand;

use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

const TOTAL_TRIALS: i32 = 100_000; // too low for trials but good 'nuff for development
const TOTAL_TICKETS: i32 = 12 * 20 * 1; // months x work days x actor(s)
const PERCENT_DOMAIN: i32 = 100;

const DAYS_TAKEN: [i32; 11] = [0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2];

fn main() {
    let mut rng = thread_rng();
    let time_to_completion = Uniform::from(0..DAYS_TAKEN.len());

    let working_days_of_year = 12 * 20;
    let mut successes = 0;

    for _ in 0..TOTAL_TRIALS {
        let mut current_duration = 0;

        for _ in 0..TOTAL_TICKETS {
            let random_index = time_to_completion.sample(&mut rng);
            current_duration += DAYS_TAKEN[random_index];
        }

        if current_duration <= working_days_of_year {
            successes += 1
        }
    }

    let prob_perc = f64::from(successes) / f64::from(TOTAL_TRIALS) * f64::from(PERCENT_DOMAIN);

    println!("Total simulations: {}", TOTAL_TRIALS);
    println!("Successes: {}", successes);
    println!("Probability of succeeding: {:.2}%", prob_perc);
}
