use std::fs;
use std::io::{self, Result};

const MEM_INFO_PATH: &str = "/proc/meminfo";

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let content = fs::read_to_string(MEM_INFO_PATH)?;

    let lines: Vec<&str> = content.split("\n").take(3).collect();
    let n = lines.len();
    if n < 3 {
        panic!("not correct file format");
    }

    let total_mem_line = lines[0];
    let items: Vec<&str> = total_mem_line.split(":").map(|a| a.trim()).collect();
    let total_mem = items[1];
    println!("total mem: {}", total_mem);

    let free_mem_line = lines[2];
    let items: Vec<&str> = free_mem_line.split(":").map(|a| a.trim()).collect();
    let free_mem = items[1];
    println!("free mem: {}", free_mem);

    /*
    for (index, line) in lines.enumerate() {
        println!("line_no: {} starts", index);
        let items: Vec<&str> = line.split(":").map(|a| a.trim()).collect();
        for item in items {
            println!("item: {}", item);
        }
        println!("line_no: {} ends", index);
    }
    */

    io::Result::Ok(())
}
