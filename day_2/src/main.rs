use anyhow::Result;
use std::str::FromStr;

fn main() {
    let ans1 = read_lines::<Direction>("input").unwrap()
        .iter()
        .fold(Location::new(), |acc, next| match next{
            Direction::Forward(dist) => Location {
               distance: acc.distance + dist,
               ..acc
            },
            Direction::Down(depth) => Location {
               depth: acc.depth + depth,
               ..acc
            },
            Direction::Up(depth) => Location {
               depth: acc.depth - depth,
               ..acc
            },
        });
    let ans2 = read_lines::<Direction>("input").unwrap()
        .iter()
        .fold(Location::new(), |acc, next| match next{
            Direction::Forward(dist) => Location {
               distance: acc.distance + dist,
               depth: acc.depth + dist * acc.aim,
               ..acc
            },
            Direction::Down(depth) => Location {
               aim: acc.aim + depth,
               ..acc
            },
            Direction::Up(depth) => Location {
               aim: acc.aim - depth,
               ..acc
            },
        });
    println!("part 1: {:?}", ans1.distance * ans1.depth);
    println!("part 2: {:?}", ans2.distance * ans2.depth);
}

#[derive(Debug)]
enum Direction {
    Forward(u64),
    Down(u64),
    Up(u64),
}
impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((dir, dist)) = s.split_once(" ") {
            let dist: u64 = dist.parse()?;

            Ok(
                match dir {
                    "forward" => Direction::Forward(dist),
                    "down" => Direction::Down(dist),
                    "up" => Direction::Up(dist),
                    _ => panic!("unhandled dir"),
                }
            )
        }
        else {
            Err(anyhow::format_err!("line split err"))
        }
    }
}

#[derive(Debug)]
struct Location {
    distance: u64,
    depth: u64,
    aim: u64,
}
impl Location {
    fn new() -> Location {
        Location {
            distance: 0,
            depth: 0,
            aim: 0,
        }
    }
}

fn read_lines<T>(filename: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(
        std::fs::read_to_string(filename)?
        .lines()
        .filter_map(|item| item.parse::<T>().ok())
        .collect()
    ) 
}
