use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

const NEIGHBORS: [(i64, i64); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Debug, PartialEq, Clone)]
struct LifeState {
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl LifeState {
    fn neighbours_count(&self, i: usize) -> u8 {
        let mut ret = 0;
        let x = (i % self.width) as i64;
        let y = (i / self.height) as i64;
        for (nx, ny) in NEIGHBORS {
            let nx = nx + x;
            let ny = ny + y;
            if nx >= 0 && nx < self.width as i64 && ny >= 0 && ny < self.height as i64 {
                match self.data[nx as usize + ny as usize * self.width] {
                    true => ret += 1,
                    false => {}
                }
            }
        }
        ret
    }
}

impl Iterator for LifeState {
    type Item = LifeState;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new = LifeState {
            width: self.width,
            height: self.height,
            data: Vec::with_capacity(self.data.len()),
        };
        for (i, val) in self.data.iter().enumerate() {
            let neighbours_count = self.neighbours_count(i);
            let new_val = match (val, neighbours_count) {
                (true, 2) | (true, 3) | (false, 3) => true,
                _ => false,
            };
            new.data.push(new_val);
        }
        Some(new)
    }
}

impl From<io::Lines<io::BufReader<File>>> for LifeState {
    fn from(lines: io::Lines<io::BufReader<File>>) -> Self {
        let mut ret = LifeState {
            width: 0,
            height: 0,
            data: Vec::with_capacity(lines.size_hint().0 * 100),
        };
        for line_result in lines {
            match line_result {
                Ok(line) => {
                    ret.height += 1;
                    let mut line_bools: Vec<bool> = line.chars().map(|c| c == 'X').collect();
                    if line_bools.len() != 0 {
                        ret.width = line_bools.len();
                        ret.data.append(&mut line_bools)
                    }
                }
                Err(e) => panic!("{}", e),
            }
        }
        assert!(
            ret.width <= std::i64::MAX as usize,
            "The state width is bigger than i64::MAX"
        );
        assert!(
            ret.height <= std::i64::MAX as usize,
            "The state height is bigger than i64::MAX"
        );
        ret
    }
}

impl fmt::Display for LifeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        for (i, cell) in self.data.iter().enumerate() {
            match *cell {
                true => ret.push_str("X"),
                false => ret.push_str("."),
            }
            if i % self.width == self.width - 1 {
                ret.push_str("\n");
            }
        }
        write!(f, "{}", ret)
    }
}

fn main() -> Result<(), &'static str> {
    // args check
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: ./{} initial_state iterations", &args[0]);
        return Err("Error: Expected 2 arguments.");
    }

    // iterations extraction
    let iterations = args[2].parse::<usize>();
    if iterations.is_err() {
        return Err("Error: Couldn't parse iterations argument. Expected usize.");
    }
    // file open
    let lines = match File::open(&args[1]) {
        Ok(file) => io::BufReader::new(file).lines(),
        Err(_) => return Err("Error: Couldn't open file"),
    };
    // create init state
    let mut life = LifeState::from(lines);
    // loop
    let life = life.next().unwrap();
    // print result
    print!("{}", life);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn neighbours_count() {
        let mut life = LifeState {
            width: 3,
            height: 3,
            data: vec![false; 9],
        };

        // No neighbours
        for i in 0..9 {
            assert_eq!(life.neighbours_count(i), 0)
        }

        // One
        life.data[0] = true;
        assert_eq!(life.neighbours_count(4), 1);

        // Two
        life.data[1] = true;
        assert_eq!(life.neighbours_count(4), 2);

        // Three
        life.data[2] = true;
        assert_eq!(life.neighbours_count(4), 3);

        // Four
        life.data[3] = true;
        assert_eq!(life.neighbours_count(4), 4);

        // Four (shouldn't consider self)
        life.data[4] = true;
        assert_eq!(life.neighbours_count(4), 4);

        // Five
        life.data[5] = true;
        assert_eq!(life.neighbours_count(4), 5);

        // Six
        life.data[6] = true;
        assert_eq!(life.neighbours_count(4), 6);

        // Seven
        life.data[7] = true;
        assert_eq!(life.neighbours_count(4), 7);

        // Eight
        life.data[8] = true;
        assert_eq!(life.neighbours_count(4), 8);
    }

    #[test]
    fn rule_s2() {
        // ...
        // XXX
        // ...
        let mut life = LifeState {
            width: 3,
            height: 3,
            data: vec![false; 9],
        };

        life.data[4] = true;
        life.data[3] = true;
        life.data[5] = true;

        let init_state = life.clone();

        // .X.
        // .X.
        // .X.
        let life2 = LifeState {
            width: 3,
            height: 3,
            data: vec![false, true, false, false, true, false, false, true, false],
        };
        let mut life = life.next().unwrap();
        assert_eq!(life.data[4], true);
        assert_eq!(life, life2);

        // with next iteration the life should return to previous state
        let life = life.next().unwrap();
        assert_eq!(life, init_state);
    }

    #[test]
    fn rule_s3() {
        // ...
        // XXX
        // X..
        let init_state = LifeState {
            width: 3,
            height: 3,
            data: vec![false, false, false, true, true, true, true, false, false],
        };

        let mut life = init_state.clone();

        // .X.
        // XX.
        // X..
        let life_next = LifeState {
            width: 3,
            height: 3,
            data: vec![false, true, false, true, true, false, true, false, false],
        };
        let mut life = life.next().unwrap();
        assert_eq!(life.data[4], true); // S3
        assert_eq!(life, life_next);

        // XX.
        // XX.
        // XX.
        let life_next = LifeState {
            width: 3,
            height: 3,
            data: vec![true, true, false, true, true, false, true, true, false],
        };
        let life = life.next().unwrap();
        assert_eq!(life, life_next);
    }

    #[test]
    fn rule_b3() {
        // X..
        // ...
        // X.X
        let init_state = LifeState {
            width: 3,
            height: 3,
            data: vec![true, false, false, false, false, false, true, false, true],
        };

        let mut life = init_state.clone();
        // ...
        // .X.
        // ...
        let life_next = LifeState {
            width: 3,
            height: 3,
            data: vec![false, false, false, false, true, false, false, false, false],
        };
        let life = life.next().unwrap();
        assert_eq!(life.data[4], true); // B3
        assert_eq!(life, life_next);
    }

    /// Tests that the results outside of ruleset work
    /// L0, L1, L4, L5, L6, L7, L8 -> D
    /// D0-2, D4-8 -> D
    #[test]
    fn no_rule() {
        // L0 -> D
        // ...
        // .X.
        // ...
        let mut life = LifeState {
            width: 3,
            height: 3,
            data: vec![false, false, false, false, true, false, false, false, false],
        };

        let life = life.next().unwrap();
        assert_eq!(life.data[4], false); // L0

        // L1 -> D
        // X..
        // .X.
        // ...
        let mut life = LifeState {
            width: 3,
            height: 3,
            data: vec![true, false, false, false, true, false, false, false, false],
        };

        let life = life.next().unwrap();
        assert_eq!(life.data[4], false); // L1

        // L4 -> D
        // XXX
        // XX.
        // ...
        let mut life = LifeState {
            width: 3,
            height: 3,
            data: vec![true, true, true, true, true, false, false, false, false],
        };

        let life = life.next().unwrap();
        assert_eq!(life.data[4], false); // L4

        // L5 -> D
        // XXX
        // XXX
        // ...
        let mut life = LifeState {
            width: 3,
            height: 3,
            data: vec![true, true, true, true, true, true, false, false, false],
        };

        let life = life.next().unwrap();
        assert_eq!(life.data[4], false); // L5

        // L6 -> D
        // XXX
        // XXX
        // X..
        let mut life = LifeState {
            width: 3,
            height: 3,
            data: vec![true, true, true, true, true, true, true, false, false],
        };

        let life = life.next().unwrap();
        assert_eq!(life.data[4], false); // L6

        // L7 -> D
        // XXX
        // XXX
        // XX.
        let mut life = LifeState {
            width: 3,
            height: 3,
            data: vec![true, true, true, true, true, true, true, true, false],
        };

        let life = life.next().unwrap();
        assert_eq!(life.data[4], false); // L7

        // L8 -> D
        // XXX
        // XXX
        // XXX
        let mut life = LifeState {
            width: 3,
            height: 3,
            data: vec![true, true, true, true, true, true, true, true, true],
        };

        let life = life.next().unwrap();
        assert_eq!(life.data[4], false); // L8

        // Dead stays dead loop
        // ...
        // ...
        // ...
        let mut init_state = LifeState {
            width: 3,
            height: 3,
            data: vec![false; 9],
        };

        for i in 0..9 {
            if i != 4 {
                init_state.data[i] = true;
            }
            let life = init_state.next().unwrap();
            // the cell should remain dead if i != 2 aka neighbours_count is != 3
            if i != 2 {
                assert_eq!(life.data[4], false);
            } else {
                // we can check the rule here, why not
                assert_eq!(life.data[4], true);
            }
        }
    }
}
