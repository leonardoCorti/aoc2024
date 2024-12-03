use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::time::Instant;

use colored::Colorize;

use aoc2024::days;

fn download_input(day: u32, session_cookie: &str) -> Result<(), String> {
    let file_path = format!("inputs/{:02}.txt", day);

    if !Path::new("inputs").exists() {
        fs::create_dir("inputs")
            .expect("couldn't create inputs directory");
    }

    if fs::metadata(&file_path).is_ok() {
        println!("File for day {} already exists.", day);
        return Ok(());
    }

    let url = format!("https://adventofcode.com/2024/day/{}/input", day);
    let status = Command::new("curl")
        .arg("-o")
        .arg(&file_path)
        .arg("-H")
        .arg(format!("Cookie: session={}", session_cookie))
        .arg(&url)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match status {
        Ok(status) if status.success() => {
            if let Ok(contents) = fs::read_to_string(&file_path) {
                if contents.contains("Please don't repeatedly request this endpoint before it unlocks!") {
                    fs::remove_file(&file_path).unwrap_or_else(|err| {
                        eprintln!("Failed to remove invalid input file: {}", err);
                    });
                    eprintln!(
                        "Day {} input is not yet available. Try again after the puzzle unlocks.",
                        day
                    );
                    std::process::exit(1);
                }
            }
            println!("Successfully downloaded input for day {}.", day);
            Ok(())
        }
        _ => Err(format!("Failed to download input for day {}.", day)),
    }
}

fn main() {
    println!();
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: aoc2024 <day>...");
        std::process::exit(1);
    }

    let session_cookie = match env::var("AOC_SESSION_COOKIE") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Please set the AOC_SESSION_COOKIE environment variable.");
            std::process::exit(1);
        }
    };

    let days = days::get_days();

    for arg in args {
        let day: u32 = arg.parse().expect("Day should be a valid number");

        if let Err(e) = download_input(day, &session_cookie) {
            eprintln!("{}", e);
            continue;
        }

        let input_file = format!("inputs/{:02}.txt", day);
        let input = fs::read_to_string(&input_file)
            .unwrap_or_else(|_| panic!("Failed to read input file: {}", input_file));

        if let Some(day_impl) = days.get(&day) {
            println!("Running Day {}...", 
                day.to_string().blue());

            let start = Instant::now();
            let part1_result = day_impl.part1(&input);
            let part1_duration = start.elapsed();
            let part1_duration_string = format!("{:.2?}", part1_duration);
            println!("\tDay {}, Part {}: {}, execution time {}",
                day.to_string().blue(),
                "1".blue(),
                part1_result.to_string().red(),
                part1_duration_string.purple());

            let start = Instant::now();
            let part2_result = day_impl.part2(&input);
            let part2_duration = start.elapsed();
            let part2_duration_string = format!("{:.2?}", part2_duration);
            println!("\tDay {}, Part {}: {}, execution time {}",
                day.to_string().blue(),
                "2".blue(),
                part2_result.to_string().red(),
                part2_duration_string.purple());

            println!();
        } else {
            eprintln!("Day {} is not implemented yet.", day);
        }
    }
}
