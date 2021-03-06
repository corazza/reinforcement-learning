use std::hash::Hash;

pub type Reward = f32;

pub mod gridworld;
pub mod gridworld_definitions;
pub mod m_wrapper;
pub mod t_corridor;

// Reward process interface
pub trait Environment {
    type Action: Copy + Hash + Eq + std::fmt::Debug + PartialOrd + Ord;
    // either an index or a state representation
    type State: Copy + Hash + Eq + std::fmt::Debug + PartialOrd + Ord;

    /// Returns None on all transitions from terminal state
    fn take_action(&mut self, action: Self::Action) -> Option<(Self::State, Reward)>;

    /// Returns all actions available at state
    fn available_actions(&self, state: Self::State) -> Vec<Self::Action>;

    fn current_state(&self) -> Self::State;

    /// Returns true if MDP is episodic and has reached the goal/terminal state
    fn terminated(&self) -> bool;

    /// Check if state is terminal
    fn is_terminal(&self, state: Self::State) -> bool;

    fn get_terminal(&self) -> Self::State;
}

/// Interface for a Markov decision process
pub trait MDP: Environment {}
/// Interface for a non-Markov decision process
pub trait NMDP: Environment {}
