use core::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

use crate::dining_philosophers::table::Table;

pub struct Philosopher {
    pub state: Box<State + Send + Sync>,
}

impl Philosopher {
    pub fn act(&mut self) {
        println!("---");
        unimplemented!();
//        self.state = self.state.transition(self.table.lock().unwrap().borrow_mut());
    }

//    pub fn get_closure<'a>(&'a mut self) -> Box<dyn FnMut() -> ()> {
//        Box::new(|| self.execute())
//    }
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
//    use crate::dining_philosophers::philosopher::Philosopher;
//    use crate::dining_philosophers::thinking::Thinking;
//    use std::sync::{Arc, Mutex};
//    use crate::dining_philosophers::table::Table;

//    #[test]
//    fn get_closures() {
//        let table = Table::new(1);
//        let seating_positions = table.get_seating_positions();
//        let mut table = Arc::new(Mutex::new(table));
//        let mut unit = Actor {
//            philosopher: Box::new(ThinkingPhilosopher::new(seating_positions[0])),
//            table: Arc::clone(&table),
//        };
//
//        let closure = unit.get_closure();
//
//        closure();
//
//        assert_eq!(unit.philosopher.state(), LeftThinking);
//    }
}
