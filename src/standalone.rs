use crate::transition::Transition;
use crate::state::State;

const TOP_MATTER: &'static str = "";

pub struct Standalone {
    initial_name: Option<String>,
    node_distance: f32,
    nodes: Vec<State>,
    edges: Vec<Transition>,
}

impl Standalone {
    pub fn new() -> Self {
        Self {
            initial_name: None,
            node_distance: 3.0,
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_state(&mut self, state: State) {
        self.nodes.push(state);
    }

    pub fn add_transition(&mut self, transition: Transition) {
        self.edges.push(transition);
    }

    pub fn build(&self) -> String {
        String::new()
    }
}