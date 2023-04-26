//! Reproduction of the calculations that Satoshi made in section 12 of the bitcoin whitepaper.

use libm::{exp, pow};

fn main() {
    // Trying to reproduce Satoshi's results

    // q=0.1
    // z=0 P=1.0000000
    // z=1 P=0.2045873
    // z=2 P=0.0509779
    // z=3 P=0.0131722
    // z=4 P=0.0034552
    // z=5 P=0.0009137
    // z=6 P=0.0002428
    // z=7 P=0.0000647
    // z=8 P=0.0000173
    // z=9 P=0.0000046
    // z=10 P=0.0000012

    let q = 0.1;
    println!("q = {q}");
    for z in 0..=10 {
        let p = attacker_success_probability(0.1, z);
        println!("z={:2}, P={}", z, p);
    }
    
    // q=0.3
    // z=0 P=1.0000000
    // z=5 P=0.1773523
    // z=10 P=0.0416605
    // z=15 P=0.0101008
    // z=20 P=0.0024804
    // z=25 P=0.0006132
    // z=30 P=0.0001522
    // z=35 P=0.0000379
    // z=40 P=0.0000095
    // z=45 P=0.0000024
    // z=50 P=0.0000006
    
    let q = 0.3;
    println!("q = {q}");
    for z in 0..=10 {
        let p = attacker_success_probability(0.1, z);
        println!("z={:2}, P={}", z, p);
    }

    // Solving for P less than 0.1%...
    // P < 0.001
    // q=0.10 z=5
    // q=0.15 z=8
    // q=0.20 z=11
    // q=0.25 z=15
    // q=0.30 z=24
    // q=0.35 z=41
    // q=0.40 z=89
    // q=0.45 z=340

    //TODO
}

/// Calculate the probability that an attacking miner will ever catch up from a deficit of z
/// blocks while trying to overtake the main chain (eg to execute a double spend).
/// 
/// Based on Satoshi's original C code from section 12.
/// 
/// #include <math.h>
/// double AttackerSuccessProbability(double q, int z)
/// {
///   double p = 1.0 - q;
///   double lambda = z * (q / p);
///   double sum = 1.0;
///   int i, k;
///   for (k = 0; k <= z; k++)
///   {
///     double poisson = exp(-lambda);
///     for (i = 1; i <= k; i++)
///       poisson *= lambda / i;
///     sum -= poisson * (1 - pow(q / p, z - k));
///   }
///   return sum;
/// }

fn attacker_success_probability(q: f64, z: u64) -> f64 {
    
    let p = 1.0 - q;
    let lambda = z as f64 * (q / p);
    let mut sum = 1.0f64;
    
    for k in 0..=z {
        let mut poisson = exp(-lambda);
        
        for i in 1..=k {
            poisson *= lambda / i as f64;
        }
        
        sum -= poisson * (1.0 - pow(q / p, (z - k) as f64));
    }
    
    sum
}