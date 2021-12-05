use std::fs;
use std::io;
 use colored::*;

const MEM_INFO_PATH: &str = "/proc/meminfo";
const NUM_OF_KBS_IN_ONE_GB: f32 = 1048576_f32;

fn main() -> io::Result<()> {
    let content = fs::read_to_string(MEM_INFO_PATH)?;

    let mut lines = content.split("\n").take(3);
    let total_mem_line = lines.nth(0).expect("not correct file format");
    let mut items = total_mem_line.split(":").map(|a| a.trim());
    let total_mem = items.nth(1).unwrap();

    let total_mem = get_mem(total_mem);
    let output = format!("total mem: {} Gb", total_mem / NUM_OF_KBS_IN_ONE_GB);
    println!("{}", output.color("blue").bold());

    let free_mem_line = lines.nth(0).expect("not correct file format");
    let mut items = free_mem_line.split(":").map(|a| a.trim());

    let free_mem = items.nth(1).unwrap();
    let free_mem = get_mem(free_mem);
    let output = format!("free mem: {0:.2} Gb", free_mem / NUM_OF_KBS_IN_ONE_GB);
    println!("{}", output.color("purple").bold());


    let percentage = (free_mem / total_mem ) * 100_f32;
    let output = format!("Percentage free: {0:.2}%", percentage);
    println!("{}", output.color(get_color(percentage)).bold());

    io::Result::Ok(())
}

fn get_color(free_percentage: f32) -> &'static str {
    if free_percentage <= 20_f32 {
        return "red"
    }
    if free_percentage <= 50_f32 {
        return "yellow";
    }
    "green"
}




fn get_mem(mem_str: &str) -> f32 {
    let index = mem_str.find(' ').unwrap();
    let (free_mem, _ ) = mem_str.split_at(index);
    let free_mem = free_mem.parse::<f32>().unwrap();
    free_mem
}
