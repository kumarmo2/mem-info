use std::fs;
use std::io;

const MEM_INFO_PATH: &str = "/proc/meminfo";

fn main() -> io::Result<()> {
    let content = fs::read_to_string(MEM_INFO_PATH)?;

    let mut lines = content.split("\n").take(3);
    let total_mem_line = lines.nth(0).expect("not correct file format");
    let mut items = total_mem_line.split(":").map(|a| a.trim());
    let total_mem = items.nth(1).unwrap();

    let total_mem = get_mem(total_mem);
    println!("total mem: {} Gb", total_mem as f32 / (1024 * 1024) as f32);

    // let _ = lines.nth(0).expect("not correct file format");

    let free_mem_line = lines.nth(0).expect("not correct file format");
    let mut items = free_mem_line.split(":").map(|a| a.trim());

    let free_mem = items.nth(1).unwrap();
    let free_mem = get_mem(free_mem);
    println!("num of free mem: {0:.2} Gb", free_mem as f32 / (1024 * 1024) as f32);


    println!("Percentage free: {0:.2}%", (free_mem as f32 / total_mem as f32) * 100 as f32);

    io::Result::Ok(())
}



fn get_mem(mem_str: &str) -> i32 {
    let index = mem_str.find(' ').unwrap();
    let (free_mem, _ ) = mem_str.split_at(index);
    let free_mem = free_mem.parse::<i32>().unwrap();
    free_mem
}
