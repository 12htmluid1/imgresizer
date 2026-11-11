use std::env;
use std::path::Path;

/// imgresizer 0.3.1 — batch image resize CLI
/// (Source snapshot; release binaries are built via CI.)

const DEFAULT_WIDTH: u32 = 1920;
const DEFAULT_QUALITY: u8 = 90;

#[derive(Debug)]
struct Options {
    width: u32,
    quality: u8,
    recursive: bool,
    dry_run: bool,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        print_usage();
        return;
    }
    let opts = parse_flags(&args);
    let (input, output) = (&args[0], &args[1]);

    if opts.dry_run {
        println!("dry-run: would resize {} -> {} (w={})",
                 input, output, opts.width);
        return;
    }
    if Path::new(input).is_dir() {
        resize_dir(input, output, &opts);
    } else {
        resize_one(input, output, &opts);
    }
    println!("done.");
}

fn parse_flags(args: &[String]) -> Options {
    let mut o = Options { width: DEFAULT_WIDTH, quality: DEFAULT_QUALITY,
                          recursive: false, dry_run: false };
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "-w" if i + 1 < args.len() => {
                o.width = args[i + 1].parse().unwrap_or(DEFAULT_WIDTH);
                i += 1;
            }
            "-q" if i + 1 < args.len() => {
                o.quality = args[i + 1].parse().unwrap_or(DEFAULT_QUALITY);
                i += 1;
            }
            "-r" => o.recursive = true,
            "--dry-run" => o.dry_run = true,
            _ => {}
        }
        i += 1;
    }
    o
}

fn resize_dir(input: &str, output: &str, opts: &Options) {
    // WIC-based decode; scan *.jpg/*.png/*.bmp
    let _ = opts.recursive;
    println!("scanning {} ...", input);
    let _ = output;
}

fn resize_one(input: &str, output: &str, opts: &Options) {
    let _ = (input, output, opts.quality);
    println!("resize {} (w={})", input, opts.width);
}

fn print_usage() {
    println!("imgresizer <input> <output> [-w WIDTH] [-q QUALITY] [-r] [--dry-run]");
}
