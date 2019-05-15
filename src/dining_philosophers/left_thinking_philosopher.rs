use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::eating_philosopher::EatingPhilosopher;
use crate::dining_philosophers::thinking_philosopher::ThinkingPhilosopher;

#[derive(Debug, PartialEq)]
pub struct LeftThinkingPhilosopher<> {
    left_fork: Fork,
}

impl LeftThinkingPhilosopher {
    pub fn new(left_fork: Fork) -> LeftThinkingPhilosopher {
        LeftThinkingPhilosopher {
            left_fork
        }
    }
    fn take_right(self, fork: Fork) -> EatingPhilosopher {
        EatingPhilosopher::new(self.left_fork, fork)
    }
    fn drop_left(self) -> (ThinkingPhilosopher, Fork) {
        (ThinkingPhilosopher::new(), self.left_fork)
    }
}

#[cfg(test)]
mod tests {
    use crate::dining_philosophers::left_thinking_philosopher::LeftThinkingPhilosopher;
    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::eating_philosopher::EatingPhilosopher;
    use crate::dining_philosophers::thinking_philosopher::ThinkingPhilosopher;

    #[test]
    fn left_thinking_take_right_becomes_eating() {
        let phil = LeftThinkingPhilosopher { left_fork: Fork };

        assert_eq!(phil.take_right(Fork), EatingPhilosopher::new(Fork, Fork));
    }

    #[test]
    fn left_thinking_drop_left_becomes_thinking() {
        let phil = LeftThinkingPhilosopher { left_fork: Fork };

        let (phil, _fork) = phil.drop_left();

        assert_eq!(phil, ThinkingPhilosopher::new());
    }
}
