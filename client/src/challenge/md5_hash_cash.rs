use common::challenge::models_md5_hash_cash::{MD5HashCashInput, MD5HashCashOutput};
use md5;
use md5::Digest;
use std::ops::Deref;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub fn md5_challenge_resolver(input: MD5HashCashInput, nb_thread: u64) -> MD5HashCashOutput {
    let pair = Arc::new((
        Condvar::new(),
        Mutex::new(false),
        Mutex::<MD5HashCashOutput>::new(MD5HashCashOutput { seed: 0, hashcode: "".to_string() })));

    let step = u64::MAX / nb_thread;

    for i in 0..nb_thread {
        let pair_clone = Arc::clone(&pair);
        let input_clone = input.clone();
        let min = step * i;
        let max = step * (i + 1);

        thread::spawn(move || {
            let (cvar, ended, res) = &*pair_clone;

            for j in min..max {
                if *ended.lock().expect("failed to lock mutex ended 1") == true {
                    break;
                } else {
                    let digest: Digest = md5::compute(format!("{:016X}{}", j, input_clone.message));

                    if check_number_of_bit_at_zero(digest.as_slice(), input_clone.complexity) == true {
                        let mut ended_mutex = ended.lock().expect("failed to lock mutex ended 2");
                        if *ended_mutex == false {
                            *ended_mutex = true;
                            *res.lock().expect("failed to lock res mutex 1") = MD5HashCashOutput { seed: j, hashcode: format!("{:032X}", digest) };
                            cvar.notify_one();
                        }

                        break;
                    }
                }
            }
        });
    }

    let (cvar, ended, res) = &*pair;

    let mut ended_mutex = ended.lock().expect("failed to lock mutex ended 3");
    while *ended_mutex == false {
        ended_mutex = cvar.wait(ended_mutex).expect("failed to wait on condvar");
    }

    return res.lock().expect("failed to lock res mutex 2").deref().clone();
}

fn check_number_of_bit_at_zero(number: &[u8], expected_of_zero: u32) -> bool {
    let mut number_as_bits: u128 = number[0] as u128;
    for i in 1..number.len() {
        number_as_bits = number_as_bits << 8;
        number_as_bits |= number[i] as u128;
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

#[cfg(test)]
mod md5_challenge_resolver_tests {
    use common::challenge::models_md5_hash_cash::MD5HashCashInput;
    use crate::md5_challenge_resolver;

    #[test]
    fn should_find_the_first_seed_for_hello() {
        let result = md5_challenge_resolver(MD5HashCashInput{ complexity: 9, message: "hello".to_string()}, 1);
        assert_eq!(844, result.seed);
        assert_eq!("00441745D9BDF8E5D3C7872AC9DBB2C3", result.hashcode);
    }

    #[test]
    fn should_find_the_first_seed_for_paul() {
        let result = md5_challenge_resolver(MD5HashCashInput{ complexity: 17, message: "Paul".to_string()}, 1);
        assert_eq!(163776, result.seed);
        assert_eq!("00005BC0D8B0898445DC6493EAEE0555", result.hashcode);
    }

    #[test]
    fn should_find_the_first_seed_for_clement() {
        let result = md5_challenge_resolver(MD5HashCashInput{ complexity: 15, message: "Clément".to_string()}, 1);
        assert_eq!(5783, result.seed);
        assert_eq!("00016A6CF04BAE18BCAA4D69B854CFCC", result.hashcode);
    }
}