extern crate markov;

use markov::Chain;
use std::process;

fn main() {
    let mut chain = Chain::of_order(1);
    //     for line in chain.iter_for(50) {
    //         let sentence: String = line.into_iter().map(|word| word + " ").collect();
    //         println!("{:?}", sentence);
    //     }
}
