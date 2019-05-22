use std::collections::HashMap;

use crate::dining_philosophers::philosopher::state_machine::{State, StateMachine};

pub struct Philosopher {
    id: usize,
    sm: Box<StateMachine + Send>,
    history: Vec<State>,
}

impl Philosopher {
    pub fn new(id: usize, sm: Box<StateMachine + Send>) -> Self {
        let mut philosopher = Philosopher {
            id,
            sm,
            history: Vec::new(),
        };
        philosopher.history.push(philosopher.state());

        philosopher
    }

    pub fn act(&mut self) {
        self.sm = self.sm.transition();
        let state = self.sm.state();
        self.history.push(state);
    }

    pub fn write(&self, store: &mut HashMap<usize, Vec<State>>) {
        store.insert(self.id, self.history.to_vec());
    }

    fn state(&self) -> State {
        self.sm.state()
    }

    fn history(&self) -> &Vec<State> {
        return &self.history;
    }

    fn id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::dining_philosophers::philosopher::philosopher::Philosopher;
    use crate::dining_philosophers::philosopher::state_machine::{State, StateMachine};
    use crate::dining_philosophers::philosopher::state_machine::State::Thinking;

    #[test]
    fn has_state() {
        let unit = Philosopher::new(1, Box::new(MockStateMachine{}));

        assert_eq!(Thinking, unit.state());
    }

    #[test]
    fn keeps_a_record_of_state_transitions() {
        let mut unit = Philosopher::new(1, Box::new(MockStateMachine{}));

        let iterations = 10;
        for _ in 1..iterations {
            unit.act();
        }

        assert_eq!(unit.history().len(), iterations);
    }

    #[test]
    fn has_id() {
        let unit = Philosopher::new(1, Box::new(MockStateMachine{}));

        assert_eq!(unit.id(), 1);
    }

    #[test]
    fn write_history() {
        let mut hash_map = HashMap::with_capacity(1);
        let mut unit = Philosopher::new(1, Box::new(MockStateMachine{}));
        let iterations = 10;
        for _ in 1..iterations {
            unit.act();
        }

        unit.write(&mut hash_map);

        assert_eq!(hash_map.get(&1).unwrap().len(), iterations);
    }

    struct MockStateMachine {}

    impl StateMachine for MockStateMachine {
        fn transition(&mut self) -> Box<StateMachine + Send> {
            Box::new(MockStateMachine{})
        }

        fn state(&self) -> State {
            State::Thinking
        }
    }
}
