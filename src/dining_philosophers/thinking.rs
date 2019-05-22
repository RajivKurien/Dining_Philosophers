use std::sync::Arc;

use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::left_thinking::LeftThinking;
use crate::dining_philosophers::philosopher::{StateMachine, State};
use crate::dining_philosophers::right_thinking::RightThinking;
use crate::dining_philosophers::table::TableInteraction;

#[derive(Debug, PartialEq)]
pub struct Thinking {
    seating_position: Arc<TableInteraction>
}

impl Thinking {
    pub fn new(seating_position: Arc<TableInteraction>) -> Thinking {
        Thinking {
            seating_position,
        }
    }
    fn take_left(&self, fork: Fork) -> LeftThinking<> {
        LeftThinking::new(fork, self.seating_position.clone())
    }
    fn take_right(&self, fork: Fork) -> RightThinking<> {
        RightThinking::new(fork, self.seating_position.clone())
    }
}

impl StateMachine for Thinking {
    fn transition(&mut self) -> Box<StateMachine + Send> {
        match self.seating_position.get_left_fork() {
            None => {
                println!("{}: Still thinking", self.seating_position.position);
                Box::new(Thinking::new(self.seating_position.clone()))
            }
            Some(fork) => {
                println!("{}: Got the left one!", self.seating_position.position);
                Box::new(self.take_left(fork))
            }
        }
    }

    fn state(&self) -> State {
        State::Thinking
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::left_thinking::LeftThinking;
    use crate::dining_philosophers::philosopher::{StateMachine, State};
    use crate::dining_philosophers::right_thinking::RightThinking;
    use crate::dining_philosophers::table::{Table, TableInteraction};
    use crate::dining_philosophers::thinking::Thinking;

    #[test]
    fn take_left_becomes_left_thinking() {
        let seating_position = Arc::new(TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) });
        let unit = Thinking::new(seating_position.clone());

        assert_eq!(unit.take_left(Fork), LeftThinking::new(Fork, seating_position.clone()));
    }

    #[test]
    fn take_right_becomes_right_thinking() {
        let seating_position = Arc::new(TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) });
        let unit = Thinking::new(seating_position.clone());

        assert_eq!(unit.take_right(Fork), RightThinking::new(Fork, seating_position));
    }

    #[test]
    fn state_is_thinking() {
        let seating_position = Arc::new(TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) });
        let unit = Thinking::new(seating_position);

        assert_eq!(unit.state(), State::Thinking);
    }

    #[test]
    fn changes_to_left_when_left_fork_available() {
        let table = Table::new(1);
        let seating_position = table.get_interactions().pop().unwrap();
        let mut unit: Box<StateMachine> = Box::new(Thinking::new(Arc::new(seating_position)));

        unit = unit.transition();

        assert_eq!(unit.state(), State::LeftThinking);
    }

    #[test]
    fn changes_to_thinking_when_left_fork_is_not_available() {
        let table = Table::new(1);
        let seating_position = table.get_interactions().pop().unwrap();
        seating_position.get_left_fork();
        let mut unit: Box<StateMachine> = Box::new(Thinking::new(Arc::new(seating_position)));

        unit = unit.transition();

        assert_eq!(unit.state(), State::Thinking);
    }
}
