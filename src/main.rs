use std::fs;
use std::io::{self, Result};

const MEM_INFO_PATH: &str = "/proc/meminfo";

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let content = fs::read_to_string(MEM_INFO_PATH)?;

    let lines: Vec<_> = content.split("\n").take(3).collect();
    let n = lines.len();
    if n < 3 {
        panic!("not correct file format");
    }
    let total_mem_line = lines[0];
    let mut items = total_mem_line.split(":").map(|a| a.trim());
    let total_mem = items.nth(1).unwrap();
    println!("total mem: {}", total_mem);

    let free_mem_line = lines[2];
    let mut items = free_mem_line.split(":").map(|a| a.trim());

    let free_mem = items.nth(1).unwrap();
    println!("free mem: {}", free_mem);

    io::Result::Ok(())
}
