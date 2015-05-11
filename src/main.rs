extern crate rand;

use std::fmt::{self, Display, Formatter};

struct Puzzle {
    state: [u8; 9]
}

impl Puzzle {
    fn rand_puzzle() -> Self {
        let mut state: [u8; 9] = [0; 9];
        for (i, elt) in state.iter_mut().enumerate() {
            *elt = i as u8;
        }

        for i in (0..state.len()).rev() {
            let pos = rand::random::<u8>() % (i + 1) as u8;
            state.swap(i, pos as usize);
        }

        Puzzle { state: state }
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {} {}\n{} {} {}\n{} {} {}\n",
               self.state[0], self.state[1], self.state[2],
               self.state[3], self.state[4], self.state[5],
               self.state[6], self.state[7], self.state[8])
    }
}

fn main() {
    let puzzle = Puzzle::rand_puzzle();
    println!("Initial state:\n{}", puzzle);

    println!("Solving...");
    let solution = solve(puzzle);
    println!("Solution:\n{}", solution);
}

fn solve(puzzle: Puzzle) -> usize {
    unimplemented!()
}
