use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::eating_philosopher::EatingPhilosopher;
use crate::dining_philosophers::thinking_philosopher::ThinkingPhilosopher;

#[derive(Debug, PartialEq)]
pub struct RightThinkingPhilosopher {
    right_fork: Fork,
}

impl RightThinkingPhilosopher {
    pub fn new(right_fork: Fork) -> RightThinkingPhilosopher {
        RightThinkingPhilosopher {
            right_fork
        }
    }
    fn take_left(self, fork: Fork) -> EatingPhilosopher {
        EatingPhilosopher::new(fork, self.right_fork)
    }
    fn drop_right(self) -> (ThinkingPhilosopher, Fork) {
        (ThinkingPhilosopher {}, self.right_fork)
    }
}

#[cfg(test)]
mod tests {
    use crate::dining_philosophers::right_thinking_philosopher::RightThinkingPhilosopher;
    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::eating_philosopher::EatingPhilosopher;
    use crate::dining_philosophers::thinking_philosopher::ThinkingPhilosopher;

    #[test]
    fn right_thinking_take_left_becomes_eating() {
        let phil = RightThinkingPhilosopher { right_fork: Fork };

        assert_eq!(phil.take_left(Fork), EatingPhilosopher::new(Fork, Fork));
    }

    #[test]
    fn right_thinking_drop_right_becomes_thinking() {
        let phil = RightThinkingPhilosopher { right_fork: Fork };

        let (phil, _fork) = phil.drop_right();

        assert_eq!(phil, ThinkingPhilosopher::new());
    }
}
