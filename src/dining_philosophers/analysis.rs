use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::dining_philosophers::philosopher::state_machine::State;
use crate::dining_philosophers::philosopher::state_machine::State::Eating;

fn calculate_percentage(history: &Vec<State>) -> f32 {
    let total: f32 = history.len() as f32;
    let no_of_thinking: i32 = history.iter()
        .map(|s| { if s != &Eating { 1 } else { 0 } })
        .sum();

    100_f32 * no_of_thinking as f32 / total
}

fn score(percentage: f32) -> f32 {
    1_f32 - (percentage / 50_f32 - 1_f32).abs()
}

pub fn score_one_run(results: &HashMap<usize, Vec<State>>) -> f32 {
    let total_score: f32 = results.iter()
        .map(|(_key, value)| {
            let perc = calculate_percentage(value);
            score(perc)
        })
        .sum();
    let normalised_score = total_score / results.keys().len() as f32;

    normalised_score
}

pub fn compute_average_score(results: &Vec<Arc<Mutex<HashMap<usize, Vec<State>>>>>) -> f32 {
    let total_score: f32 = results.iter()
        .map(|map| {
            score_one_run(&map.lock().unwrap())
        })
        .sum();
    total_score / results.len() as f32
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use crate::dining_philosophers::analysis::{calculate_percentage, compute_average_score, score, score_one_run};
    use crate::dining_philosophers::philosopher::state_machine::State::{Eating, LeftThinking, RightThinking, Thinking};
    use crate::dining_philosophers::philosopher::state_machine::State;

    #[test]
    fn calculate_percentage_thinking() {
        let results = vec![Thinking, LeftThinking, RightThinking];
        let percentage = calculate_percentage(&results);

        assert_eq!(100_f32, percentage);
    }

    #[test]
    fn calculate_percentage_thinking_with_eating() {
        let results = vec![Thinking, LeftThinking, RightThinking, Eating];
        let percentage = calculate_percentage(&results);

        assert_eq!(75_f32, percentage);
    }

    #[test]
    fn calculate_percentage_thinking_with_equal_eating() {
        let results = vec![Thinking, LeftThinking, Eating, Eating];
        let percentage = calculate_percentage(&results);

        assert_eq!(50_f32, percentage);
    }

    #[test]
    fn calculate_score_based_on_percentage() {
        assert_eq!(0.5_f32, score(75_f32));
    }

    #[test]
    fn max_score_of_one_when_fifty_percent() {
        assert_eq!(1_f32, score(50_f32));
    }

    #[test]
    fn min_score_of_zero_when_hundred_percent() {
        assert_eq!(0_f32, score(100_f32));
    }

    #[test]
    fn min_score_of_zero_when_zero_percent() {
        assert_eq!(0_f32, score(0_f32));
    }

    #[test]
    fn min_score_a_run() {
        let mut results: HashMap<usize, Vec<State>> = HashMap::new();
        results.insert(0, vec![Thinking]);
        results.insert(1, vec![Thinking]);

        let score = score_one_run(&results);
        assert_eq!(0_f32, score);
    }

    #[test]
    fn max_score_for_a_run() {
        let mut results: HashMap<usize, Vec<State>> = HashMap::new();
        results.insert(0, vec![Thinking, Eating]);
        results.insert(1, vec![Thinking, Eating]);

        let score = score_one_run(&results);
        assert_eq!(1_f32, score);
    }

    #[test]
    fn intermediate_score_for_a_run() {
        let mut results: HashMap<usize, Vec<State>> = HashMap::new();
        results.insert(0, vec![Thinking, Thinking]);
        results.insert(1, vec![Thinking, Eating]);

        let score = score_one_run(&results);
        assert_eq!(0.5_f32, score);
    }

    #[test]
    fn average_score_over_several_runs() {
        let mut results: Vec<Arc<Mutex<HashMap<usize, Vec<State>>>>> = Vec::new();
        let run_one: Arc<Mutex<HashMap<usize, Vec<State>>>> = Arc::new(Mutex::new(HashMap::new()));
        let run_two: Arc<Mutex<HashMap<usize, Vec<State>>>> = Arc::new(Mutex::new(HashMap::new()));

        run_one.lock().unwrap().insert(0, vec![Thinking, Thinking]);
        run_one.lock().unwrap().insert(1, vec![Thinking, Eating]);
        run_two.lock().unwrap().insert(0, vec![Thinking, Thinking]);
        run_two.lock().unwrap().insert(1, vec![Thinking, Eating]);

        results.push(run_one);
        results.push(run_two);

        assert_eq!(0.5_f32, compute_average_score(&results));
    }

    #[test]
    fn max_average_score_over_several_runs() {
        let mut results: Vec<Arc<Mutex<HashMap<usize, Vec<State>>>>> = Vec::new();
        let run_one: Arc<Mutex<HashMap<usize, Vec<State>>>> = Arc::new(Mutex::new(HashMap::new()));
        let run_two: Arc<Mutex<HashMap<usize, Vec<State>>>> = Arc::new(Mutex::new(HashMap::new()));

        run_one.lock().unwrap().insert(0, vec![Thinking, Eating]);
        run_one.lock().unwrap().insert(1, vec![Thinking, Eating]);
        run_two.lock().unwrap().insert(0, vec![Thinking, Eating]);
        run_two.lock().unwrap().insert(1, vec![Thinking, Eating]);

        results.push(run_one);
        results.push(run_two);

        assert_eq!(1_f32, compute_average_score(&results));
    }

    #[test]
    fn min_average_score_over_several_runs() {
        let mut results: Vec<Arc<Mutex<HashMap<usize, Vec<State>>>>> = Vec::new();
        let run_one: Arc<Mutex<HashMap<usize, Vec<State>>>> = Arc::new(Mutex::new(HashMap::new()));
        let run_two: Arc<Mutex<HashMap<usize, Vec<State>>>> = Arc::new(Mutex::new(HashMap::new()));

        run_one.lock().unwrap().insert(0, vec![Thinking, Thinking]);
        run_one.lock().unwrap().insert(1, vec![Eating, Eating]);
        run_two.lock().unwrap().insert(0, vec![Thinking, Thinking]);
        run_two.lock().unwrap().insert(1, vec![Eating, Eating]);

        results.push(run_one);
        results.push(run_two);

        assert_eq!(0_f32, compute_average_score(&results));
    }
}
