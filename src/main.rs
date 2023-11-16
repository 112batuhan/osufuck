use std::path::Path;

use osu_beatmap_parser::BeatmapLevel;

fn main() {
    let beatmap_path = Path::new("assets/test.osu");

    let beatmap: BeatmapLevel = BeatmapLevel::open(&beatmap_path).unwrap();

    // Getting all the hit objects
    let objects = beatmap.hit_objects;
    dbg!(objects);
    let str = ">++++++++++>+>+[
        [+++++[>++++++++<-]>.<++++++[>--------<-]+<<<]>.>>[
            [-]<[>+<-]>>[<<+>+>-]<[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-
                [>+<-[>+<-[>+<-[>[-]>+>+<<<-[>+<-]]]]]]]]]]]+>>>
        ]<<<
    ]";
    brainfuck::eval_string(str).unwrap();
}
