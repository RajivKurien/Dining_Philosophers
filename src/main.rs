extern crate env_logger;
#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::dining_philosophers::analysis::compute_average_score;
use crate::dining_philosophers::philosopher::philosopher::Philosopher;
use crate::dining_philosophers::philosopher::state_machine::{State, StateMachine};
use crate::dining_philosophers::resource_hierarchy_impl::thinking::Thinking;
use crate::dining_philosophers::table::{Table, TableInteraction};
use crate::thread_pool::thread_pool::ThreadPool;

mod thread_pool;
mod dining_philosophers;


fn main() {
    env_logger::init();
    static NUMBER_OF_PHILOSOPHERS: usize = 5;
    static ITERATIONS_PER_RUN: i32 = 200;
    static TOTAL_RUNS: usize = 50;
    let mut results: Arc<Vec<Arc<Mutex<HashMap<usize, Vec<State>>>>>> = create_results_vector(TOTAL_RUNS, NUMBER_OF_PHILOSOPHERS);

    run_several_simulations(&mut results, TOTAL_RUNS, NUMBER_OF_PHILOSOPHERS, ITERATIONS_PER_RUN);

    info!("*** Final Score = {:#?} ***", compute_average_score(&results));
}

fn run_several_simulations(results: &mut Arc<Vec<Arc<Mutex<HashMap<usize, Vec<State>>>>>>, total_runs: usize, number_of_philosophers: usize, iterations: i32) {
    let pool = ThreadPool::new(20);
    for i in 0..total_runs {
        let store = Arc::clone(&results[i]);
        pool.execute(move || {
            run_simulation(number_of_philosophers, iterations, &store, i)
        });
    }
}

fn run_simulation(number_of_philosophers: usize, iterations: i32, results: &Arc<Mutex<HashMap<usize, Vec<State>>>>, run_number: usize) {
    info!("{}: Simulating {} philosophers for {} steps", run_number, number_of_philosophers, iterations);

    let mut table_interactions: Vec<TableInteraction> = Table::new(number_of_philosophers)
        .get_interactions();

    let pool = ThreadPool::new(number_of_philosophers);

    for _ in 0..number_of_philosophers {
        let table_interaction = table_interactions.pop().unwrap();

        let mut p = Philosopher::new(
            table_interaction.position,
            Box::new(Thinking::new(table_interaction)));

        let results_copy = Arc::clone(results);

        pool.execute(move || {
            for __ in 0..iterations {
                p.act();
            }
            p.write(&mut results_copy.lock().unwrap());
        })
    }
}

fn create_results_vector(outer_vec_size: usize, hash_map_size: usize) -> Arc<Vec<Arc<Mutex<HashMap<usize, Vec<State>>>>>> {
    let mut v: Vec<Arc<Mutex<HashMap<usize, Vec<State>>>>> = Vec::with_capacity(outer_vec_size);
    for __ in 0..outer_vec_size {
        v.push(Arc::new(Mutex::new(HashMap::with_capacity(hash_map_size))))
    }
    Arc::new(v)
}

pub struct AlwaysThinking {}

impl StateMachine for AlwaysThinking {
    fn transition(&mut self) -> Box<StateMachine + Send> {
        Box::new(AlwaysThinking {})
    }

    fn state(&self) -> State {
        State::Thinking
    }
}
