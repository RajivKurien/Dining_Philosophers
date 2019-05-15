use crate::dining_philosophers::fork::Fork;

#[derive(Debug,PartialEq)]
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

    fn get_forks(&mut self, position: usize) -> (Option<Fork>, Option<Fork>) {
        let next_position = (position + 1) % self.forks.len();
        (self.forks[position].take(), self.forks[next_position].take())
    }

    fn return_fork(&mut self, fork: Fork, position: usize) {
        self.forks[position] = Some(fork);
    }
}

#[cfg(test)]
mod tests {
    use crate::dining_philosophers::table::Table;
    use crate::dining_philosophers::fork::Fork;

    #[test]
    fn construct_table() {
        let table = Table::new(1);

        assert_eq!(table, Table { forks: vec![Some(Fork {})] });
    }

    #[test]
    fn get_forks_by_position() {
        let mut table = Table::new(2);

        let (left_fork, right_fork) = table.get_forks(0);

        assert_ne!(left_fork, None);
        assert_ne!(right_fork, None);
    }

    #[test]
    fn get_forks_for_last_position() {
        let mut table = Table::new(2);

        let (left_fork, right_fork) = table.get_forks(1);

        assert_ne!(left_fork, None);
        assert_ne!(right_fork, None);
    }

    #[test]
    #[should_panic]
    fn get_forks_panics_when_position_is_larger_than_table_size() {
        let mut table = Table::new(2);

        table.get_forks(3);
    }

    #[test]
    fn cannot_get_fork_when_in_use() {
        let mut table = Table::new(1);

        let (_left_fork, right_fork) = table.get_forks(0);

        assert_eq!(right_fork, None);
    }

    #[test]
    fn return_fork_to_table() {
        let mut table = Table::new(1);
        let position = 0;
        let (left_fork, _right_fork) = table.get_forks(position);

        table.return_fork(left_fork.unwrap(), position);
        let (left_fork, _right_fork) = table.get_forks(position);

        assert_ne!(left_fork, None);
    }

    #[test]
    #[should_panic]
    fn return_fork_panics_when_position_larger_than_table_size() {
        let mut table = Table::new(1);
        let (left_fork, _right_fork) = table.get_forks(0);

        table.return_fork(left_fork.unwrap(), 2);
    }
}


