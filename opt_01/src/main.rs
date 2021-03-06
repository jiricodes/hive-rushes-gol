use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Clone)]
struct LifeState {
    width: usize,
    height: usize,
    data: Vec<Vec<u8>>,
}

impl LifeState {
    fn neighbours_count(&self, x: usize, y: usize) -> u8 {
        // This is safe, because of the padding introduced
        self.data[x - 1][y - 1]
            + self.data[x][y - 1]
            + self.data[x + 1][y - 1]
            + self.data[x - 1][y]
            + self.data[x + 1][y]
            + self.data[x - 1][y + 1]
            + self.data[x][y + 1]
            + self.data[x + 1][y + 1]
    }
}

impl Iterator for LifeState {
    type Item = LifeState;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new = self.clone();
        for y in 1..(self.height - 1) {
            for x in 1..(self.width - 1) {
                let neighbours_count = self.neighbours_count(x, y);
                let new_val = matches!(
                    (self.data[x][y], neighbours_count),
                    (1, 2) | (1, 3) | (0, 3)
                );
                new.data[x][y] = new_val.into();
            }
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
                    let mut line_bools: Vec<u8> =
                        line.chars().map(|c| if c == 'X' { 1 } else { 0 }).collect();
                    if !line_bools.is_empty() {
                        line_bools.insert(0, 0);
                        line_bools.push(0);
                        ret.width = line_bools.len();
                        ret.data.push(line_bools)
                    }
                }
                Err(e) => panic!("{}", e),
            }
        }
        ret.data.insert(0, vec![0; ret.width]);
        ret.data.push(vec![0; ret.width]);
        ret.height += 2;
        ret
    }
}

impl From<&str> for LifeState {
    fn from(s: &str) -> Self {
        let lines = s.split('\n');
        let mut ret = LifeState {
            width: 0,
            height: 0,
            data: Vec::with_capacity(lines.size_hint().0 * 100),
        };
        for line in lines {
            ret.height += 1;
            let mut line_bools: Vec<u8> =
                line.chars().map(|c| if c == 'X' { 1 } else { 0 }).collect();
            if !line_bools.is_empty() {
                line_bools.insert(0, 0);
                line_bools.push(0);
                ret.width = line_bools.len();
                ret.data.push(line_bools)
            }
        }
        ret.data.insert(0, vec![0; ret.width]);
        ret.data.push(vec![0; ret.width]);
        ret.height += 2;
        ret
    }
}

impl fmt::Display for LifeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        for (i, line) in self.data.iter().enumerate() {
            if i == 0 || i == self.height - 1 {
                continue;
            }
            let mut line_string: String = line
                .iter()
                .map(|x| if *x == 1 { 'X' } else { '.' })
                .collect();
            if i != self.height - 2 {
                line_string.push('\n');
            }
            ret.push_str(&line_string);
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
    for _ in 0..iterations.unwrap() {
        life = life.next().unwrap();
    }
    // print result
    print!("{}", life);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn neighbours_count() {
        let mut life = LifeState::from("...\n...\n...");

        // No neighbours
        for y in 1..4 {
            for x in 1..4 {
                assert_eq!(life.neighbours_count(x, y), 0)
            }
        }

        // One
        life.data[1][1] = 1;
        assert_eq!(life.neighbours_count(2, 2), 1);

        // Two
        life.data[1][2] = 1;
        assert_eq!(life.neighbours_count(2, 2), 2);

        // Three
        life.data[1][3] = 1;
        assert_eq!(life.neighbours_count(2, 2), 3);

        // Four
        life.data[2][1] = 1;
        assert_eq!(life.neighbours_count(2, 2), 4);

        // Four (shouldn't consider self)
        life.data[2][2] = 1;
        assert_eq!(life.neighbours_count(2, 2), 4);

        // Five
        life.data[2][3] = 1;
        assert_eq!(life.neighbours_count(2, 2), 5);

        // Six
        life.data[3][1] = 1;
        assert_eq!(life.neighbours_count(2, 2), 6);

        // Seven
        life.data[3][2] = 1;
        assert_eq!(life.neighbours_count(2, 2), 7);

        // Eight
        life.data[3][3] = 1;
        assert_eq!(life.neighbours_count(2, 2), 8);
    }

    #[test]
    fn rule_s2() {
        // ...
        // XXX
        // ...
        let mut life = LifeState::from("...\nXXX\n...");

        let init_state = life.clone();

        // .X.
        // .X.
        // .X.
        let life2 = LifeState::from(".X.\n.X.\n.X.");
        let mut life = life.next().unwrap();
        assert_eq!(life.data[2][2], 1);
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
        let init_state = LifeState::from("...\nXXX\nX..");

        let mut life = init_state.clone();

        // .X.
        // XX.
        // X..
        let life_next = LifeState::from(".X.\nXX.\nX..");
        let mut life = life.next().unwrap();
        assert_eq!(life.data[2][2], 1); // S3
        assert_eq!(life, life_next);

        // XX.
        // XX.
        // XX.
        let life_next = LifeState::from("XX.\nXX.\nXX.");
        let life = life.next().unwrap();
        assert_eq!(life, life_next);
    }

    #[test]
    fn rule_b3() {
        // X..
        // ...
        // X.X
        let init_state = LifeState::from("X..\n...\nX.X");

        let mut life = init_state.clone();
        // ...
        // .X.
        // ...
        let life_next = LifeState::from("...\n.X.\n...");
        let life = life.next().unwrap();
        assert_eq!(life.data[2][2], 1); // B3
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
        let mut life = LifeState::from("...\n.X.\n...");

        let life = life.next().unwrap();
        assert_eq!(life.data[2][2], 0); // L0

        // L1 -> D
        // X..
        // .X.
        // ...
        let mut life = LifeState::from("X..\n.X.\n...");

        let life = life.next().unwrap();
        assert_eq!(life.data[2][2], 0); // L1

        // L4 -> D
        // XXX
        // XX.
        // ...
        let mut life = LifeState::from("XXX\nXX.\n...");

        let life = life.next().unwrap();
        assert_eq!(life.data[2][2], 0); // L4

        // L5 -> D
        // XXX
        // XXX
        // ...
        let mut life = LifeState::from("XXX\nXXX\n...");

        let life = life.next().unwrap();
        assert_eq!(life.data[2][2], 0); // L5

        // L6 -> D
        // XXX
        // XXX
        // X..
        let mut life = LifeState::from("XXX\nXXX\nX..");

        let life = life.next().unwrap();
        assert_eq!(life.data[2][2], 0); // L6

        // L7 -> D
        // XXX
        // XXX
        // XX.
        let mut life = LifeState::from("XXX\nXXX\nXX.");

        let life = life.next().unwrap();
        assert_eq!(life.data[2][2], 0); // L7

        // L8 -> D
        // XXX
        // XXX
        // XXX
        let mut life = LifeState::from("XXX\nXXX\nXXX");

        let life = life.next().unwrap();
        assert_eq!(life.data[2][2], 0); // L8

        // Dead stays dead loop
        // ...
        // ...
        // ...
        let mut init_state = LifeState::from("...\n...\n...");

        for i in 0..9 {
            let x = i % 3 + 1;
            let y = i / 3 + 1;
            if !(x == 2 && y == 2) {
                init_state.data[x][y] = 1;
            }
            let life = init_state.next().unwrap();
            // the cell should remain dead if i != 2 aka neighbours_count is != 3
            if i != 2 {
                assert_eq!(life.data[2][2], 0);
            } else {
                // we can check the rule here, why not
                assert_eq!(life.data[2][2], 1);
            }
        }
    }
}
