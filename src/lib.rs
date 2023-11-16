use crate::beatmap_parser::Command;
use notify::{Config, RecommendedWatcher, Watcher};
use osu_beatmap_parser::BeatmapLevel;
use std::path::Path;

mod beatmap_parser;

pub fn execute(beatmap_path: &Path) {
    let beatmap_result = BeatmapLevel::open(beatmap_path);

    let beatmap = match beatmap_result {
        Ok(beatmap) => beatmap,
        Err(err) => {
            println!("Failed to parse beatmap: {}", err);
            return;
        }
    };

    let raw_commands = Command::get_from_osu_objects(&beatmap.hit_objects);
    let bf_string = Command::turn_to_bf_string(raw_commands);

    println!("Parsed bf string: {}", bf_string);

    brainfuck::eval_string(&bf_string).unwrap_or_else(|err| println!("Evaluation error: {}", err));
}

pub fn debug(beatmap_path: &Path) {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher =
        RecommendedWatcher::new(tx, Config::default()).expect("Failed to create file watcher");

    watcher
        .watch(beatmap_path, notify::RecursiveMode::NonRecursive)
        .expect("Failed to add file to watcher");

    println!("Debug mod has started!");

    execute(beatmap_path);

    for res in rx {
        match res {
            Ok(event) => {
                if let notify::EventKind::Modify(_) = event.kind {
                    println!("Change detected in file. Executing again.");
                    execute(beatmap_path);
                    println!()
                }
            }
            Err(error) => {
                println!("File watch error: {}", error);
                break;
            }
        }
    }
}
