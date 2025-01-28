use std::collections::HashMap;

use glam::IVec2;

use crate::{agent::Agent, map::Map};

// Simple state enum for the environment
// Run indicates that the environment ran the last turn
// End indicates that the environment has reached a finishing state
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EnvironmentState {
    START,
    RUN,
    END,
}

/**
 * Environment represented as a trait, exposing functions needed to interact with interfaces such as GUIs or other systems.
 */
pub trait Environment {
    // Runs a step of the environment and updates its state accordingly
    fn run(&mut self);

    // Get the map state from the environment
    fn get_map(&self) -> &Map;
    // Get agents in the environment
    fn get_agents(&self) -> Vec<Box<&impl Agent>>;
    // Gets the goal for a certain agent
    fn get_goal(&self, agent: &impl Agent) -> Option<IVec2>;
    // Gets the environemnt state (START, RUN, END) along with a turn counter
    fn get_state(&self) -> (EnvironmentState, u32);
    // Get other information about the environment
    fn get_environment_info(&self) -> HashMap<String, String>;
}
