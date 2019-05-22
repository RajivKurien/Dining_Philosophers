use std::sync::Arc;

use crate::dining_philosophers::table::TableInteraction;
use crate::dining_philosophers::thinking::Thinking;

pub struct Philosopher {
    state: Box<State + Send + Sync>,
}

impl Philosopher {
    pub fn new(action: TableInteraction) -> Self {
        Philosopher {
            state: Box::new(Thinking::new(Arc::new(action))),
        }
    }

    pub fn act(&mut self) {
        self.state = self.state.transition();
    }

    pub fn state(&self) -> Status {
        self.state.state()
    }
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Thinking,
    LeftThinking,
    RightThinking,
    Eating,
}

pub trait State {
    fn transition(&mut self) -> Box<State + Send + Sync>;

    /// This is used only for unit testing
    /// Since we are using Trait Objects, it is difficult to get the specific type
    /// of a Philosopher
    fn state(&self) -> Status;
}

#[cfg(test)]
mod tests {
    use crate::dining_philosophers::philosopher::Philosopher;
    use crate::dining_philosophers::philosopher::Status::LeftThinking;
    use crate::dining_philosophers::table::Table;

    #[test]
    fn philosopher_state_changes() {
        let table = Table::new(2);
        let mut interactions = table.get_interactions();
        let mut philosopher = Philosopher::new(interactions.pop().unwrap());

        philosopher.act();

        assert_eq!(LeftThinking, philosopher.state());
    }
}
