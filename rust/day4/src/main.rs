extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;


/*
extern crate md5;
use md5::compute;
use md5::Digest;

fn hash_for_value(secret: &str, solution: i32) -> &md5::Digest {
    let mut hash_string = String::from(secret);
    hash_string.push_str(&solution.to_string());

    return &md5::compute(hash_string.as_bytes());
}
*/


fn check_hash(hash: &[u8], zero_count: i32) -> bool {
    let mut count_left = zero_count;
    let mut position = 0;

    while count_left > 0 && position < 16 {
        if count_left >= 2 {
            count_left -= 2;

            if hash[position] != 0x00 {
                return false;
            } else if count_left == 0 {
                return true;
            }

            position += 1;
        }

        if count_left == 1 {
            if hash[position] & 0xF0 != 0x00 {
                return false;
            } else {
                return true
            }
        }
    }

    return false;
}


fn hash_for_value2(sh: &mut Md5, hash: &mut [u8], secret: &str, solution: i32) {
    sh.input(secret.as_bytes());
    sh.input(solution.to_string().as_bytes());
    sh.result(hash);
    sh.reset();
}


fn problem_one() {
    let mut counter = 0;
    let secret = "iwrupvqb";
    let mut hash: [u8; 16] = [0; 16];
    let mut sh = Md5::new();

    while counter < 10000000 {
        hash_for_value2(&mut sh, &mut hash, secret, counter);

        if check_hash(&hash, 5) {
            println!("Found {}", counter);
            return;
        }

        counter += 1;
    }
}


fn problem_two() {
    let mut counter = 0;
    let secret = "iwrupvqb";
    let mut hash: [u8; 16] = [0; 16];
    let mut sh = Md5::new();

    while counter < 10000000 {
        hash_for_value2(&mut sh, &mut hash, secret, counter);

        if check_hash(&hash, 6) {
            println!("Found {}", counter);
            return;
        }

        counter += 1;
    }
}


fn main() {
    problem_one();
    problem_two();
}
