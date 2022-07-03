use std::ops::Deref;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use md5;
use md5::Digest;
use common::challenge::models_md5_hash_cash::{MD5HashCashInput, MD5HashCashOutput};

pub fn md5_challenge_resolver(input: MD5HashCashInput) -> MD5HashCashOutput {
    let pair = Arc::new((Mutex::<u64>::new(0), Condvar::new(), Mutex::new(false), Mutex::<u64>::new(0), Mutex::<Digest>::new(Digest([0;16]))));

    for _ in 0..10 {
        let pair_clone = Arc::clone(&pair);
        let input_clone = input.clone();

        thread::spawn(move || {
            let (lock, cvar, ended,res_seed, res_digest) = &*pair_clone;

            let mut seed_mutex = lock.lock().unwrap();
            *seed_mutex += 1;
            let mut seed = seed_mutex.deref().clone();
            drop(seed_mutex);


            let mut digest: Digest = md5::compute(format!("{:016X}{}", seed, input_clone.message));
            while check_number_of_bit_at_zero(digest.as_slice(), input_clone.complexity) == false && *ended.lock().unwrap() == false {

                seed_mutex = lock.lock().unwrap();
                *seed_mutex += 1;
                seed = seed_mutex.deref().clone();
                drop(seed_mutex);

                digest = md5::compute(format!("{:016X}{}", seed, input_clone.message.clone()));
            }

            let mut ended_mutex = ended.lock().unwrap();

            if *ended_mutex == false {
                *ended_mutex = true;
                *res_digest.lock().unwrap() = digest;
                *res_seed.lock().unwrap() = seed;
                cvar.notify_one();
            }
        });
    }

    let (_, cvar,ended,res_seed, res_digest) = &*pair;

    let mut ended_mutex = ended.lock().unwrap();
    while *ended_mutex == false {
        ended_mutex = cvar.wait(ended_mutex).unwrap();
    }

    return MD5HashCashOutput { seed: *res_seed.lock().unwrap(), hashcode: format!("{:032X}", *res_digest.lock().unwrap()) };
}

fn check_number_of_bit_at_zero(number: &[u8], expected_of_zero: u32) -> bool {

    let mut number_as_bits: u128 = number[0] as u128;
    for i in 1..number.len() {
        number_as_bits = number_as_bits << 8;
        number_as_bits += number[i] as u128;
    }
    number_as_bits = number_as_bits.reverse_bits();
    let mut number_of_zero = 0;
    while number_of_zero < expected_of_zero {
        if (number_as_bits & 0x1) == 0 {
            number_of_zero += 1;
        } else {
            return false;
        }
        number_as_bits = number_as_bits >> 1;
    }
    return true;
}