mod thread_pool;
mod dining_philosophers;


use crate::thread_pool::thread_pool::ThreadPool;
use crate::dining_philosophers::table::{Table, SeatingPosition};
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
    let seating_positions: Vec<Arc<SeatingPosition>> = table.get_seating_positions().into_iter()
        .map(|s| Arc::new(s))
        .collect();

    let table = Arc::new(Mutex::new(table));
    let mut actors = Vec::with_capacity(size);
//    let thread_pool = ThreadPool::new(size);

    for i in 0..size {
        actors.push(Actor {
            philosopher: Box::new(ThinkingPhilosopher::new(seating_positions[i].clone())),
            table: Arc::clone(&table),
        })
    }

    let update_state = |i: usize, actors: &mut Vec<Actor>| {
        loop {
            (&mut actors[i] as &mut Actor).execute();
            thread::sleep(Duration::from_secs(2));
        }
    };

    // Todo: Use the interior mutability pattern to pass an array of actors to threads

//    let mut join_handles = Vec::with_capacity(size);

//    for i in 0..size {
//        join_handles.push(thread::spawn( || {
//            update_state(i, &mut actors);
//        }));
//
//        join_handles[0].join();
//    }
}
