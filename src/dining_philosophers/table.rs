use crate::dining_philosophers::fork::Fork;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Table {
    forks: Vec<Option<Fork>>,
}

impl Table {
    pub fn new(size: usize) -> Table {
        let mut forks = Vec::with_capacity(size);
        let mut seating_positions = Vec::with_capacity(size);

        for id in 0..size {
            forks.push(Some(Fork {}));
            seating_positions.push(Rc::new(SeatingPosition { position: id }))
        }

        Table {
            forks,
        }
    }

    pub fn get_seating_positions(&self) -> Vec<SeatingPosition> {
        let size = self.forks.len();
        let mut seating_positions = Vec::with_capacity(size);

        for id in 0..size {
            seating_positions.push(SeatingPosition { position: id })
        }

        seating_positions
    }

    fn get_fork(&mut self, position: usize) -> Option<Fork> {
        self.forks[position].take()
    }

    fn return_fork(&mut self, fork: Fork, position: usize) {
        self.forks[position] = Some(fork);
    }
}

#[derive(Debug, PartialEq, Copy)]
pub struct SeatingPosition {
    pub position: usize
}

impl SeatingPosition {
    pub fn get_left_fork(&self, table: &mut Table) -> Option<Fork> {
        table.get_fork(self.position)
    }
    pub fn return_left_fork(&self, fork: Fork, table: &mut Table) {
        table.return_fork(fork, self.position);
    }
    pub fn get_right_fork(&self, table: &mut Table) -> Option<Fork> {
        let next_position = (self.position + 1) % table.forks.len();
        table.get_fork(next_position)
    }
    pub fn return_right_fork(&self, fork: Fork, table: &mut Table) {
        let next_position = (self.position + 1) % table.forks.len();
        table.return_fork(fork, next_position);
    }
}

impl Clone for SeatingPosition {
    fn clone(&self) -> Self {
        SeatingPosition {
            position: self.position
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dining_philosophers::table::{Table, SeatingPosition};
    use crate::dining_philosophers::fork::Fork;
    use std::rc::Rc;

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
    #[should_panic]
    fn get_forks_panics_when_position_is_larger_than_table_size() {
        let mut unit = Table::new(2);

        unit.get_fork(3);
    }

    #[test]
    fn cannot_get_fork_when_in_use() {
        let mut unit = Table::new(1);
        let fork = unit.get_fork(0);

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
    fn seating_position_get_fork() {
        let mut table = Table::new(1);
        let mut seating_positions = table.get_seating_positions();
        let unit = seating_positions.pop().unwrap();

        let fork = unit.get_left_fork(&mut table);

        assert_ne!(fork, None);
    }

    #[test]
    fn seating_position_get_adjacent_fork() {
        let mut table = Table::new(2);
        let mut seating_positions = table.get_seating_positions();
        let unit = seating_positions.pop().unwrap();

        let _left_fork = unit.get_left_fork(&mut table);
        let fork = unit.get_right_fork(&mut table);

        assert_ne!(fork, None);
    }

    #[test]
    fn seating_position_cannot_get_same_fork() {
        let mut table = Table::new(1);
        let mut seating_positions = table.get_seating_positions();
        let unit = seating_positions.pop().unwrap();

        let _left_fork = unit.get_left_fork(&mut table);
        let fork = unit.get_right_fork(&mut table);

        assert_eq!(fork, None);
    }

    #[test]
    fn seating_position_return_fork() {
        let mut table = Table::new(1);
        let mut seating_positions = table.get_seating_positions();
        let unit = seating_positions.pop().unwrap();

        let fork = unit.get_left_fork(&mut table).unwrap();

        unit.return_left_fork(fork, &mut table);

        assert_ne!(table.get_fork(0), None);
    }

    #[test]
    fn seating_position_return_adjacent_fork() {
        let mut table = Table::new(2);
        let mut seating_positions = table.get_seating_positions();
        let unit = seating_positions.pop().unwrap();

        let fork = unit.get_left_fork(&mut table).unwrap();

        unit.return_right_fork(fork, &mut table);

        assert_ne!(table.get_fork(0), None);
    }
}


