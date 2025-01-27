pub trait Agent {
    // Get textual representation of the agent
    fn get_symbol(&self) -> String;
}