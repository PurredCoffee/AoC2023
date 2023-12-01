mod one;

use std::fs::File;
use std::io::BufReader;

trait Day {}
trait Solution<T: Day> {
    fn solve(f: BufReader<File>);
}

pub fn run(id: u32,f: BufReader<File>) {
    match id {
        1 => {
            <() as Solution<one::One>>::solve(f);
        }
        _ => {
            println!("\033[37mDay {} not implemented\033[0m", id);
        }
    }
}