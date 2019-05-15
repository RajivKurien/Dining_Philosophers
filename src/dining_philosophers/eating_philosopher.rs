use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::left_thinking_philosopher::LeftThinkingPhilosopher;
use crate::dining_philosophers::right_thinking_philosopher::RightThinkingPhilosopher;

#[derive(Debug, PartialEq)]
pub struct EatingPhilosopher {
    left_fork: Fork,
    right_fork: Fork,
}

impl EatingPhilosopher {
    pub fn new(left_fork: Fork, right_fork: Fork) -> EatingPhilosopher {
        EatingPhilosopher {
            left_fork,
            right_fork,
        }
    }

    fn drop_left(self) -> (RightThinkingPhilosopher, Fork) {
        (RightThinkingPhilosopher::new(self.right_fork), self.left_fork)
    }
    fn drop_right(self) -> (LeftThinkingPhilosopher, Fork) {
        (LeftThinkingPhilosopher::new(self.left_fork), self.right_fork)
    }
}

#[cfg(test)]
mod tests {
    use crate::dining_philosophers::eating_philosopher::EatingPhilosopher;
    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::left_thinking_philosopher::LeftThinkingPhilosopher;
    use crate::dining_philosophers::right_thinking_philosopher::RightThinkingPhilosopher;

    #[test]
    fn eating_drop_right_becomes_left_thinking() {
        let phil = EatingPhilosopher { left_fork: Fork, right_fork: Fork };

        let (phil, _fork) = phil.drop_right();

        assert_eq!(phil, LeftThinkingPhilosopher::new(Fork));
    }

    #[test]
    fn eating_drop_left_becomes_right_thinking() {
        let phil = EatingPhilosopher { left_fork: Fork, right_fork: Fork };

        let (phil, _fork) = phil.drop_left();

        assert_eq!(phil, RightThinkingPhilosopher::new(Fork));
    }
}
