extern crate cryptopals;
extern crate hex;

use cryptopals::*;
use std::fmt;

struct ComputedXor {
    mask: u8,
    score: f32,
    value: Vec<u8>,
}

impl ComputedXor {
    fn new() -> ComputedXor {
        ComputedXor {
            mask: 0,
            score: 0.0,
            value: Vec::new(),
        }
    }
}

impl fmt::Display for ComputedXor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "mask: {}, score: {:000.3}, val: {}",
            self.mask,
            self.score,
            String::from_utf8_lossy(&self.value)
        )
    }
}

pub fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let input_bytes = hex::decode(input).unwrap();

    let best_mask = (0..=255)
        .map(|mask| {
            let mut score = 0.0;
            let value = input_bytes
                .iter()
                .map(|b| {
                    let unmasked = b ^ mask;
                    score += LETTER_FREQUENCIES.get(&(unmasked as char)).unwrap_or(&0.0);
                    unmasked
                }).collect();
            ComputedXor {
                score: score,
                value: value,
                mask: mask,
            }
        }).fold(ComputedXor::new(), |acc, x| {
            if x.score > acc.score {
                x
            } else {
                acc
            }
        });

    println!("{}", best_mask);
}