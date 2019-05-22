# Dining Philosophers
#### Problem: How to design an algorithm so that each philosopher can continue forever between eating and thinking without starving.
- Five philosphers sit around a table
- Forks are placed between each pair
- The philosophers alternatively think and eat
- To eat they need two forks
- A fork can only be held by one philosopher at a time



https://en.wikipedia.org/wiki/Dining_philosophers_problem

Philosophers seated at the numbers, and the forks at the letters.
             
           ,'''.
     2   ,'  b  `. 3
       ,'         `.
     ,' a         c `.
     \               /
    1 \             / 4
       \  d      e /
        \_________/
             5



## Philosopher states

                             (2)
               / ------ Left Thinking  ------ \  
              /                                \
             /                                  \
     (1) Thinking  <-------------------------  Eating  (4)
             \                                  /
              \                                /
               \ ------ Right Thinking ------ /
                             (3)
               
               
               
Each philosopher transitions between four possible states:
- **Thinking**: the philosopher does not hold any forks. So they are thinking
- **Left Thinking**: the philosopher holds the fork to their left. They are still thinking
- **Right Thinking**: the philosopher holds the fork to their right. They are still thinking
- **Eating**: the philosopher holds both their forks, so they are eating.

At each state the philosopher (except state 4) has two (excluding self) possible states they can attempt to move towards.
Eg: A philosopher in state 1 can attempt move to 2 or 3.
A philosopher at state 4 can move to three other states (excluding the identity transition). 


## Logging
The project uses the env_logger crate which is configured via the environment variable `RUST_LOG`. Eg:

```
RUST_LOG=debug ./target/debug/Dining_Philosophers
```

## Thread pool
The thread pool implementation is straight from the book:
[Building a multi-threaded web server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
