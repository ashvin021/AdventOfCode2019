#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

pub mod intcode;

use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Seek, Write},
    net::TcpStream,
    path::Path,
};

use native_tls::TlsConnector;

#[macro_export]
macro_rules! timed_main {
    ($a: expr, $($body:tt)+) => {
    use num_format::{Locale, ToFormattedString};

        fn main() {
            let args: Vec<String> = std::env::args().collect();

            if args.len() > 2 {
                println!("Usage: <day> <number of iterations>");
                std::process::exit(1);
            }

            let loops = if args.len() == 1 {
                $a
            } else if let Ok(num) = args[1].parse::<u128>() {
                num
            } else {
                println!("Invalid number of iterations, continuing with default..");
                println!("Usage: <day> <number of iterations>");
                $a
            };

            let start = std::time::Instant::now();
            for i in 0..loops {
                let (p1, p2) = { $($body)+ };

                if i == 0 {
                    println!("Part 1 answer: {}", p1);
                    println!("Part 2 answer: {}", p2);
                }
            }
            let time = start.elapsed().as_nanos();
            let locale = Locale::en_GB;
            println!("Total time for {} iterations: {}ns", loops, time.to_formatted_string(&locale));
            println!("Average time for {} iterations: {}ns", loops, (time / loops).to_formatted_string(&locale));
        }
    };
}

// Functions for fetching and caching input, borrowed from BeniotZugmeyer/RustyAdventOfCode

fn fetch_aoc(path: &str) -> impl Iterator<Item = String> {
    let session = std::env::var("AOC_SESSION").expect("AOC_SESSION variable is missing");
    let connector = TlsConnector::new().expect("Couldn't create a TLS connector");
    let stream =
        TcpStream::connect("adventofcode.com:443").expect("Failed to connect to the server");
    let mut stream = connector
        .connect("adventofcode.com", stream)
        .expect("Couldn't connect to server with TLS");

    let request = &format!(
        "\
         GET {} HTTP/1.0\r\n\
         Cookie: session={}\r\n\
         \r\n\
         ",
        path, session
    )
    .into_bytes();

    stream.write_all(request).unwrap();

    let reader = BufReader::new(stream);
    let mut lines = reader.lines().filter_map(Result::ok);

    let status = lines.next().expect("Empty response from the server");

    if status.ends_with("404 Not Found") {
        panic!("Got a 404 for {}", path)
    }

    lines.skip_while(|line| !line.is_empty()).skip(1)
}

fn from_cache<T: FnOnce() -> Option<Vec<String>>>(
    name: &str,
    factory: T,
) -> Option<impl Iterator<Item = String>> {
    let cache_dir = Path::new("../cache");
    fs::create_dir_all(cache_dir).expect("Failed to create cache dir");

    let file_cache_path = cache_dir.join(name);
    let file = match File::open(&file_cache_path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            if let Some(result) = factory() {
                // Populate cache
                let mut file = fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&file_cache_path)
                    .expect("Failed to open cache file for writing");
                file.write_all(result.join("\n").as_bytes())
                    .expect("Failed to write cache");
                file.seek(io::SeekFrom::Start(0))
                    .expect("Failed to seek file cache");
                file
            } else {
                return None;
            }
        }
        Err(error) => panic!("Failed to open cache file: {}", error),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    Some(reader.lines().filter_map(Result::ok))
}

pub fn get_input(day: u8) -> impl Iterator<Item = String> {
    from_cache(&format!("day_{:02}_input", day), move || {
        Some(fetch_aoc(&format!("/2019/day/{}/input", day)).collect())
    })
    .unwrap()
}
