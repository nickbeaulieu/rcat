use std::fs::*;
use std::io::*;
use std::path::*;
use std::time::*;

struct Flags {
    number: bool,
    squeeze_blank: bool,
    delay: bool,
    verbose: bool,
}

fn usage() {
    print!(concat!(
        "Usage: rcat <option> <file>\n",
        "Concatenate file(s) to standard output.\n",
        "\n",
        "With no <file>, or when <file> is -, read stdin.\n",
        "\n",
        "  -n, --number             number all output lines\n",
        "  -d, --delay              add a short delay between chars\n",
        "  -s, --squeeze-blank      suppress repeated empty output lines\n",
        "  -v, --verbose            display verbose output\n",
        "      --help     display this help and exit\n",
        "      --version  output version information and exit\n",
        "\n",
        "Examples:\n",
        "$ rcat -n ./file   Adds line numbers to all output lines.\n",
        "$ rcat             Copy stdin to stdout.\n"
    ));
}

fn print_buffer(buffer: &mut [u8; 1024], count: usize, flags: &Flags, line_number: &mut i32) {
    let mut last_char: u8 = 10; // start with newline
    let mut counter = 0;
    let start = Instant::now();        

    for idx in 0..count {
        let char = buffer[idx];
        if last_char == 10 && last_char == char {
            counter += 1;
        } else {
            counter = 0;
        }

        if flags.squeeze_blank && counter > 1 {
            continue;
        }

        if last_char == 10 {
            if flags.number {
                print!("{:6}\t", line_number);
            }
            *line_number += 1;
        }

        print!("{}", char as char);
        std::io::stdout().flush().unwrap();
        if flags.delay {
            std::thread::sleep(Duration::from_millis(8));
        }

        last_char = char;
    }

    if flags.verbose && count > 0 {
        let elapsed = start.elapsed();
        println!("rcat: Read {} bytes in {}ms", count, elapsed.as_millis());
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut files: Vec<String> = vec![];
    let mut flags = Flags { number: false, squeeze_blank: false, delay: false, verbose: false};

    for arg in args {
        if !arg.starts_with('-') || arg.len() <= 1 {
            files.push(arg); // must be a file name if it's not an argument!
        } else {
            match arg.as_str() {
                "-n" | "--number" => flags.number = true,
                "-s" | "--squeeze-blank" => flags.squeeze_blank = true,
                "-d" | "--delay" => flags.delay = true,
                "-v" | "--verbose" => flags.verbose = true,
                "-h" | "-?" | "--help" => {
                    usage();
                    return;
                }
                _ => {
                    eprintln!("rcat: invalid option -- '{}'", arg);
                    eprintln!("Try 'rcat --help' for more information.");
                    return;
                }
            }
        }
    }

    let mut buffer = [0u8; 1024];
    let mut line_number = 1;

    if files.len() < 1 {
        let mut stdin = std::io::stdin();
        let byte_count = stdin.read(&mut buffer).unwrap_or_else(|_| {
            eprintln!("rcat: stdin read error");
            0
        });
        print_buffer(&mut buffer, byte_count, &flags, &mut line_number);
    } else {
        for file in files {
            let path  = Path::new(&file);
            if let Ok(mut file) = File::open(path) {
                let byte_count = file.read(&mut buffer).unwrap_or_else(|_| {
                    eprintln!("rcat: {:#?}: read error", path);
                    0
                });
    
                print_buffer(&mut buffer, byte_count, &flags, &mut line_number);
            } else {
                eprintln!("rcat: {:#?}: {}", path, "No such file or directory");
            }
        }
    }
}
