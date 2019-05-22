use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::left_thinking::LeftThinking;
use crate::dining_philosophers::philosopher::{State, StateMachine};
use crate::dining_philosophers::right_thinking::RightThinking;
use crate::dining_philosophers::table::TableInteraction;

#[derive(Debug, PartialEq)]
pub struct Eating {
    left_fork: Option<Fork>,
    right_fork: Option<Fork>,
    table_interaction: Option<TableInteraction>,
}

impl Eating {
    pub fn new(left_fork: Fork, right_fork: Fork, table_interaction: TableInteraction) -> Eating {
        Eating {
            left_fork: Some(left_fork),
            right_fork: Some(right_fork),
            table_interaction: Some(table_interaction),
        }
    }

    fn drop_left(&mut self, table_interaction: TableInteraction) -> RightThinking {
        table_interaction.return_left_fork(self.left_fork.take().unwrap());
        RightThinking::new(self.right_fork.take().unwrap(), table_interaction)
    }
    fn drop_right(&mut self, table_interaction: TableInteraction) -> LeftThinking {
        table_interaction.return_right_fork(self.right_fork.take().unwrap());
        LeftThinking::new(self.left_fork.take().unwrap(), table_interaction)
    }
}

impl StateMachine for Eating {
    fn transition(&mut self) -> Box<StateMachine + Send> {
        match self.table_interaction.take() {
            None => { panic!("No longer valid") }
            Some(t) => {
                debug!("{}: Drop left, to right thinking", t.position);
                Box::new(self.drop_left(t))
            }
        }
    }

    fn state(&self) -> State {
        State::Eating
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::dining_philosophers::eating::Eating;
    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::left_thinking::LeftThinking;
    use crate::dining_philosophers::philosopher::{State, StateMachine};
    use crate::dining_philosophers::right_thinking::RightThinking;
    use crate::dining_philosophers::table::{Table, TableInteraction};

    #[test]
    fn eating_drop_right_becomes_left_thinking() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let mut unit = Eating { left_fork: Some(Fork), right_fork: Some(Fork), table_interaction: Some(table_interaction) };
        let table_interaction = unit.table_interaction.take().unwrap();

        let unit = unit.drop_right(table_interaction);

        assert_eq!(unit, LeftThinking::new(Fork, TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) }));
    }

    #[test]
    fn eating_drop_left_becomes_right_thinking() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let mut unit = Eating { left_fork: Some(Fork), right_fork: Some(Fork), table_interaction: Some(table_interaction) };
        let table_interaction = unit.table_interaction.take().unwrap();

        let unit = unit.drop_left(table_interaction);

        assert_eq!(unit, RightThinking::new(Fork, TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) }));
    }

    #[test]
    fn state_is_eating() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let unit = Eating::new(Fork, Fork, table_interaction);

        assert_eq!(unit.state(), State::Eating);
    }

    #[test]
    fn changes_to_right_thinking() {
        let table = Table::new(2);
        let table_interaction = table.get_interactions().pop().unwrap();
        let left_fork = table_interaction.get_left_fork().unwrap();
        let right_fork = table_interaction.get_right_fork().unwrap();
        let mut unit: Box<StateMachine> = Box::new(Eating::new(left_fork, right_fork, table_interaction));

        unit = unit.transition();

        assert_eq!(unit.state(), State::RightThinking);
    }

    #[test]
    fn acts_to_return_left_fork() {
        let table = Table::new(2);
        let mut interactions = table.get_interactions();
        let table_interaction = interactions.pop().unwrap();
        let left_fork = table_interaction.get_left_fork().unwrap();
        let right_fork = table_interaction.get_right_fork().unwrap();
        let mut unit: Box<StateMachine> = Box::new(Eating::new(left_fork, right_fork, table_interaction));

        unit = unit.transition(); // to right thinking
        unit = unit.transition(); // back to eating

        assert_eq!(unit.state(), State::Eating);
    }

    #[test]
    #[should_panic]
    fn cannot_call_transition_twice_on_same_instance() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let mut unit = Eating::new(Fork, Fork, table_interaction);

        unit.transition();
        unit.transition();
    }
}
