use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::philosopher::state_machine::{State, StateMachine};
use crate::dining_philosophers::resource_hierarchy_impl::left_thinking::LeftThinking;
use crate::dining_philosophers::resource_hierarchy_impl::right_thinking::RightThinking;
use crate::dining_philosophers::table::TableInteraction;

#[derive(Debug, PartialEq)]
pub struct Thinking {
    table_interaction: Option<TableInteraction>
}

impl Thinking {
    pub fn new(table_interaction: TableInteraction) -> Thinking {
        unimplemented!()
    }
}

impl StateMachine for Thinking {
    fn transition(&mut self) -> Box<StateMachine + Send> {
        unimplemented!()
    }

    fn state(&self) -> State {
        State::Thinking
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::philosopher::state_machine::{State, StateMachine};
    use crate::dining_philosophers::resource_hierarchy_impl::left_thinking::LeftThinking;
    use crate::dining_philosophers::resource_hierarchy_impl::right_thinking::RightThinking;
    use crate::dining_philosophers::resource_hierarchy_impl::thinking::Thinking;
    use crate::dining_philosophers::table::{Table, TableInteraction};

    #[test]
    fn state_is_thinking() {
        let table_interaction = TableInteraction { position: 0, table: Arc::new(Mutex::new(Table::new(1))) };
        let unit = Thinking::new(table_interaction);

        assert_eq!(unit.state(), State::Thinking);
    }

    #[test]
    fn changes_to_left_when_left_fork_available() {
        // Write test here
    }

    #[test]
    fn changes_to_thinking_when_left_fork_is_not_available() {
        // Write test here
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
