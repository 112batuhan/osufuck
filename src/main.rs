use std::path::Path;

use osu_beatmap_parser::BeatmapLevel;
use parse::Command;



mod parse;

fn main() {
    let beatmap_path = Path::new("assets/test.osu");



    let beatmap: BeatmapLevel = BeatmapLevel::open(beatmap_path).unwrap();

    let raw_commands = Command::get_from_osu_objects(&beatmap.hit_objects);
    let bf_string = Command::turn_to_bf_string(raw_commands);

    dbg!(&bf_string);

    brainfuck::eval_string(&bf_string).unwrap();
}
