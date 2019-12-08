use std::cmp::{max, min};

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn from(s: &str) -> Direction {
        use Direction::*;
        match s {
            "U" => Up,
            "D" => Down,
            "R" => Right,
            "L" => Left,
            _ => panic!("Unrecognized direction {}", s),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance_from_origin(&self) -> u32 {
        self.distance_from(&Point { x: 0, y: 0 })
    }

    fn distance_from(&self, other: &Point) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32
    }
}

#[derive(Debug)]
struct PointWithSteps {
    point: Point,
    steps: u32,
}

#[derive(PartialEq, Debug)]
struct Segment {
    a: Point,
    b: Point,
}

impl Segment {
    fn len(&self) -> u32 {
        (self.a.x - self.b.x).abs() as u32 + (self.a.y - self.b.y).abs() as u32
    }
}

#[derive(PartialEq, Debug)]
struct Wire {
    v: Vec<(Direction, u32)>,
}

impl Wire {
    fn from(s: &str) -> Wire {
        Wire {
            v: s.split(",").map(|s| decode_step(s)).collect(),
        }
    }
}

impl Wire {
    fn segments(&self) -> Vec<(Segment)> {
        let mut current = Point { x: 0, y: 0 };
        let mut segments = vec![];
        for e in &self.v {
            let next = match e.0 {
                Direction::Up => Point {
                    x: current.x,
                    y: current.y + e.1 as i32,
                },
                Direction::Down => Point {
                    x: current.x,
                    y: current.y - e.1 as i32,
                },
                Direction::Right => Point {
                    x: current.x + e.1 as i32,
                    y: current.y,
                },
                Direction::Left => Point {
                    x: current.x - e.1 as i32,
                    y: current.y,
                },
            };
            segments.push(Segment {
                a: current,
                b: next,
            });
            current = next;
        }
        segments
    }
}

/// Do segments intersect? and if so where? We can simplify the logic because everything is vertical/horizontal.
/// We can have multiple intersection if the wires run along
fn segments_intersect(seg1: &Segment, seg2: &Segment) -> Vec<Point> {
    let mut intersects = vec![];
    if max(seg1.a.x, seg1.b.x) < min(seg2.a.x, seg2.b.x)
        || min(seg1.a.x, seg1.b.x) > max(seg2.a.x, seg2.b.x)
    {
        return intersects;
    }
    if max(seg1.a.y, seg1.b.y) < min(seg2.a.y, seg2.b.y)
        || min(seg1.a.y, seg1.b.y) > max(seg2.a.y, seg2.b.y)
    {
        return intersects;
    }

    if seg1.a.x == seg1.b.x && seg2.a.x == seg2.b.x {
        if seg1.a.x != seg2.a.x {
            panic!("Should not happen");
        }
        for y in max(min(seg1.a.y, seg1.b.y), min(seg2.a.y, seg2.b.y))
            ..min(max(seg1.a.y, seg1.b.y), max(seg2.a.y, seg2.b.y))
        {
            intersects.push(Point { x: seg1.a.x, y })
        }
        return intersects;
    }
    if seg1.a.y == seg1.b.y && seg2.a.y == seg2.b.y {
        if seg1.a.y != seg2.a.y {
            panic!("Should not happen");
        }
        for y in max(min(seg1.a.x, seg1.b.x), min(seg2.a.x, seg2.b.x))
            ..min(max(seg1.a.x, seg1.b.x), max(seg2.a.x, seg2.b.x))
        {
            intersects.push(Point { x: seg1.a.y, y })
        }
        return intersects;
    }
    if seg1.a.x == seg1.b.x {
        return vec![Point {
            x: seg1.a.x,
            y: seg2.a.y,
        }];
    } else {
        return vec![Point {
            x: seg2.a.x,
            y: seg1.a.y,
        }];
    }
}

/// Decodes a step
/// ```
/// assert_eq!(decode_step("Da12"), (Direction::Down, 12))
/// ```
fn decode_step(s: &str) -> (Direction, u32) {
    (Direction::from(&s[0..1]), s[1..].parse().unwrap())
}

/// Splits the input in 2 wires
fn split_input(s: &str) -> Vec<Wire> {
    s.split("\n").map(|s| Wire::from(s)).collect()
}

fn crossing_distance(s: &str) -> u32 {
    let wires = split_input(s);
    assert_eq!(wires.len(), 2);
    let mut intersections = vec![];
    for s1 in wires.get(0).unwrap().segments() {
        for s2 in wires.get(1).unwrap().segments() {
            intersections.extend(segments_intersect(&s1, &s2));
        }
    }
    intersections
        .into_iter()
        .filter(|p| *p != Point { x: 0, y: 0 })
        .map(|p| p.distance_from_origin())
        .min()
        .unwrap()
}

fn intersections_with_steps(wire1: &Wire, wire2: &Wire) -> Vec<PointWithSteps> {
    let mut intersections = vec![];
    let mut steps1 = 0;
    for s1 in wire1.segments() {
        let mut steps2 = 0;
        for s2 in wire2.segments() {
            let int = segments_intersect(&s1, &s2);
            for i in int {
                let p = PointWithSteps {
                    point: i,
                    steps: steps1 + steps2 + i.distance_from(&s1.a) + i.distance_from(&s2.a),
                };
                intersections.push(p);
            }
            steps2 += s2.len();
        }
        steps1 += s1.len()
    }
    intersections
}

fn best_intersection(s: &str) -> u32 {
    let wires = split_input(s);
    assert_eq!(wires.len(), 2);
    intersections_with_steps(&wires.get(0).unwrap(), &wires.get(1).unwrap())
        .into_iter()
        .filter(|p| p.point != Point { x: 0, y: 0 })
        .map(|i| i.steps)
        .min()
        .unwrap()
}

pub fn entry_a(wires: String) -> String {
    crossing_distance(wires.as_str()).to_string()
}

pub fn entry_b(wires: String) -> String {
    best_intersection(wires.as_str()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_step() {
        assert_eq!(decode_step("D12"), (Direction::Down, 12))
    }

    #[test]
    fn test_decode_wire() {
        assert_eq!(
            Wire::from("D12,U14,R1"),
            Wire {
                v: vec![
                    (Direction::Down, 12),
                    (Direction::Up, 14),
                    (Direction::Right, 1)
                ]
            }
        )
    }

    #[test]
    fn test_split_input() {
        assert_eq!(
            split_input("U32,U5,L2\nD12,U14,R1"),
            vec![
                Wire {
                    v: vec![
                        (Direction::Up, 32),
                        (Direction::Up, 5),
                        (Direction::Left, 2)
                    ]
                },
                Wire {
                    v: vec![
                        (Direction::Down, 12),
                        (Direction::Up, 14),
                        (Direction::Right, 1)
                    ]
                }
            ]
        );
        assert_eq!(
            split_input(
                "U32,U5,L2
D12,U14,R1"
            ),
            vec![
                Wire {
                    v: vec![
                        (Direction::Up, 32),
                        (Direction::Up, 5),
                        (Direction::Left, 2)
                    ]
                },
                Wire {
                    v: vec![
                        (Direction::Down, 12),
                        (Direction::Up, 14),
                        (Direction::Right, 1)
                    ]
                }
            ]
        )
    }

    #[test]
    fn test_segments() {
        let w = Wire {
            v: vec![
                (Direction::Up, 32),
                (Direction::Up, 5),
                (Direction::Left, 2),
            ],
        };
        let expected = vec![
            Segment {
                a: Point { x: 0, y: 0 },
                b: Point { x: 0, y: 32 },
            },
            Segment {
                a: Point { x: 0, y: 32 },
                b: Point { x: 0, y: 37 },
            },
            Segment {
                a: Point { x: 0, y: 37 },
                b: Point { x: -2, y: 37 },
            },
        ];
        assert_eq!(w.segments(), expected);
    }

    #[test]
    fn test_intersect() {
        let seg1 = Segment {
            a: Point { x: 0, y: 0 },
            b: Point { x: 1, y: 0 },
        };
        let seg2 = Segment {
            a: Point { x: 2, y: 0 },
            b: Point { x: 2, y: 1 },
        };
        assert_eq!(segments_intersect(&seg1, &seg2), vec![]);

        let seg1 = Segment {
            a: Point { x: -5, y: 0 },
            b: Point { x: 5, y: 0 },
        };
        let seg2 = Segment {
            a: Point { x: 0, y: -5 },
            b: Point { x: 0, y: 5 },
        };
        assert_eq!(segments_intersect(&seg1, &seg2), vec![Point { x: 0, y: 0 }]);
    }

    #[test]
    fn official_results_a() {
        assert_eq!(
            crossing_distance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
        assert_eq!(
            crossing_distance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }

    #[test]
    fn official_results_b() {
        assert_eq!(
            best_intersection(
                "R8,U5,L5,D3
U7,R6,D4,L4"
            ),
            30
        );
        assert_eq!(
            best_intersection(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            610
        );
        assert_eq!(
            best_intersection(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
