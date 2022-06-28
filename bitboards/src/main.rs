use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

type Cluster = u64;
const CLUSTER_SIZE: usize = 62;

/// The game of life state represenation using collection of `Cluster`
/// (`u64`), where each cluster represents state of 62 cells in a row.
///
#[derive(Debug, Clone)]
struct LifeState {
    width: usize,
    height: usize,
    grid: Box<[Cluster]>,
}

impl LifeState {
    fn new(width: usize, height: usize) -> Self {
        let columns = (width + CLUSTER_SIZE - 1) / CLUSTER_SIZE;
        Self {
            width,
            height,
            grid: vec![0; columns * height].into(),
        }
    }
    fn is_alive(&self, x: usize, y: usize) -> bool {
        let i = (x / CLUSTER_SIZE) * self.height + y;
        let offset = (x % CLUSTER_SIZE) + 1;
        ((self.grid[i] >> offset) & 0b1) == 1
    }

    fn set(&mut self, x: usize, y: usize) {
        let i = (x / CLUSTER_SIZE) * self.height + y;
        let offset = (x % CLUSTER_SIZE) + 1;
        self.grid[i] |= 0b1 << offset;
    }

    fn next(&mut self) {
        todo!()
    }
}

impl PartialEq for LifeState {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.grid == other.grid
    }
}

impl From<io::Lines<io::BufReader<File>>> for LifeState {
    fn from(lines: io::Lines<io::BufReader<File>>) -> Self {
        let mut width: Option<usize> = None;
        let mut height = 0;
        let mut data: Vec<Vec<u8>> = Vec::new();
        for line_result in lines {
            match line_result {
                Ok(line) => {
                    height += 1;
                    let line_bools: Vec<u8> =
                        line.chars().map(|c| if c == 'X' { 1 } else { 0 }).collect();
                    if !line_bools.is_empty() {
                        if width.is_some() {
                            assert_eq!(
                                line_bools.len(),
                                width.unwrap(),
                                "Line of different lenght"
                            );
                        } else {
                            width = Some(line_bools.len());
                        }
                        data.push(line_bools);
                    }
                }
                Err(e) => panic!("{}", e),
            }
        }
        let mut life = LifeState::new(width.unwrap(), height);
        for y in 0..height {
            for x in 0..width.unwrap() {
                if data[y][x] == 1 {
                    life.set(x, y);
                }
            }
        }
        life
    }
}

impl From<&str> for LifeState {
    fn from(s: &str) -> Self {
        let lines = s.split('\n');
        let mut width: Option<usize> = None;
        let mut height = 0;
        let mut data: Vec<Vec<u8>> = Vec::new();
        for line in lines {
            height += 1;
            let line_bools: Vec<u8> = line.chars().map(|c| if c == 'X' { 1 } else { 0 }).collect();
            if !line_bools.is_empty() {
                if width.is_some() {
                    assert_eq!(line_bools.len(), width.unwrap(), "Line of different lenght");
                } else {
                    width = Some(line_bools.len());
                }
                data.push(line_bools);
            }
        }
        let mut life = LifeState::new(width.unwrap(), height);
        for y in 0..height {
            for x in 0..width.unwrap() {
                if data[y][x] == 1 {
                    life.set(x, y);
                }
            }
        }
        life
    }
}

impl fmt::Display for LifeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_alive(x, y) {
                    ret.push('X');
                } else {
                    ret.push('.');
                }
            }
            ret.push('\n');
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
    // for _ in 0..iterations.unwrap() {
    //     life.next();
    // }
    // print result
    print!("{}", life);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str() {
        let life = LifeState::from("...\n...\n...");

        // No cells set
        for y in 0..life.height {
            for x in 0..life.width {
                assert!(!life.is_alive(x, y))
            }
        }

        let life = LifeState::from(".X.\n...\nX..");

        assert!(!life.is_alive(0, 0));
        assert!(life.is_alive(1, 0));
        assert!(!life.is_alive(2, 0));
        assert!(!life.is_alive(0, 1));
        assert!(!life.is_alive(1, 1));
        assert!(!life.is_alive(2, 1));
        assert!(life.is_alive(0, 2));
        assert!(!life.is_alive(1, 2));
        assert!(!life.is_alive(2, 2));
    }

    #[test]
    fn rule_s2() {
        // ...
        // XXX
        // ...
        let mut life = LifeState::from("...\nXXX\n...");

        // .X.
        // .X.
        // .X.
        let life2 = LifeState::from(".X.\n.X.\n.X.");
        assert_ne!(life, life2);

        // with next iteration the life should return to previous state
    }

    #[test]
    fn rule_s3() {
        // ...
        // XXX
        // X..
        let life = LifeState::from("...\nXXX\nX..");

        // .X.
        // XX.
        // X..
        let life = LifeState::from(".X.\nXX.\nX..");

        // XX.
        // XX.
        // XX.
        let life = LifeState::from("XX.\nXX.\nXX.");
    }

    #[test]
    fn rule_b3() {
        // X..
        // ...
        // X.X
        let life = LifeState::from("X..\n...\nX.X");

        // ...
        // .X.
        // ...
        let life = LifeState::from("...\n.X.\n...");
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

        // L1 -> D
        // X..
        // .X.
        // ...
        let mut life = LifeState::from("X..\n.X.\n...");

        // L4 -> D
        // XXX
        // XX.
        // ...
        let mut life = LifeState::from("XXX\nXX.\n...");

        // L5 -> D
        // XXX
        // XXX
        // ...
        let mut life = LifeState::from("XXX\nXXX\n...");

        // L6 -> D
        // XXX
        // XXX
        // X..
        let mut life = LifeState::from("XXX\nXXX\nX..");

        // L7 -> D
        // XXX
        // XXX
        // XX.
        let mut life = LifeState::from("XXX\nXXX\nXX.");

        // L8 -> D
        // XXX
        // XXX
        // XXX
        let mut life = LifeState::from("XXX\nXXX\nXXX");

        // Dead stays dead loop
        // ...
        // ...
        // ...
        let mut init_state = LifeState::from("...\n...\n...");

        // for i in 0..9 {
        //     let x = i % 3 + 1;
        //     let y = i / 3 + 1;
        //     if !(x == 2 && y == 2) {
        //         init_state.last[x][y] = 1;
        //     }
        //     let mut life = init_state.clone();
        //     life.next();
        //     // the cell should remain dead if i != 2 aka neighbours_count is != 3
        //     if i != 2 {
        //         assert_eq!(life.last[2][2], 0);
        //     } else {
        //         // we can check the rule here, why not
        //         assert_eq!(life.last[2][2], 1);
        //     }
        // }
    }
}
