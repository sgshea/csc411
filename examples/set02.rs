use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

use csc411::{
    action::{Action, Direction},
    agent::Agent,
    environment::{Environment, EnvironmentState},
    map::{Map, Tile},
};
use glam::IVec2;

#[derive(Clone, Copy, PartialEq, Eq)]
struct PositionNode {
    position: IVec2,
    cost: i32,
}

impl PartialOrd for PositionNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.partial_cmp(&other.cost)?.reverse())
    }
}

impl Ord for PositionNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.partial_cmp(&other.cost).unwrap().reverse()
    }
}

struct AStar {
    came_from: HashMap<IVec2, IVec2>,
    cost_so_far: HashMap<IVec2, i32>,

    frontier: BinaryHeap<PositionNode>,
}

impl AStar {
    fn new(start: IVec2) -> AStar {
        let mut cost_so_far = HashMap::new();
        cost_so_far.insert(start, 0);
        let mut frontier = BinaryHeap::new();
        frontier.push(PositionNode {
            position: start,
            cost: 0,
        });
        AStar {
            came_from: HashMap::new(),
            cost_so_far,
            frontier,
        }
    }

    fn run(&mut self, map: &Map, goal: &IVec2) -> Option<Direction> {
        if self.frontier.len() > 0 {
            let current = self.frontier.pop()?;
            if current.position == *goal {
                return None;
            }

            let cost = self.cost_so_far[&current.position] + 1;
            // Get neighbors
            let neighbors = map.get_neighbors(&current.position);
            for (neighbor, (_direction, _tile)) in &neighbors {
                if !self.cost_so_far.contains_key(&neighbor) || cost < self.cost_so_far[&neighbor] {
                    self.cost_so_far.insert(*neighbor, cost);
                    let priority = cost + manhattan_distance(&neighbor, goal);
                    self.frontier.push(PositionNode {
                        position: *neighbor,
                        cost: priority,
                    });
                    self.came_from.insert(*neighbor, current.position);
                }
            }

            // Choose least cost
            if !self.frontier.is_empty() {
                let next = self.frontier.pop()?;
                // Get from neighbors map and return
                let (direction, _tile) = neighbors.get(&next.position)?;
                return Some(*direction);
            }
        }

        None
    }
}

fn manhattan_distance(a: &IVec2, b: &IVec2) -> i32 {
    ((b.x - a.x).abs() + (b.y - a.y).abs()) as i32
}

struct Robot {
    position: IVec2,
}

impl Agent for Robot {
    fn get_symbol(&self) -> String {
        "R".to_string()
    }
}

struct SimulationEnvironment {
    map: Map,
    robot: Robot,
    goal_position: IVec2,

    astar: AStar,

    state: EnvironmentState,
    turn_count: u32,
}

impl SimulationEnvironment {
    fn new(map: Map, robot_position: IVec2, goal_position: IVec2) -> Self {
        let robot = Robot {
            position: robot_position,
        };
        Self {
            map,
            robot,
            goal_position,
            astar: AStar::new(robot_position),
            turn_count: 0,
            state: EnvironmentState::START,
        }
    }
}

impl Environment for SimulationEnvironment {
    fn run(&mut self) {
        self.turn_count += 1;

        let action = match self.astar.run(&self.map, &self.goal_position) {
            Some(direction) => Action::Move { direction },
            None => Action::Wait,
        };
        match action {
            Action::Move { direction } => match direction {
                Direction::Up => self.robot.position += IVec2::new(0, -1),
                Direction::Down => self.robot.position += IVec2::new(0, 1),
                Direction::Left => self.robot.position += IVec2::new(-1, 0),
                Direction::Right => self.robot.position += IVec2::new(1, 0),
            },
            Action::Wait => {}
        }

        // Check if end condition reached and set state accordingly
        self.state = if self.robot.position == self.goal_position {
            EnvironmentState::END
        } else {
            EnvironmentState::RUN
        };
    }

    fn get_agents(&self) -> Vec<Box<&impl Agent>> {
        vec![Box::new(&self.robot)]
    }

    fn get_goal(&self, _agent: &impl Agent) -> Option<IVec2> {
        Some(self.goal_position)
    }

    fn get_state(&self) -> (EnvironmentState, u32) {
        (self.state, self.turn_count)
    }

    fn get_environment_info(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    fn get_map(&self) -> &Map {
        &self.map
    }
}

impl Display for SimulationEnvironment {
    // Displays agent on top of map
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if self.robot.position == IVec2::new(x as i32, y as i32) {
                    output.push_str(&self.robot.get_symbol());
                    continue;
                } else {
                    match self.map.get_tile(IVec2::new(x as i32, y as i32)) {
                        Tile::IMPASSABLE => output.push('W'),
                        Tile::CLEAN => output.push('C'),
                        Tile::DIRTY => output.push('D'),
                        Tile::TARGET => output.push('T'),
                    }
                }
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

fn main() {
    let map = Map::load_from_file("assets/maps/map01.txt").unwrap();
    let target_position = map
        .get_all_of_type(Tile::TARGET)
        .keys()
        .next()
        .copied()
        .expect("map should have at least one target");
    let robot_position = IVec2::new(0, 0);
    let mut env = SimulationEnvironment::new(map, robot_position, target_position);

    for _ in 0..100 {
        env.run();
        println!("{}\nstate:{:?}", env, env.get_state());
    }
}
