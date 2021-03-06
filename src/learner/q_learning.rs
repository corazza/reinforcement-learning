use crate::environment::{Environment, Reward};
pub use crate::learner::{TabularLearner, TabularLearnerConfig, TabularLearnerData};

pub struct QLearning<E: Environment> {
    pub config: TabularLearnerConfig,
    data: TabularLearnerData<E>,
}

impl<E: Environment> QLearning<E> {
    pub fn new(config: TabularLearnerConfig, terminal_state: E::State) -> QLearning<E> {
        let data = TabularLearnerData::new(terminal_state);
        QLearning { config, data }
    }
}

impl<E: Environment> TabularLearner<E> for QLearning<E> {
    // env is preinitialized
    fn episode(&mut self, env: &mut E) -> Reward {
        self.data.terminal_state = env.get_terminal();
        let mut state = env.current_state();
        let mut gain: Reward = 0.0;

        loop {
            let action = self.epsilon_greedy(self.config.epsilon, env.current_state(), env);
            let (next_state, reward) = env.take_action(action).unwrap();
            // episode() assumes gamma=1
            gain += reward;
            let target = reward + self.config.gamma * self.max_action_value(next_state, &env);
            self.update(self.config.alpha, state, action, target);

            if self.config.debug {
                println!("{:?} -> {:?}", state, next_state);
            }

            state = next_state;
            if state == self.data.terminal_state {
                break;
            }
        }

        gain
    }

    fn data(&self) -> &TabularLearnerData<E> {
        &self.data
    }

    fn data_mut(&mut self) -> &mut TabularLearnerData<E> {
        &mut self.data
    }

    fn config(&self) -> &TabularLearnerConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut TabularLearnerConfig {
        &mut self.config
    }
}
