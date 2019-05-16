# Dining Philosophers

- Five philosphers sit around a table
- Forks are placed between each pair
- The philosophers alternatively think and eat
- To eat they need two forks
- A fork can only be held by one philosopher at a time

##### Problem: How to design an algorithm so that each philosopher can continue forever between eating and thinking without starving.

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
     (1) Thinking                              Eating  (4)
             \                                  /
              \                                /
               \ ------ Right Thinking ------ /
                             (3)
               
               
               
Each philosopher transitions between four possible states:
- **Thinking**: the philosopher does not hold any forks. So they are thinking
- **Left Thinking**: the philosopher holds the fork to their left. They are still thinking
- **Right Thinking**: the philosopher holds the fork to their right. They are still thinking
- **Eating**: the philosopher holds both their forks, so they are eating.

At each state the philosopher has three other possible states they can attempt to move towards.
Eg: A philosopher in state 1 can attempt move to 1, 2, or 3. 
