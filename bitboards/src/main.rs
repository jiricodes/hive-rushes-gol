//! Original creator [exrok](https://github.com/exrok), no license specified.
//! reimplemented just for testing and learning purposes

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

    /// computes the generation of the grid in place.
    pub fn tick(&mut self) {
        /// computes the generation of column. Assumes that the most and least significant
        /// bits of the clusters store the state of the adjacent cells.
        fn tick_column(column: &mut [Cluster]) {
            fn tick_cluster(cluster: &mut Cluster, above: Cluster, below: Cluster) {
                let bit_sum = |a, b, c| (a ^ b ^ c, a & b | a & c | b & c);
                let (ix, iy) = bit_sum(above, *cluster, below);
                let (ax, ay) = bit_sum(ix << 1, above ^ below, ix >> 1);
                let (bx, by) = bit_sum(iy << 1, above & below, iy >> 1);
                *cluster |= ax; // three (odd_total /w the condition below)
                *cluster &= (ay ^ bx) & !by; // two_or_three_mod4 & !more_than_three
            }

            let mut clusters = column.iter_mut();
            let mut curr = if let Some(c) = clusters.next() {
                c
            } else {
                return;
            };
            let mut above = 0;

            for below in clusters {
                let tmp = *curr;
                tick_cluster(&mut curr, above, *below);
                above = tmp;
                curr = below;
            }
            tick_cluster(&mut curr, above, 0);
        }

        let edge_mask = 0x8000_0000_0000_0001;
        //tail_mask is used to zero extra width in the last rowsumn
        let tail_width = (self.width + CLUSTER_SIZE - 1) % CLUSTER_SIZE + 1;
        let tail_mask = edge_mask | (!1u64 << tail_width);
        let mut columns = self.grid.chunks_exact_mut(self.height);
        let mut prev = columns.next().unwrap();

        // Store the next and prev cell of the adjacent clusters of each column into
        // the temporary cells in each cluster. Once we have set&extracted the outer
        // cells of each column we progress to the next state w/ tick_column.
        if let Some(mut curr) = columns.next() {
            for (first, second) in prev.iter_mut().zip(curr.iter()) {
                *first ^= ((second << CLUSTER_SIZE) ^ *first) & edge_mask;
            }

            for next in columns {
                for ((left, mid), right) in prev.iter().zip(curr.iter_mut()).zip(next.iter()) {
                    *mid ^= (((left >> CLUSTER_SIZE) | (right << CLUSTER_SIZE)) ^ *mid) & edge_mask
                }
                tick_column(prev);
                prev = curr;
                curr = next;
            }

            for (left, last) in prev.iter().zip(curr.iter_mut()) {
                *last ^= ((left >> CLUSTER_SIZE) ^ *last) & tail_mask;
            }
            tick_column(curr);
        } else {
            for f in prev.iter_mut() {
                //Update bounds on the single column
                *f &= !tail_mask;
            }
        }
        tick_column(prev);
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
    for _ in 0..iterations.unwrap() {
        life.tick();
    }
    // print result
    // print!("{}", life);
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
    fn setter() {
        let mut life = LifeState::new(5, 5);
        life.set(2, 2);
        assert!(life.is_alive(2, 2));
        assert!(!life.is_alive(1, 2));
    }

    #[test]
    fn rule_s2() {
        // ...
        // XXX
        // ...
        let init_life = LifeState::from("...\nXXX\n...");
        let mut life = init_life.clone();

        // .X.
        // .X.
        // .X.
        let life2 = LifeState::from(".X.\n.X.\n.X.");
        assert_ne!(life, life2);
        life.tick();
        println!("{}", life);

        assert!(life.is_alive(1, 1));
        assert_eq!(life, life2);

        // with next iteration the life should return to previous state
        life.tick();
        assert_eq!(life, init_life);
    }

    #[test]
    fn rule_s3() {
        // ...
        // XXX
        // X..
        let mut life = LifeState::from("...\nXXX\nX..");

        // .X.
        // XX.
        // X..
        let life2 = LifeState::from(".X.\nXX.\nX..");
        life.tick();
        assert!(life.is_alive(1, 1));
        assert_eq!(life, life2);

        // XX.
        // XX.
        // XX.
        let life3 = LifeState::from("XX.\nXX.\nXX.");
        life.tick();
        assert!(life.is_alive(1, 1));
        assert_eq!(life, life3);
    }

    #[test]
    fn rule_b3() {
        // X..
        // ...
        // X.X
        let mut life = LifeState::from("X..\n...\nX.X");

        // ...
        // .X.
        // ...
        let life2 = LifeState::from("...\n.X.\n...");
        life.tick();
        assert!(life.is_alive(1, 1));
        assert_eq!(life, life2);
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
        life.tick();
        assert!(!life.is_alive(1, 1));

        // L1 -> D
        // X..
        // .X.
        // ...
        let mut life = LifeState::from("X..\n.X.\n...");
        life.tick();
        assert!(!life.is_alive(1, 1));

        // L4 -> D
        // XXX
        // XX.
        // ...
        let mut life = LifeState::from("XXX\nXX.\n...");
        life.tick();
        assert!(!life.is_alive(1, 1));

        // L5 -> D
        // XXX
        // XXX
        // ...
        let mut life = LifeState::from("XXX\nXXX\n...");
        life.tick();
        assert!(!life.is_alive(1, 1));

        // L6 -> D
        // XXX
        // XXX
        // X..
        let mut life = LifeState::from("XXX\nXXX\nX..");
        life.tick();
        assert!(!life.is_alive(1, 1));

        // L7 -> D
        // XXX
        // XXX
        // XX.
        let mut life = LifeState::from("XXX\nXXX\nXX.");
        life.tick();
        assert!(!life.is_alive(1, 1));

        // L8 -> D
        // XXX
        // XXX
        // XXX
        let mut life = LifeState::from("XXX\nXXX\nXXX");
        life.tick();
        assert!(!life.is_alive(1, 1));

        // Dead stays dead loop
        // ...
        // ...
        // ...
        let mut init_state = LifeState::from("...\n...\n...");

        for i in 0..9 {
            let x = i % 3;
            let y = i / 3;
            if !(x == 1 && y == 1) {
                init_state.set(x, y);
            }
            println!("{}", init_state);
            let mut life = init_state.clone();
            life.tick();
            // the cell should remain dead if i != 2 aka neighbours_count is != 3
            if i != 2 {
                assert!(!life.is_alive(1, 1));
            } else {
                // we can check the rule here, why not
                assert!(life.is_alive(1, 1));
            }
        }
    }
}
