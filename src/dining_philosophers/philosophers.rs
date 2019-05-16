use crate::dining_philosophers::table::Table;
use std::sync::{Mutex, Arc};
use core::borrow::BorrowMut;

pub struct Actor {
    pub philosopher: Box<Philosopher + Send + Sync>,
    pub table: Arc<Mutex<Table>>,
}

impl Actor {
    pub fn execute(&mut self) {
        println!("---");
        self.philosopher = self.philosopher.act(self.table.lock().unwrap().borrow_mut());
    }

//    pub fn get_closure<'a>(&'a mut self) -> Box<dyn FnMut() -> ()> {
//        Box::new(|| self.execute())
//    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Thinking,
    LeftThinking,
    RightThinking,
    Eating,
}

pub trait Philosopher {
    fn act(& mut self, table: &mut Table) -> Box<Philosopher + Send + Sync>;

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
    use crate::dining_philosophers::philosophers::State::LeftThinking;

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
