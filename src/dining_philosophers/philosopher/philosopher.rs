use std::collections::HashMap;

use crate::dining_philosophers::table::TableInteraction;
use crate::dining_philosophers::philosopher::state_machine::{StateMachine, State};
use crate::dining_philosophers::resource_hierarchy_impl::thinking::Thinking;

pub struct Philosopher {
    id: usize,
    sm: Box<StateMachine + Send>,
    history: Vec<State>,
}

impl Philosopher {
    pub fn new(action: TableInteraction) -> Self {
        let mut philosopher = Philosopher {
            id: action.position,
            sm: Box::new(Thinking::new(action)),
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

    pub fn state(&self) -> State {
        self.sm.state()
    }

    pub fn history(&self) -> &Vec<State> {
        return &self.history;
    }

    pub fn write(&self, store: &mut HashMap<usize, Vec<State>>) {
        store.insert(self.id, self.history.to_vec());
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::dining_philosophers::table::Table;
    use crate::dining_philosophers::philosopher::philosopher::Philosopher;
    use crate::dining_philosophers::philosopher::state_machine::State::Thinking;

    #[test]
    fn starts_as_thinking() {
        let mut interactions = Table::new(2).get_interactions();

        let unit = Philosopher::new(interactions.pop().unwrap());

        assert_eq!(Thinking, unit.state());
    }

    #[test]
    fn keeps_a_record_of_state_transitions() {
        let mut interactions = Table::new(2).get_interactions();
        let mut unit = Philosopher::new(interactions.pop().unwrap());

        let iterations = 10;
        for _ in 1..iterations {
            unit.act();
        }

        assert_eq!(unit.history().len(), iterations);
    }

    #[test]
    fn has_id() {
        let mut interactions = Table::new(2).get_interactions();

        let unit = Philosopher::new(interactions.pop().unwrap());

        assert_eq!(unit.id(), 1);
    }

    #[test]
    fn write_history() {
        let mut interactions = Table::new(2).get_interactions();
        let mut unit = Philosopher::new(interactions.pop().unwrap());
        let mut hash_map = HashMap::with_capacity(1);
        let iterations = 10;
        for _ in 1..iterations {
            unit.act();
        }

        unit.write(&mut hash_map);

        assert_eq!(hash_map.get(&1).unwrap().len(), iterations);
    }
}
