extern crate env_logger;
#[macro_use]
extern crate log;

use std::time::Duration;

use crate::dining_philosophers::philosopher::Philosopher;
use crate::dining_philosophers::table::{Table, TableInteraction};
use crate::thread_pool::thread_pool::ThreadPool;

mod thread_pool;
mod dining_philosophers;


fn main() {
    env_logger::init();

    let table_size = 5;
    let iterations = 2;
    info!("The {} philosophers", table_size);
    info!("In {} iterations", iterations);

    let pool = ThreadPool::new(table_size);
    let mut seating_positions: Vec<TableInteraction> = Table::new(table_size).get_interactions()
        .into_iter()
        .collect();

    for _ in 0..table_size {
        let mut phil = Philosopher::new(seating_positions.pop().unwrap());
        pool.execute(move || {
            for __ in 0..iterations {
                phil.act();
            }
        })
    }
}
