use crate::dining_philosophers::fork::Fork;
use crate::dining_philosophers::left_thinking_philosopher::LeftThinkingPhilosopher;
use crate::dining_philosophers::right_thinking_philosopher::RightThinkingPhilosopher;

#[derive(Debug, PartialEq)]
pub struct ThinkingPhilosopher {}

impl<> ThinkingPhilosopher {
    pub fn new() -> ThinkingPhilosopher {
        ThinkingPhilosopher {}
    }
    fn take_left(&self, fork: Fork) -> LeftThinkingPhilosopher<> {
        LeftThinkingPhilosopher::new(fork)
    }
    fn take_right(&self, fork: Fork) -> RightThinkingPhilosopher<> {
        RightThinkingPhilosopher::new(fork)
    }
}

#[cfg(test)]
mod tests {
    use crate::dining_philosophers::thinking_philosopher::ThinkingPhilosopher;
    use crate::dining_philosophers::left_thinking_philosopher::LeftThinkingPhilosopher;
    use crate::dining_philosophers::fork::Fork;
    use crate::dining_philosophers::right_thinking_philosopher::RightThinkingPhilosopher;

    #[test]
    fn thinking_take_left_becomes_left_thinking() {
        let phil = ThinkingPhilosopher::new();

        assert_eq!(phil.take_left(Fork), LeftThinkingPhilosopher::new(Fork));
    }

    #[test]
    fn thinking_take_right_becomes_right_thinking() {
        let phil = ThinkingPhilosopher::new();

        assert_eq!(phil.take_right(Fork), RightThinkingPhilosopher::new(Fork));
    }
}
