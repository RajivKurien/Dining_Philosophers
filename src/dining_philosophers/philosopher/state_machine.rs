#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Thinking,
    LeftThinking,
    RightThinking,
    Eating,
}

pub trait StateMachine {
    fn transition(&mut self) -> Box<StateMachine + Send>;
    fn state(&self) -> State;
}
