use crate::dining_philosophers::eating::Eating;
use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::philosopher::{State, StateMachine};
use crate::dining_philosophers::table::TableInteraction;
use crate::dining_philosophers::thinking::Thinking;

#[derive(Debug, PartialEq)]
pub struct LeftThinking {
    left_fork: Option<Fork>,
    table_interaction: Option<TableInteraction>,
}

impl LeftThinking {
    pub fn new(left_fork: Fork, table_interaction: TableInteraction) -> LeftThinking {
        LeftThinking {
            left_fork: Some(left_fork),
            table_interaction: Some(table_interaction),
        }
    }
    fn take_right(&mut self, fork: Fork, table_interaction: TableInteraction) -> Eating {
        Eating::new(self.left_fork.take().unwrap(), fork, table_interaction)
    }
    fn drop_left(&mut self, table_interaction: TableInteraction) -> Thinking {
        table_interaction.return_left_fork(self.left_fork.take().unwrap());
        Thinking::new(table_interaction)
    }
}

impl StateMachine for LeftThinking {
    fn transition(&mut self) -> Box<StateMachine + Send> {
        match self.table_interaction.take() {
            None => { panic!("No longer valid") }
            Some(t) => {
                match t.get_right_fork() {
                    None => {
                        debug!("{}: Not right, back to thinking", t.position);
                        Box::new(self.drop_left(t))
                    }
                    Some(fork) => {
                        debug!("{}: Got right. Eating!", t.position);
                        Box::new(self.take_right(fork, t))
                    }
                }
            }
        }
    }

    fn state(&self) -> State {
        State::LeftThinking
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::dining_philosophers::eating::Eating;
    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::left_thinking::LeftThinking;
    use crate::dining_philosophers::philosopher::{State, StateMachine};
    use crate::dining_philosophers::table::{Table, TableInteraction};
    use crate::dining_philosophers::thinking::Thinking;

    #[test]
    fn take_right_becomes_eating() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };

        let mut unit = LeftThinking { left_fork: Some(Fork), table_interaction: Some(table_interaction) };
        let table_interaction = unit.table_interaction.take().unwrap();

        assert_eq!(unit.take_right(Fork, table_interaction), Eating::new(Fork, Fork, TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) }));
    }

    #[test]
    fn drop_left_becomes_thinking() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let mut unit = LeftThinking { left_fork: Some(Fork), table_interaction: Some(table_interaction) };
        let table_interaction = unit.table_interaction.take().unwrap();

        let unit = unit.drop_left(table_interaction);

        assert_eq!(unit, Thinking::new(TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) }));
    }

    #[test]
    fn state_is_left_thinking() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let unit = LeftThinking::new(Fork, table_interaction);

        assert_eq!(unit.state(), State::LeftThinking);
    }

    #[test]
    fn changes_to_eating_when_right_fork_available() {
        let table = Table::new(2);
        let table_interaction = table.get_interactions().pop().unwrap();
        let fork = table_interaction.get_left_fork().unwrap();
        let mut unit: Box<StateMachine> = Box::new(LeftThinking::new(fork, table_interaction));

        unit = unit.transition();

        assert_eq!(unit.state(), State::Eating);
    }

    #[test]
    fn changes_to_thinking_when_right_fork_is_not_available() {
        let table_interaction = Table::new(1).get_interactions().pop().unwrap();
        let mut fork = table_interaction.get_left_fork();
        let mut unit: Box<StateMachine> = Box::new(LeftThinking::new(fork.take().unwrap(), table_interaction));

        unit = unit.transition();

        assert_eq!(unit.state(), State::Thinking);
    }

    #[test]
    fn returns_left_fork_when_right_fork_is_not_available() {
        let table_interaction = Table::new(1).get_interactions().pop().unwrap();
        let mut fork = table_interaction.get_left_fork();
        let mut unit: Box<StateMachine> = Box::new(LeftThinking::new(fork.take().unwrap(), table_interaction));

        unit = unit.transition();
        unit = unit.transition();

        assert_eq!(unit.state(), State::LeftThinking);
    }

    #[test]
    #[should_panic]
    fn cannot_call_transition_twice_on_same_instance() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let mut unit = LeftThinking::new(Fork, table_interaction);

        unit.transition();
        unit.transition();
    }
}
