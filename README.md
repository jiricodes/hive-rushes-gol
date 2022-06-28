# hive-rushes-gol
Small Game of Life challenge.

This was a 48h timed project at Hive coding school, in June 2022. Unfortunately I couldn't participate at the time, but wanted to check it out anyways.

My start time: Jun 27, 2022 19:30
My end time: N/A

Timelog:
- Jun 27 ~2hrs

## Goal
Write working version of [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) and then try to optimize it as much as possible.

In the end there should be at least two versions of the binary present, one optimized and one not, to prove the difference.

## Project description
Instructions
- any language
- no third party libraries
- we assume input file to be valid
- usage `./life initial_state iterations` (binary name can vary)
- print out state after `iterations` from the given `initial_state`
- printout identical to `initial_state`

File format
- text file
- one or more lines
- every line is same length (at least 1) and ends with `\n`
- lines contain only `.` for dead cell and `X` for live cell

Boundaries
- output is the same size as input
- each cell outside boundaries is dead and remains dead

Bonuses
- GUI (can use 3rd party libs)
- Infinite world
- [Rulestring](https://conwaylife.com/wiki/Rulestring)

## My implementations

### [Simple](simple/)
The most basic implementation with no designed optimizations to complete the project according to instructions.

Only optimizations involved were in standard `cargo build --release` [compilation settings](https://doc.rust-lang.org/cargo/reference/profiles.html#release).

real	1m47.226s
user	1m46.649s
sys	0m0.461s

### [opt_01](opt_01/)
According to [flamegraph](https://github.com/flamegraph-rs/flamegraph) the `simple` version spent 93% of the time in [`neighbours_count()`](simple/src/main.rs)[line 25]. So this version tries to iprove that.

real	0m39.539s
user	0m39.333s
sys	0m0.201s

