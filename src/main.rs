use std::env;
use std::fs;
use std::process::Command;
use std::process::Stdio;
use std::time::Instant;

pub mod days;

fn download_input(day: u32, session_cookie: &str) -> Result<(), String> {
    let file_path = format!("inputs/{}.txt", day);

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

        let input_file = format!("inputs/{}.txt", day);
        let input = fs::read_to_string(&input_file)
            .unwrap_or_else(|_| panic!("Failed to read input file: {}", input_file));

        if let Some(day_impl) = days.get(&day) {
            println!("Running Day {}...", day);

            let start = Instant::now();
            let part1_result = day_impl.part1(&input);
            let part1_duration = start.elapsed();
            println!("Day {}, Part 1: {}", day, part1_result);
            println!("Execution time for Part 1: {:.2?}", part1_duration);

            let start = Instant::now();
            let part2_result = day_impl.part2(&input);
            let part2_duration = start.elapsed();
            println!("Day {}, Part 2: {}", day, part2_result);
            println!("Execution time for Part 2: {:.2?}", part2_duration);
        } else {
            eprintln!("Day {} is not implemented yet.", day);
        }
    }
}