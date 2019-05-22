use std::sync::{Arc, Mutex};

use crate::dining_philosophers::fork::Fork;

#[derive(Debug, PartialEq)]
pub struct Table {
    forks: Vec<Option<Fork>>,
}

impl Table {
    pub fn new(size: usize) -> Table {
        let mut forks = Vec::with_capacity(size);

        for _ in 0..size {
            forks.push(Some(Fork {}));
        }

        Table {
            forks,
        }
    }

    pub fn get_interactions(self) -> Vec<TableInteraction> {
        let size = self.forks.len();
        let mut table_interactions = Vec::with_capacity(size);
        let table = Mutex::new(self);
        let arc = Arc::new(table);

        for id in 0..size {
            table_interactions.push(TableInteraction { position: id, table: Arc::clone(&arc) })
        }

        table_interactions
    }

    fn get_fork(&mut self, position: usize) -> Option<Fork> {
        match position < self.forks.len() {
            true => self.forks[position].take(),
            false => None
        }
    }

    fn return_fork(&mut self, fork: Fork, position: usize) {
        self.forks[position] = Some(fork);
    }
}

#[derive(Debug)]
pub struct TableInteraction {
    pub position: usize,
    pub table: Arc<Mutex<Table>>,
}


impl PartialEq for TableInteraction {
    fn eq(&self, other: &TableInteraction) -> bool {
        self.position == other.position
    }
}

impl TableInteraction {
    pub fn get_left_fork(&self) -> Option<Fork> {
        match self.table.lock()
            .map(|mut t| {
                t.get_fork(self.position)
            }) {
            Ok(value) => { value }
            Err(_) => {
                println!("{}: Ack! Couldn't get hold of table", self.position);
                None
            }
        }
    }
    pub fn return_left_fork(&self, fork: Fork) {
        match self.table.lock() {
            Ok(mut table) => { table.return_fork(fork, self.position) }
            Err(_) => { panic!("Could not return the left fork!!") }
        }
    }
    pub fn get_right_fork(&self) -> Option<Fork> {
        match self.table.lock()
            .map(|mut t| {
                let next_position = (self.position + 1) % t.forks.len();
                t.get_fork(next_position)
            }) {
            Ok(value) => { value }
            Err(_) => { None }
        }
    }
    pub fn return_right_fork(&self, fork: Fork) {
        match self.table.lock() {
            Ok(mut table) => {
                let next_position = (self.position + 1) % table.forks.len();
                table.return_fork(fork, next_position)
            }
            Err(_) => { panic!("Could not return the right fork!!") }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::table::Table;

    #[test]
    fn construct_table() {
        let unit = Table::new(1);

        assert_eq!(unit, Table { forks: vec![Some(Fork {})] });
    }

    #[test]
    fn get_fork_by_position() {
        let mut unit = Table::new(2);

        let fork = unit.get_fork(0);

        assert_ne!(fork, None);
    }

    #[test]
    fn cannot_get_fork_of_index_larger_than_table_size() {
        let mut unit = Table::new(2);

        assert_eq!(unit.get_fork(3), None);
    }

    #[test]
    fn cannot_get_fork_of_index_equal_to_table_size() {
        let mut unit = Table::new(2);

        assert_eq!(unit.get_fork(2), None);
    }

    #[test]
    fn cannot_get_fork_when_in_use() {
        let mut unit = Table::new(1);
        let _fork = unit.get_fork(0);

        let same_fork = unit.get_fork(0);

        assert_eq!(same_fork, None);
    }

    #[test]
    fn return_fork_to_table() {
        let mut unit = Table::new(1);
        let position = 0;
        let fork = unit.get_fork(position);

        unit.return_fork(fork.unwrap(), position);
        let fork = unit.get_fork(position);

        assert_ne!(fork, None);
    }

    #[test]
    #[should_panic]
    fn return_fork_panics_when_position_larger_than_table_size() {
        let mut unit = Table::new(1);
        let fork = unit.get_fork(0);

        unit.return_fork(fork.unwrap(), 2);
    }

    #[test]
    fn table_interaction_get_fork() {
        let mut table_interactions = Table::new(1).get_interactions();
        let unit = table_interactions.pop().unwrap();

        let fork = unit.get_left_fork();

        assert_ne!(fork, None);
    }

    #[test]
    fn table_interaction_get_adjacent_fork() {
        let mut table_interactions = Table::new(2).get_interactions();
        let unit = table_interactions.pop().unwrap();

        unit.get_left_fork();
        let fork = unit.get_right_fork();

        assert_ne!(fork, None);
    }

    #[test]
    fn table_interaction_cannot_get_same_fork() {
        let mut table_interactions = Table::new(1).get_interactions();
        let unit = table_interactions.pop().unwrap();

        unit.get_left_fork();
        let fork = unit.get_right_fork();

        assert_eq!(fork, None);
    }

    #[test]
    fn table_interaction_returns_fork() {
        let mut table_interactions = Table::new(1).get_interactions();
        let unit = table_interactions.pop().unwrap();
        let fork = unit.get_left_fork().unwrap();

        unit.return_left_fork(fork);

        assert_ne!(unit.get_left_fork(), None);
    }

    #[test]
    fn table_interaction_returns_adjacent_fork() {
        let mut table_interactions = Table::new(1).get_interactions();
        let unit = table_interactions.pop().unwrap();
        let fork = unit.get_left_fork().unwrap();

        unit.return_right_fork(fork);

        assert_ne!(unit.get_left_fork(), None);
    }
}
