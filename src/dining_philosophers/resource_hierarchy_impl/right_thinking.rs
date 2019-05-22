use crate::dining_philosophers::resource_hierarchy_impl::eating::Eating;
use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::table::TableInteraction;
use crate::dining_philosophers::resource_hierarchy_impl::thinking::Thinking;
use crate::dining_philosophers::philosopher::state_machine::{StateMachine, State};

#[derive(Debug, PartialEq)]
pub struct RightThinking {
    right_fork: Option<Fork>,
    table_interaction: Option<TableInteraction>,
}

impl RightThinking {
    pub fn new(right_fork: Fork, table_interaction: TableInteraction) -> RightThinking {
        RightThinking { right_fork: Some(right_fork), table_interaction: Some(table_interaction) }
    }
    fn take_left(&mut self, fork: Fork, table_interaction: TableInteraction) -> Eating {
        Eating::new(fork, self.right_fork.take().unwrap(), table_interaction)
    }
    fn drop_right(&mut self, table_interaction: TableInteraction) -> Thinking {
        table_interaction.return_right_fork(self.right_fork.take().unwrap());
        Thinking::new(table_interaction)
    }
}

impl StateMachine for RightThinking {
    fn transition(&mut self) -> Box<StateMachine + Send> {
        match self.table_interaction.take() {
            None => { panic!("No longer valid") }
            Some(t) => {
                match t.get_left_fork() {
                    None => {
                        debug!("{}: Not left, back to thinking", t.position);
                        Box::new(self.drop_right(t))
                    }
                    Some(fork) => {
                        debug!("{}: Got left. Eating!", t.position);
                        Box::new(self.take_left(fork, t))
                    }
                }
            }
        }
    }

    fn state(&self) -> State {
        State::RightThinking
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::resource_hierarchy_impl::right_thinking::RightThinking;
    use crate::dining_philosophers::table::{Table, TableInteraction};
    use crate::dining_philosophers::resource_hierarchy_impl::thinking::Thinking;
    use crate::dining_philosophers::philosopher::state_machine::{State, StateMachine};
    use crate::dining_philosophers::resource_hierarchy_impl::eating::Eating;

    #[test]
    fn right_thinking_take_left_becomes_eating() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let mut unit = RightThinking { right_fork: Some(Fork), table_interaction: Some(table_interaction) };
        let table_interaction = unit.table_interaction.take().unwrap();

        assert_eq!(unit.take_left(Fork, table_interaction), Eating::new(Fork, Fork, TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) }));
    }

    #[test]
    fn right_thinking_drop_right_becomes_thinking() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let mut unit = RightThinking { right_fork: Some(Fork), table_interaction: Some(table_interaction) };
        let table_interaction = unit.table_interaction.take().unwrap();

        let unit = unit.drop_right(table_interaction);

        assert_eq!(unit, Thinking::new(TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) }));
    }

    #[test]
    fn state_is_right_thinking() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let unit = RightThinking::new(Fork, table_interaction);

        assert_eq!(unit.state(), State::RightThinking);
    }

    #[test]
    fn changes_to_eating_when_left_fork_available() {
        let table = Table::new(2);
        let table_interaction = table.get_interactions().pop().unwrap();
        let fork = table_interaction.get_right_fork().unwrap();
        let mut unit: Box<StateMachine> = Box::new(RightThinking::new(fork, table_interaction));

        unit = unit.transition();

        assert_eq!(unit.state(), State::Eating);
    }

    #[test]
    fn changes_to_thinking_when_left_fork_is_not_available() {
        let table = Table::new(1);
        let table_interaction = table.get_interactions().pop().unwrap();
        let mut fork = table_interaction.get_right_fork();
        let mut unit: Box<StateMachine> = Box::new(RightThinking::new(fork.take().unwrap(), table_interaction));

        unit = unit.transition();

        assert_eq!(unit.state(), State::Thinking);
    }

    #[test]
    fn returns_right_fork_when_left_fork_is_not_available() {
        let table_interaction = Table::new(1).get_interactions().pop().unwrap();
        let fork = table_interaction.get_right_fork().take().unwrap();
        let mut unit: Box<StateMachine> = Box::new(RightThinking::new(fork, table_interaction));

        unit = unit.transition();
        unit = unit.transition();

        assert_ne!(unit.state(), State::Thinking);
    }

    #[test]
    #[should_panic]
    fn cannot_call_transition_twice_on_same_instance() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let mut unit = RightThinking::new(Fork, table_interaction);

        unit.transition();
        unit.transition();
    }
}
