use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::left_thinking::LeftThinking;
use crate::dining_philosophers::right_thinking::RightThinking;
use crate::dining_philosophers::philosopher::{State, Status};
use crate::dining_philosophers::table::{Table, TableInteraction};
use std::sync::Arc;

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

impl State for Thinking {
    fn transition(&mut self, table: &mut Table) -> Box<State + Send + Sync> {
        match self.seating_position.get_left_fork(table) {
            None => {
                println!("{}: Still thinking", self.seating_position.position);
                Box::new(Thinking::new(self.seating_position.clone()))
            }
            Some(fork) => {
                println!("{}: Got the left one!", self.seating_position.position);
                Box::new(LeftThinking::new(fork, self.seating_position.clone()))
            }
        }
    }

    fn state(&self) -> Status {
        Status::Thinking
    }
}

#[cfg(test)]
mod tests {
    use crate::dining_philosophers::thinking::Thinking;
    use crate::dining_philosophers::left_thinking::LeftThinking;
    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::right_thinking::RightThinking;
    use crate::dining_philosophers::philosopher::{State, Status};
    use crate::dining_philosophers::table::{Table, TableInteraction};
    use std::sync::{Arc, Mutex};

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

        assert_eq!(unit.state(), Status::Thinking);
    }

    #[test]
    fn changes_to_left_when_left_fork_available() {
        let mut table = Table::new(1);
        let seating_position = table.get_interactions().pop().unwrap();
        let mut unit: Box<State> = Box::new(Thinking::new(Arc::new(seating_position)));

        let mut table = Table::new(1);
        unit = unit.transition(&mut table);

        assert_eq!(unit.state(), Status::LeftThinking);
    }

    #[test]
    fn changes_to_thinking_when_left_fork_is_not_available() {
        let mut table = Table::new(1);
        let seating_position = table.get_interactions().pop().unwrap();

        let mut table = Table::new(1);
        seating_position.get_left_fork(&mut table);
        let mut unit: Box<State> = Box::new(Thinking::new(Arc::new(seating_position)));

        unit = unit.transition(&mut table);

        assert_eq!(unit.state(), Status::Thinking);
    }
}
