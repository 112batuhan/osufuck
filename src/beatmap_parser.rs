use itertools::Itertools;
use osu_beatmap_parser::section::hit_objects::{HitObject, HitObjectType, SliderPoint};
use osu_beatmap_parser::section::CommaListOf;

#[derive(PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
/// Needed because [`SliderPoint`] does not implement clone.
impl From<&SliderPoint> for Point {
    fn from(slider_point: &SliderPoint) -> Self {
        Self {
            x: slider_point.x,
            y: slider_point.y,
        }
    }
}

pub enum Orientation {
    CLOCKWISE,
    COUNTERCLOCKWISE,
    COLINEAR,
}

impl Orientation {
    pub fn orientation(point_vec: &[Point]) -> Self {
        let mut sum = 0.0;
        let mut v1 = &point_vec[point_vec.len() - 1];

        for i in 0..point_vec.len() {
            let v2 = &point_vec[i];
            sum += (v2.x as f32 - v1.x as f32) * (v2.y as f32 + v1.y as f32);
            v1 = v2;
        }

        match sum {
            _ if sum > 0.0 => Self::CLOCKWISE,
            _ if sum < 0.0 => Self::COUNTERCLOCKWISE,
            _ => Self::COLINEAR,
        }
    }
    pub fn as_bf_command(self) -> &'static str {
        match self {
            Self::CLOCKWISE => "[",
            Self::COUNTERCLOCKWISE => "]",
            Self::COLINEAR => "",
        }
    }
}

pub enum SliderDirection {
    LEFT,
    RIGHT,
}

impl SliderDirection {
    pub fn direction(start: &Point, end: &Point) -> Self {
        if start.x < end.x {
            Self::RIGHT
        } else {
            Self::LEFT
        }
    }
    pub fn as_bf_command(self) -> &'static str {
        match self {
            Self::LEFT => "<",
            Self::RIGHT => ">",
        }
    }
}

pub enum NotePosition {
    UP,
    DOWN,
}

impl NotePosition {
    pub fn position(p: Point) -> Self {
        if p.y > 192 {
            Self::DOWN
        } else {
            Self::UP
        }
    }
    pub fn as_bf_command(self) -> &'static str {
        match self {
            Self::UP => "+",
            Self::DOWN => "-",
        }
    }
}

pub enum Command {
    INCREMENT(NotePosition),
    CURSOR(SliderDirection),
    LOOP(Orientation),
    OUTPUT,
}

impl Command {
    pub fn get_from_osu_objects(objects: &CommaListOf<HitObject>) -> Vec<Command> {
        objects
            .iter()
            .filter_map(|object| {
                let start_point = Point {
                    x: object.x,
                    y: object.y,
                };

                match &object.object_params {
                    HitObjectType::HitCircle => {
                        Some(Command::INCREMENT(NotePosition::position(start_point)))
                    }
                    HitObjectType::Slider(slider_params) => {
                        let curve_points: Vec<Point> = slider_params
                            .curve_points
                            .iter()
                            .map(|point| point.into())
                            .dedup()
                            .collect();

                        if curve_points.len() <= 2 {
                            // curve points should always have at least one point. So unwrap is safe. (hopefully)
                            let end_point = curve_points.last().unwrap();
                            Some(Command::CURSOR(SliderDirection::direction(
                                &start_point,
                                end_point,
                            )))
                        } else {
                            Some(Command::LOOP(Orientation::orientation(&curve_points)))
                        }
                    }

                    HitObjectType::Spinner(_) => Some(Command::OUTPUT),
                    _ => None,
                }
            })
            .collect()
    }

    pub fn turn_to_bf_string(commands: Vec<Self>) -> String {
        let mut bf_code = String::new();
        for command in commands {
            match command {
                Command::INCREMENT(note_position) => {
                    bf_code.push_str(note_position.as_bf_command());
                }
                Command::CURSOR(slider_direction) => {
                    bf_code.push_str(slider_direction.as_bf_command());
                }
                Command::LOOP(orientation) => {
                    bf_code.push_str(orientation.as_bf_command());
                }
                Command::OUTPUT => {
                    bf_code.push_str(".");
                }
            }
        }
        bf_code
    }
}
