use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::vec;

pub fn is_prime(n: u64) -> bool{
    if n <= 2 {
        return true;
    }
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

//APPROCCIO 1
pub fn find_primes_1(limit: u64, n_threads: u64) -> Vec<u64> {
    let counter = Arc::new(Mutex::new(2));
    let mut handles = vec![];
    let result = Arc::new(Mutex::new(Vec::new()));

    for _ in 0..n_threads {
        let counter = Arc::clone(&counter);
        let result = Arc::clone(&result);

        let handle = std::thread::spawn(move || {
            let mut local_primes = Vec::new();

            loop {
                let mut num = counter.lock().unwrap();
                if *num > limit {
                    break;
                }
                let current = *num;
                *num += 1;
                drop(num);

                if is_prime(current) {
                    local_primes.push(current);
                }
            }

            let mut res = result.lock().unwrap();
            res.extend(local_primes);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let primes = Arc::try_unwrap(result).unwrap().into_inner().unwrap();
    primes
}

//APPROCCIO 2
pub fn find_primes_2(limit: u64, n_threads: u64) -> Vec<u64> {
    let mut handles = vec![];
    for i in 0..n_threads {
        let handle = std::thread::spawn(move || {
            let mut local_primes = Vec::new();
            let mut num = i;
            loop {
                if num > limit {
                    break;
                }
                if is_prime(num) {
                    local_primes.push(num);
                }
                num += n_threads;
            }
            local_primes
        });
        handles.push(handle);
    }
    let mut all_primes = Vec::new();
    for handle in handles {
        let mut local = handle.join().unwrap();
        all_primes.append(&mut local);
    }
    all_primes.sort();
    all_primes
}

fn main() {
    let now = Instant::now();
    let primes = find_primes_2(100, 8);
    let elapsed = now.elapsed();
    println!("Found {} primes in {:?}", primes.len(), elapsed);
}

