use clap::{Parser, ValueEnum};

use osufuck::{debug, execute};
use rfd::FileDialog;
use std::io;
use std::path::PathBuf;

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
enum Mode {
    DEBUG,
    EXECUTE,
}

/// osufuck! parser.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Mode to run the program in.
    ///
    /// execute: parse and execute the map once
    /// debug: parse and execute the map when .osu file changes.
    #[arg(short, long, value_enum, default_value_t = Mode::EXECUTE)]
    mode: Mode,
    /// The path to the .osu file to parse or watch depending on the mode.
    /// If not provided, program will ask the path with a file picker window.
    #[clap(short, long)]
    path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let osu_file = args.path.unwrap_or_else(|| {
        FileDialog::new()
            .add_filter("beatmap file", &["osu"])
            .pick_file()
            .expect("Failed to pick an .osu file")
    });

    let beatmap_path = osu_file.as_path();

    match args.mode {
        Mode::DEBUG => debug(beatmap_path),
        Mode::EXECUTE => execute(beatmap_path),
    }
    println!("Press enter to close.");
    io::stdin().read_line(&mut String::new()).unwrap();
}
