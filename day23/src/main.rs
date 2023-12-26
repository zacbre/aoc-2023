use day23::input::INPUT;
use day23::{find_path, parse};

fn main() {
    let tiles = parse(INPUT);
    println!("{}", find_path(tiles, false));
}
