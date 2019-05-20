mod thread_pool;
mod dining_philosophers;


use crate::dining_philosophers::table::{Table, TableInteraction};
use crate::dining_philosophers::thinking::Thinking;
use crate::dining_philosophers::philosopher::Philosopher;

use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

extern crate futures;

fn main() {
    println!("The five philosophers");

    let size = 5;
    let table = Table::new(size);
    let seating_positions: Vec<Arc<TableInteraction>> = table.get_interactions().into_iter()
        .map(|s| Arc::new(s))
        .collect();

    let mut actors = Vec::with_capacity(size);

    for i in 0..size {
        actors.push(Philosopher {
            state: Box::new(Thinking::new(seating_positions[i].clone()))
        })
    }

    let update_state = |i: usize, actors: &mut Vec<Philosopher>| {
        loop {
            (&mut actors[i] as &mut Philosopher).act();
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
