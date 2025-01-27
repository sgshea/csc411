use glam::IVec2;

/**
 * Represents a possible movement direction for the agent.
 */
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }

    pub fn to_ivec2(&self) -> IVec2 {
        match self {
            Self::Up => IVec2::new(0, -1),
            Self::Down => IVec2::new(0, 1),
            Self::Left => IVec2::new(-1, 0),
            Self::Right => IVec2::new(1, 0),
        }
    }
}

/**
 * An action that can be taken by the agent.
 */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    Move { direction: Direction },
    Wait,
}