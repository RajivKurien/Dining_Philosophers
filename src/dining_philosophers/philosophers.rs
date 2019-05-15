use std::fmt::{Debug, Formatter, Error};
use crate::dining_philosophers::table::Table;

pub trait Philosopher {
    fn act(self, table: &mut Table) -> &Philosopher;
}

impl Debug for Philosopher {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Philosopher")
    }
}
