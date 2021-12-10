use colored::*;
use crossterm::{
    cursor::{MoveToColumn, MoveToRow, RestorePosition, SavePosition, Show},
    execute,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::fs;
use std::io::{self, stdout};
use std::thread;
use std::time::Duration;

const MEM_INFO_PATH: &str = "/proc/meminfo";
const NUM_OF_KBS_IN_ONE_GB: f32 = 1048576_f32;

// TODO: currently if the cursor is at the end of the terminal view
// and then u start the program. it outputs multiple lines instead
// of updating the info in the same lines.
fn main() -> io::Result<()> {
    let mut stdout = stdout();
    // let clear_all = Clear(ClearType::All);
    let clear_down = Clear(ClearType::FromCursorDown);
    execute!(stdout, SavePosition)?;
    let n = 4;
    for i in 1..n {
        // Clear the terminal and move to top left before printing to terminal.
        read_mem_info_and_display_info()?;
        thread::sleep(Duration::from_secs(2));

        if i != n - 1 {
            execute!(stdout, RestorePosition, clear_down)?;
        }
    }
    /*
     * TODO: handle sigint for graceful shutdown */

    io::Result::Ok(())
}

fn read_mem_info_and_display_info() -> io::Result<()> {
    let content = fs::read_to_string(MEM_INFO_PATH)?;

    let mut lines = content.split("\n").take(3);
    let total_mem_line = lines.nth(0).expect("not correct file format");
    let mut items = total_mem_line.split(":").map(|a| a.trim());
    let total_mem = items.nth(1).unwrap();

    let total_mem = get_mem(total_mem);
    let free_mem_line = lines.nth(0).expect("not correct file format");
    let mut items = free_mem_line.split(":").map(|a| a.trim());

    let free_mem = items.nth(1).unwrap();
    let free_mem = get_mem(free_mem);

    let used_mem_in_gb = (total_mem - free_mem) / NUM_OF_KBS_IN_ONE_GB;
    let total_mem_in_gb = (total_mem) / NUM_OF_KBS_IN_ONE_GB;

    let output = format!(
        "used: {0:.2}/{1:.2}",
        used_mem_in_gb,
        (total_mem / NUM_OF_KBS_IN_ONE_GB)
    );
    println!(
        "{}",
        output
            .color(get_color_for_used_memory_output(
                used_mem_in_gb / total_mem_in_gb
            ))
            .bold()
    );

    io::Result::Ok(())
}

fn get_color_for_used_memory_output(used_mem_fraction: f32) -> &'static str {
    if used_mem_fraction >= 0.8 {
        return "red";
    }
    if used_mem_fraction >= 0.4 {
        return "yellow";
    }
    "green"
}

fn get_color(free_percentage: f32) -> &'static str {
    if free_percentage <= 20_f32 {
        return "red";
    }
    if free_percentage <= 50_f32 {
        return "yellow";
    }
    "green"
}

fn get_mem(mem_str: &str) -> f32 {
    let index = mem_str.find(' ').unwrap();
    let (free_mem, _) = mem_str.split_at(index);
    let free_mem = free_mem.parse::<f32>().unwrap();
    free_mem
}
