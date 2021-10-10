extern crate rand;

use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

const TOTAL_TRIALS: i32 = 100_000; // too low for trials but good 'nuff for development
const PERCENT_DOMAIN: i32 = 100;
const MIN_VALUE: i32 = 1;
const SUP_VALUE: i32 = 11; // maximum is one less

fn main() {
    let mut rng = thread_rng();
    let trial = Uniform::from(MIN_VALUE..SUP_VALUE);

    let the_sum = 1 + SUP_VALUE - 1;
    let mut sum_matches = 0;

    for _ in 0..TOTAL_TRIALS {
        let first_trial = trial.sample(&mut rng);
        let second_trial = trial.sample(&mut rng);

        if first_trial + second_trial == the_sum {
            sum_matches += 1
        }
    }

    let prob_perc: f64 =
        f64::from(sum_matches) / f64::from(TOTAL_TRIALS) * f64::from(PERCENT_DOMAIN);

    println!("Total trials: {}", TOTAL_TRIALS);
    println!("Total matches ({}): {}", the_sum, sum_matches);
    println!(
        "Probability of achieving the sum {}: {:.2}%",
        the_sum, prob_perc
    );
}
