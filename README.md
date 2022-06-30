# hive-rushes-gol
Small Game of Life challenge.

This was a 48h timed project at Hive coding school, in June 2022. Unfortunately I couldn't participate at the time, but wanted to check it out anyways.


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


### [opt_01](opt_01/)
According to [flamegraph](https://github.com/flamegraph-rs/flamegraph) the `simple` version spent 93% of the time in [`neighbours_count()`](simple/src/main.rs)[line 25]. So this version tries to iprove that.

The improvement was achieved by padding the board with always false "frame" and using 2D array (1D would work too, but the benefits are doubtful. Maybe for future version). This way we can ommit boundary checking and simply unroll the 'neighbours_count()' loop. Additionally, the field representation was changed to `u8` per cell from `bool` in order to save on some casting (this change is speculative whether it brought any improvements).

### [opt_02](opt_02/)
Double buffer version that seemed to make little to no difference on my machine.

### [bitboards](bitboards/)
Implementation created by [exrok](https://github.com/exrok) and included here only for educational purposes.
So blazing fast! Bitboards and bit twiddling like this is increadibly smart.