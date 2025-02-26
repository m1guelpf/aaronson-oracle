use itertools::Itertools;
use rand::seq::IndexedRandom;
use std::collections::{HashMap, VecDeque};

pub struct Predictor {
    n: usize,
    history: VecDeque<char>,
    pub total_predictions: usize,
    pub correct_predictions: usize,
    grams: HashMap<String, Prediction>,
}

impl Predictor {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            total_predictions: 0,
            correct_predictions: 0,
            history: VecDeque::with_capacity(n),
            grams: Self::generate_grams(n, &['f', 'j']),
        }
    }

    pub fn predict(&mut self, answer: char) -> Option<char> {
        if self.history.len() < self.n {
            self.history.push_back(answer);
            return None;
        }

        let gram = self
            .grams
            .get_mut(&self.history.iter().collect::<String>())
            .expect("gram should exist");

        let prediction = gram.predict();
        gram.register(answer);

        self.history.pop_front();
        self.history.push_back(answer);

        self.total_predictions += 1;
        if prediction == answer {
            self.correct_predictions += 1;
        }

        Some(prediction)
    }

    fn generate_grams(size: usize, chars: &[char]) -> HashMap<String, Prediction> {
        let iter = std::iter::repeat_n(chars.iter(), size);

        iter.multi_cartesian_product()
            .map(|chars| (chars.iter().join(""), Prediction::default()))
            .collect::<HashMap<_, _>>()
    }
}

#[derive(Debug, Default)]
struct Prediction {
    f: usize,
    j: usize,
}

impl Prediction {
    fn predict(&self) -> char {
        if self.f > self.j {
            return 'f';
        }
        if self.j > self.f {
            return 'j';
        }

        *['f', 'j'].choose(&mut rand::rng()).unwrap()
    }

    fn register(&mut self, c: char) {
        match c {
            'f' => self.f += 1,
            'j' => self.j += 1,
            _ => panic!("Invalid character"),
        }
    }
}
