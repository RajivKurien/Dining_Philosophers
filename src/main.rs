mod thread_pool;
mod dining_philosophers;


use crate::thread_pool::thread_pool::ThreadPool;
use crate::dining_philosophers::table::Table;
use crate::dining_philosophers::thinking_philosopher::ThinkingPhilosopher;
use crate::dining_philosophers::philosophers::Actor;

use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

extern crate futures;

fn main() {
    println!("The five philosophers");

    let size = 5;
    let table = Table::new(size);
    let seating_positions = table.get_seating_positions();
    let table = Arc::new(Mutex::new(table));
    let mut actors = Vec::with_capacity(size);
    let thread_pool = ThreadPool::new(size);

    for i in 0..size {
        actors.push(Actor {
            philosopher: Box::new(ThinkingPhilosopher::new(seating_positions[i])),
            table: Arc::clone(&table),
        })
    }

    thread_pool.execute(move || {
        loop {
            for i in 0..size {
                actors[i].execute();
            }
            thread::sleep(Duration::from_secs(2));
        }
    });
}
