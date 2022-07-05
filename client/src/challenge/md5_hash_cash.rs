use common::challenge::models_md5_hash_cash::{MD5HashCashInput, MD5HashCashOutput};
use md5;
use md5::Digest;
use std::ops::Deref;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use byteorder::{BigEndian, ReadBytesExt};

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
                if *ended.lock().unwrap() == true {
                    break;
                } else {
                    let mut digest: Digest = md5::compute(format!("{:016X}{}", j, input_clone.message));

                    if check_number_of_bit_at_zero(digest.as_slice(), input_clone.complexity) == true {
                        let mut ended_mutex = ended.lock().unwrap();
                        if *ended_mutex == false {
                            *ended_mutex = true;
                            *res.lock().unwrap() = MD5HashCashOutput { seed: j, hashcode: format!("{:032X}", digest) };
                            cvar.notify_one();
                        }

                        break;
                    }
                }
            }
        });
    }

    let (cvar, ended, res) = &*pair;

    let mut ended_mutex = ended.lock().unwrap();
    while *ended_mutex == false {
        ended_mutex = cvar.wait(ended_mutex).unwrap();
    }

    return res.lock().unwrap().deref().clone();
}

fn check_number_of_bit_at_zero(number: &[u8], expected_of_zero: u32) -> bool {
    let mut number_as_bits: u128 = number[0] as u128;
    for i in 1..number.len() {
        number_as_bits = number_as_bits << 8;
        number_as_bits &= number[i] as u128;
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
