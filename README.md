# MultiSim.rs
**A rust simulation program that simulates things like Game of Life and more.**
---
# Game of Life
**A rust implementation of Conways Game of Life**.

## What is Game of Life?
Conways Game of Life is a simple [cellular automaton](https://en.wikipedia.org/wiki/Cellular_automaton) that consists of just **4 simple rules:**
```
1: Any live cell with fewer than two live neighbours dies, as if by underpopulation.
2: Any live cell with two or three live neighbours lives on to the next generation.
3: Any live cell with more than three live neighbours dies, as if by overpopulation.
4: Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
```
This implementation is written in Rust using the [Macroquad](https://macroquad.rs/) library.

Learn more about Game of Life from the [Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

---
## Installation instructions
You can either download the compiled binaries from the [releases tab](https://github.com/kn-ht/game-of-life/releases)
or **build from source**

### Building from source
Before installing you need to have [Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) and (*optionally*) [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) installed on your system.
**If using git:**
```
git clone https://github.com/kn-ht/game-of-life
cd game-of-life
cargo build --release
```
**if _not_ using git:**
First download the source file as a zip by using the download > as zip button, then
unzip it, open it inside your terminal,
and run
`cargo build --release`

The compiled binary will be in `./target/release`
