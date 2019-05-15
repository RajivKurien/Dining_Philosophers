mod thread_pool;
mod dining_philosophers;


use crate::thread_pool::thread_pool::ThreadPool;
use crate::dining_philosophers::philosophers::{ Philosopher};
use crate::dining_philosophers::table::Table;


fn main() {
    println!("The five philosophers");

    let size = 5;
    let philosophers: Vec<Box<dyn Philosopher>> = Vec::with_capacity(size);
    let pool = ThreadPool::new(size);
    let table = Table::new(size);
}
