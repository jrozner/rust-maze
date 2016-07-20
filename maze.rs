use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::fmt;

fn main() {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);

    let maze_raw: Vec<Vec<char>> = f.lines()
        .map({
            |line| match line {
                Ok(line) => line.chars().collect(),
                Err(_) => vec![],
            }
        })
        .collect();

    let mut maze = Maze::new();
    for (y, row) in maze_raw.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            match *col {
                'S' => maze.set_start(Coordinate::new(y, x)),
                ' ' | 'E' => {
                    let coordinate = Coordinate::new(y, x);

                    if *col == 'E' {
                        maze.set_end(coordinate)
                    }

                    // check above
                    let above = maze_raw[y-1][x];
                    if above == ' ' || above == 'S' || above == 'E' {
                        maze.add_connection(Coordinate::new(y-1, x), coordinate);
                    }

                    // check right
                    let right = maze_raw[y][x+1];
                    if right == ' ' || right == 'S' || right == 'E' {
                        maze.add_connection(Coordinate::new(y, x+1), coordinate);
                    }

                    // check below
                    if y + 1 < row.len() {
                        let below = maze_raw[y+1][x];
                        if below == ' ' || below == 'S' || below == 'E' {
                            maze.add_connection(Coordinate::new(y+1, x), coordinate);
                        }
                    }

                    // check left
                    let left = maze_raw[y][x - 1];
                    if left == ' ' || left == 'S' || left == 'E' {
                        maze.add_connection(Coordinate::new(y, x-1), coordinate);
                    }
                }
                _ => {}
            }
        }
    }

    match maze.solve() {
        Some(solution) => {
            for m in solution {
                println!("{}", m)
            }
        },
        None => println!("No solution"),
    }
}

struct Maze {
    start: Option<Coordinate>,
    end: Option<Coordinate>,
    connections: HashMap<Coordinate,HashSet<Coordinate>>,
}

impl Maze {
    fn new() -> Maze {
        return Maze{
            start: None,
            end: None,
            connections: HashMap::new(),
        }
    }

    fn set_start(&mut self, start: Coordinate) {
        self.start = Some(start);
    }

    fn set_end(&mut self, end: Coordinate) {
        self.end = Some(end);
    }

    fn add_connection(&mut self, from: Coordinate, to: Coordinate) {
        let f = self.connections.entry(from).or_insert(HashSet::new());
        f.insert(to);
    }

    fn solve(self) -> Option<LinkedList<Coordinate>> {
        if self.start == None {
            return None
        }

        let start = self.start.unwrap();

        if self.end == None {
            return None
        }

        let end = self.end.unwrap();

        let mut open: HashSet<Coordinate> = HashSet::new();
        let mut closed: HashSet<Coordinate> = HashSet::new();
        let mut g_score = HashMap::new();
        let mut f_score = HashMap::new();
        let mut connections = HashMap::new();

        g_score.insert(start, 0);
        f_score.insert(start, ((start.x + start.y) as i64 - (end.x + end.y) as i64).abs());

        open.insert(start);

        while open.len() > 0 {
            let cheapest = *open.iter().min_by_key(|coordinate| f_score.get(coordinate).unwrap()).unwrap();
            if cheapest == end {
                return Some(reconstruct_path(connections, cheapest));
            }

            open.remove(&cheapest);
            closed.insert(cheapest);
            let neighbors = self.connections.get(&cheapest).unwrap();
            for neighbor in neighbors {
                if closed.contains(neighbor) {
                    continue;
                }

                let score = g_score.get(&cheapest).unwrap() + ((neighbor.x + neighbor.y) as i64 - (cheapest.x + cheapest.y) as i64).abs();
                if !open.contains(neighbor) {
                    open.insert(*neighbor);
                } else if score >= g_score[neighbor] {
                    continue;
                }

                connections.insert(*neighbor, cheapest);
                g_score.insert(*neighbor, score);
                f_score.insert(*neighbor, score + ((neighbor.x + neighbor.y) as i64 - (end.x + end.y) as i64).abs());
            }
        }

        None
    }
}
fn reconstruct_path(connections: HashMap<Coordinate,Coordinate>, start: Coordinate) -> LinkedList<Coordinate> {
    let mut path = LinkedList::new();
    let mut current = start;
    path.push_front(current);
    while connections.contains_key(&current) {
        current = *connections.get(&current).unwrap();
        path.push_front(current);
    }

    path
}

#[derive(Debug,Eq,PartialEq,Copy,Clone,Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x: x, y: y }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
