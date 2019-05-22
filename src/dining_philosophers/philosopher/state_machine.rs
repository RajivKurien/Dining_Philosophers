#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Thinking,
    LeftThinking,
    RightThinking,
    Eating,
}

pub trait StateMachine {
    fn transition(&mut self) -> Box<StateMachine + Send>;

    /// This is used only for unit testing
    /// Since we are using Trait Objects, it is difficult to get the specific type
    /// of a Philosopher
    fn state(&self) -> State;
}
