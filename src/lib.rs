#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// An abstract representation of two possible actions.
pub enum Choice {
    Left,
    Right,
}

impl Choice {
    const fn to_bit(self) -> u32 {
        match self {
            Self::Left => 0,
            Self::Right => 1,
        }
    }

    /// Represent the choice as one of two possible values.
    pub fn display<T>(self, choices: [T; 2]) -> T {
        let [left, right] = choices;

        match self {
            Self::Left => left,
            Self::Right => right,
        }
    }
}

impl From<bool> for Choice {
    fn from(b: bool) -> Self {
        if b { Self::Right } else { Self::Left }
    }
}

pub struct Predictor {
    /// The size of the n-grams used by the predictor.
    n: usize,
    /// The current state of the predictor.
    state: u32,
    /// The number of bits in the state. It will fall out of sync once it reaches `n`.
    count: usize,
    /// A vector of predictions, indexed by the state.
    grams: Vec<Prediction>,
    /// The total number of predictions made.
    pub total_predictions: usize,
    /// The number of correct predictions made.
    pub correct_predictions: usize,
}

impl Predictor {
    /// Create a new predictor with the given n-gram size.
    ///
    /// # Arguments
    /// * `n` - The size of the n-grams used by the predictor.
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            state: 0,
            count: 0,
            total_predictions: 0,
            correct_predictions: 0,
            grams: vec![Prediction::default(); 1 << n],
        }
    }

    /// Predict the next choice, and register the correct choice.
    pub fn predict(&mut self, choice: Choice) -> Option<Choice> {
        if self.count < self.n {
            self.state = (self.state << 1) | choice.to_bit();
            self.count += 1;
            return None;
        }

        let prediction = self.grams[self.state as usize].predict();
        self.grams[self.state as usize].register(choice);

        self.state = ((self.state << 1) | choice.to_bit()) & ((1 << self.n) - 1);

        self.total_predictions += 1;
        if prediction == choice {
            self.correct_predictions += 1;
        }

        Some(prediction)
    }
}

#[derive(Debug, Clone, Copy, Default)]
/// A prediction for a given n-gram.
struct Prediction {
    left: usize,
    right: usize,
}

impl Prediction {
    /// Predict the next choice based on the current n-gram.
    fn predict(&self) -> Choice {
        match self.left.cmp(&self.right) {
            Ordering::Less => Choice::Right,
            Ordering::Greater => Choice::Left,
            Ordering::Equal => Choice::from(rand::random::<bool>()),
        }
    }

    /// Register the given choice.
    const fn register(&mut self, choice: Choice) {
        match choice {
            Choice::Left => self.left += 1,
            Choice::Right => self.right += 1,
        }
    }
}
