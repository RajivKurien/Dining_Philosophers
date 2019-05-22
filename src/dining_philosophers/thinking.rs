use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::left_thinking::LeftThinking;
use crate::dining_philosophers::philosopher::{State, StateMachine};
use crate::dining_philosophers::right_thinking::RightThinking;
use crate::dining_philosophers::table::TableInteraction;

#[derive(Debug, PartialEq)]
pub struct Thinking {
    table_interaction: Option<TableInteraction>
}

impl Thinking {
    pub fn new(table_interaction: TableInteraction) -> Thinking {
        Thinking {
            table_interaction: Some(table_interaction),
        }
    }
    fn take_left(&mut self, fork: Fork, seating_position: TableInteraction) -> LeftThinking<> {
        LeftThinking::new(fork, seating_position)
    }
    fn take_right(&mut self, fork: Fork, seating_position: TableInteraction) -> RightThinking<> {
        RightThinking::new(fork, seating_position)
    }
}

impl StateMachine for Thinking {
    fn transition(&mut self) -> Box<StateMachine + Send> {
        match self.table_interaction.take() {
            None => { panic!("No longer valid") }
            Some(t) => {
                match t.get_left_fork() {
                    None => {
                        debug!("{}: Still thinking", t.position);
                        Box::new(Thinking::new(t))
                    }
                    Some(fork) => {
                        debug!("{}: Got the left one!", t.position);
                        Box::new(self.take_left(fork, t))
                    }
                }
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
    use crate::dining_philosophers::philosopher::{State, StateMachine};
    use crate::dining_philosophers::right_thinking::RightThinking;
    use crate::dining_philosophers::table::{Table, TableInteraction};
    use crate::dining_philosophers::thinking::Thinking;

    #[test]
    fn take_left_becomes_left_thinking() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let mut unit = Thinking::new(table_interaction);
        let table_interaction = unit.table_interaction.take().unwrap();

        assert_eq!(unit.take_left(Fork, table_interaction), LeftThinking::new(Fork, TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) }));
    }

    #[test]
    fn take_right_becomes_right_thinking() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let mut unit = Thinking::new(table_interaction);
        let table_interaction = unit.table_interaction.take().unwrap();

        assert_eq!(unit.take_right(Fork, table_interaction), RightThinking::new(Fork, TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) }));
    }

    #[test]
    fn state_is_thinking() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let unit = Thinking::new(table_interaction);

        assert_eq!(unit.state(), State::Thinking);
    }

    #[test]
    fn changes_to_left_when_left_fork_available() {
        let table = Table::new(1);
        let table_interaction = table.get_interactions().pop().unwrap();
        let mut unit: Box<StateMachine> = Box::new(Thinking::new(table_interaction));

        unit = unit.transition();

        assert_eq!(unit.state(), State::LeftThinking);
    }

    #[test]
    fn changes_to_thinking_when_left_fork_is_not_available() {
        let table = Table::new(1);
        let table_interaction = table.get_interactions().pop().unwrap();
        table_interaction.get_left_fork();
        let mut unit: Box<StateMachine> = Box::new(Thinking::new(table_interaction));

        unit = unit.transition();

        assert_eq!(unit.state(), State::Thinking);
    }

    #[test]
    #[should_panic]
    fn cannot_call_transition_twice_on_same_instance() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let mut unit = Thinking::new(table_interaction);

        unit.transition();
        unit.transition();
    }
}
