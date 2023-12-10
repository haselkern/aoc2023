use aoc2023::{assert_example, Vec2};
use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/10");

fn main() {
    assert_example!(part1, "10-test-1", 4);
    assert_example!(part1, "10-test-2", 8);
    println!("Part 1: {}", part1(INPUT));

    assert_example!(part2, "10-test-3", 4);
    assert_example!(part2, "10-test-4", 4);
    assert_example!(part2, "10-test-5", 8);
    assert_example!(part2, "10-test-6", 10);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    parse(input).length() / 2
}

fn part2(input: &str) -> usize {
    parse(input).count_empty_tiles_inside()
}

struct Maze {
    tiles: HashMap<Vec2<i64>, Tile>,
    start: Vec2<i64>,
}

impl Maze {
    /// Find all tile positions that form a closed loop.
    fn pipe_loop(&self) -> HashMap<Vec2<i64>, PipeSegmentDirection> {
        let mut pipes = HashMap::new();
        let mut position = self.start;
        let mut came_from = None;

        loop {
            let current_tile = *self.tiles.get(&position).unwrap();

            match current_tile {
                Tile::Empty => panic!("followed the maze to an empty tile?! {position:?}"),
                Tile::Start if !pipes.is_empty() => {
                    break;
                }
                Tile::Start => {
                    let (to, from) = self.start_connections();
                    let segment_direction = PipeSegmentDirection::new(from, to);
                    pipes.insert(position, segment_direction);
                    position += to.to_vec2();
                    came_from = Some(to.invert());
                }
                Tile::Pipe(pipe) => {
                    let from = came_from.expect("came_from");
                    let to = pipe.other_side(from).expect("other side");
                    let segment_direction = PipeSegmentDirection::new(from, to);
                    pipes.insert(position, segment_direction);
                    position += to.to_vec2();
                    came_from = Some(to.invert());
                }
            }
        }

        pipes
    }

    /// Count the number of tiles inside the polygon described by the pipe loop
    /// using the [winding number algorithm](https://en.wikipedia.org/wiki/Point_in_polygon#Winding_number_algorithm).
    fn count_empty_tiles_inside(&self) -> usize {
        let max_dimensions = self.tiles.keys().fold(Vec2::<i64>::default(), |a, b| Vec2 {
            x: a.x.max(b.x),
            y: a.y.max(b.y),
        });
        let pipe_loop = self.pipe_loop();

        let mut tiles_inside = 0;

        for y in 0..=max_dimensions.y {
            let mut winding = 0;
            // Use this to prevent consecutive runs of changes in winding number.
            let mut last_change = None;

            for x in 0..=max_dimensions.x {
                let position = Vec2::new(x, y);
                let pipe_segment = pipe_loop.get(&position).copied();

                if let Some(pipe_segment) = pipe_segment {
                    if Some(pipe_segment) == last_change {
                        // We already did this change
                    } else {
                        match pipe_segment.winding() {
                            None => {}
                            Some(w) => {
                                winding += w;
                                last_change = Some(pipe_segment);
                            }
                        }
                    }
                } else {
                    last_change = None;
                    if winding != 0 {
                        tiles_inside += 1;
                    }
                }
            }
        }

        tiles_inside
    }

    fn length(&self) -> usize {
        self.pipe_loop().len()
    }

    /// Returns the pipes that are connected to the start.
    fn start_connections(&self) -> (Direction, Direction) {
        let neighbors = Direction::all().into_iter().flat_map(|dir| {
            let pos = dir.to_vec2() + self.start;
            self.tiles.get(&pos).map(|&tile| (dir, tile))
        });
        let connected_neighbors = neighbors.filter(|(dir, tile)| match tile {
            Tile::Empty => false,
            Tile::Start => panic!("move_from_start has start as neighbor?!"),
            Tile::Pipe(pipe) => pipe.other_side(dir.invert()).is_some(),
        });
        let mut connected = connected_neighbors.map(|(dir, _tile)| dir);

        let a = connected.next().expect("first start connection");
        let b = connected.next().expect("second start connection");
        assert_eq!(connected.next(), None, "Got a third start connection?!");

        (a, b)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum PipeSegmentDirection {
    Up,
    Down,
    Other,
}

impl PipeSegmentDirection {
    fn new(from: Direction, to: Direction) -> Self {
        if from == Direction::North || to == Direction::South {
            Self::Down
        } else if from == Direction::South || to == Direction::North {
            Self::Up
        } else {
            Self::Other
        }
    }

    fn winding(self) -> Option<i64> {
        match self {
            PipeSegmentDirection::Up => Some(1),
            PipeSegmentDirection::Down => Some(-1),
            PipeSegmentDirection::Other => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_vec2(self) -> Vec2<i64> {
        match self {
            Direction::North => Vec2::new(0, -1),
            Direction::East => Vec2::new(1, 0),
            Direction::South => Vec2::new(0, 1),
            Direction::West => Vec2::new(-1, 0),
        }
    }

    fn all() -> [Self; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    fn invert(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Pipe {
    a: Direction,
    b: Direction,
}

impl Pipe {
    /// Get the other side of the pipe given one side.
    /// Returns None if there is no connection from the given side.
    fn other_side(&self, from: Direction) -> Option<Direction> {
        if self.a == from {
            Some(self.b)
        } else if self.b == from {
            Some(self.a)
        } else {
            None
        }
    }

    fn new(a: Direction, b: Direction) -> Self {
        Self { a, b }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Start,
    Pipe(Pipe),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Pipe(Pipe::new(Direction::North, Direction::South)),
            '-' => Self::Pipe(Pipe::new(Direction::East, Direction::West)),
            'L' => Self::Pipe(Pipe::new(Direction::North, Direction::East)),
            'J' => Self::Pipe(Pipe::new(Direction::North, Direction::West)),
            '7' => Self::Pipe(Pipe::new(Direction::West, Direction::South)),
            'F' => Self::Pipe(Pipe::new(Direction::South, Direction::East)),
            '.' => Self::Empty,
            'S' => Self::Start,
            other => panic!("unknown tile '{other}'"),
        }
    }
}

fn parse(input: &str) -> Maze {
    let mut tiles = HashMap::new();
    let mut start = None;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = c.into();
            let position = Vec2::new(x as i64, y as i64);

            if tile == Tile::Start {
                start = Some(position);
            }

            tiles.insert(position, tile);
        }
    }

    let start = start.expect("start position");

    Maze { tiles, start }
}
