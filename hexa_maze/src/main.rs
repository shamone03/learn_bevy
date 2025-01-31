use std::ops::{BitAnd, BitOr, Not};

#[derive(Clone, Copy)]
pub struct NodeState(u8);

impl NodeState {
    const LEFT: NodeState = NodeState(1 << 0);
    const UP_LEFT: NodeState = NodeState(1 << 1);
    const UP_RIGHT: NodeState = NodeState(1 << 2);
    const RIGHT: NodeState = NodeState(1 << 3);
    const DOWN_LEFT: NodeState = NodeState(1 << 4);
    const DOWN_RIGHT: NodeState = NodeState(1 << 5);

    const NONE: NodeState = NodeState(1 << 6);
    const VISITED: NodeState = NodeState(1 << 7);
}

impl BitOr for NodeState {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        NodeState(self.0 | rhs.0)
    }
}

impl BitAnd for NodeState {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        NodeState(self.0 & rhs.0)
    }
}

impl Not for NodeState {
    type Output = Self;

    fn not(self) -> Self::Output {
        NodeState(!self.0)
    }
}

pub fn generate<const WIDTH: usize, const HEIGHT: usize>() -> [[NodeState; HEIGHT]; WIDTH] {
    let state = NodeState::LEFT
        | NodeState::RIGHT
        | NodeState::UP_LEFT
        | NodeState::UP_RIGHT
        | NodeState::DOWN_LEFT
        | NodeState::DOWN_RIGHT;
    [[state; HEIGHT]; WIDTH]
}

fn main() {
    println!("Hello, world!");
}
