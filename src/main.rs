extern crate env_logger;
#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::dining_philosophers::philosopher::philosopher::Philosopher;
use crate::dining_philosophers::philosopher::state_machine::State;
use crate::dining_philosophers::table::{Table, TableInteraction};
use crate::thread_pool::thread_pool::ThreadPool;

mod thread_pool;
mod dining_philosophers;


fn main() {
    env_logger::init();
    static NUMBER_OF_PHILOSOPHERS: usize = 5;
    static ITERATIONS: i32 = 200;
    let results: Arc<Mutex<HashMap<usize, Vec<State>>>> =
        Arc::new(Mutex::new(HashMap::with_capacity(NUMBER_OF_PHILOSOPHERS)));

    run_simulation(NUMBER_OF_PHILOSOPHERS, ITERATIONS, &results);

    info!("{:#?}", results.lock().unwrap());
}

fn run_simulation(number_of_philosophers: usize, iterations: i32, results: &Arc<Mutex<HashMap<usize, Vec<State>>>>) {
    info!("The {} philosophers for {} iterations", number_of_philosophers, iterations);

    let mut seating_positions: Vec<TableInteraction> = Table::new(number_of_philosophers).get_interactions();
    let pool = ThreadPool::new(number_of_philosophers);
    for _ in 0..number_of_philosophers {
        let mut p = Philosopher::new(seating_positions.pop().unwrap());
        let results_copy = Arc::clone(results);
        pool.execute(move || {
            for __ in 0..iterations {
                p.act();
            }
            p.write(&mut results_copy.lock().unwrap());
        })
    }
}
