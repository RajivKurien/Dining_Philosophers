use crate::dining_philosophers::table::TableInteraction;
use crate::dining_philosophers::thinking::Thinking;

pub struct Philosopher {
    sm: Box<StateMachine + Send>,
}

impl Philosopher {
    pub fn new(action: TableInteraction) -> Self {
        Philosopher {
            sm: Box::new(Thinking::new(action)),
        }
    }

    pub fn act(&mut self) {
        self.sm = self.sm.transition();
    }

    pub fn state(&self) -> State {
        self.sm.state()
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Thinking,
    LeftThinking,
    RightThinking,
    Eating,
}

pub trait StateMachine {
    fn transition(&mut self) -> Box<StateMachine + Send>;

    /// This is used only for unit testing
    /// Since we are using Trait Objects, it is difficult to get the specific type
    /// of a Philosopher
    fn state(&self) -> State;
}

#[cfg(test)]
mod tests {
    use crate::dining_philosophers::philosopher::Philosopher;
    use crate::dining_philosophers::philosopher::State::Thinking;
    use crate::dining_philosophers::table::Table;

    #[test]
    fn philosopher_starts_as_thinking() {
        let mut interactions = Table::new(2).get_interactions();

        let unit = Philosopher::new(interactions.pop().unwrap());

        assert_eq!(Thinking, unit.state());
    }
}
