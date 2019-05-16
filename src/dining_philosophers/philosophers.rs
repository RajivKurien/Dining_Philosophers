use crate::dining_philosophers::table::Table;
use std::sync::{Mutex, Arc};
use core::borrow::BorrowMut;

pub struct Actor {
    pub philosopher: Box<Philosopher + Send>,
    pub table: Arc<Mutex<Table>>,
}

impl Actor {
    pub fn execute(&mut self) {
        println!("---");
        self.philosopher = self.philosopher.act(self.table.lock().unwrap().borrow_mut());
    }

    pub fn get_closure(&mut self) -> Box<dyn Fn(&mut Self) -> ()> {
        Box::new(|actor| actor.execute())
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Thinking,
    LeftThinking,
    RightThinking,
    Eating,
}

pub trait Philosopher {
    fn act(&mut self, table: &mut Table) -> Box<Philosopher + Send>;

    /// This is used only for unit testing
    /// Since we are using Trait Objects, it is difficult to get the specific type
    /// of a Philosopher
    fn state(&self) -> State;
}

#[cfg(test)]
mod tests {
    use crate::dining_philosophers::philosophers::Actor;
    use crate::dining_philosophers::thinking_philosopher::ThinkingPhilosopher;
    use std::sync::{Arc, Mutex};
    use crate::dining_philosophers::table::Table;
    use crate::dining_philosophers::philosophers::State::Thinking;

//    #[test]
//    fn get_closures() {
//        let mut table = Arc::new(Mutex::new(Table::new(1)));
//        let mut unit = Actor {
//            philosopher: Box::new(ThinkingPhilosopher::new(seating_positions[i])),
//            table: Arc::clone(&table),
//        };
//
//        let closure = unit.get_closure();
//
//        closure.
//
//        assert_eq!(unit.philosopher.state(), Thinking);
//    }
}
