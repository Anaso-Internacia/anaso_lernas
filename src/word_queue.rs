use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    rc::Rc,
};

use rand::{seq::IteratorRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::DATA;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct PlayerWordData {
    pub text: &'static str,
    pub level: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct WordQueue {
    data: RefCell<VecDeque<PlayerWordData>>,
    three_others: Cell<[&'static str; 3]>,
}

impl PartialEq for WordQueue {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl WordQueue {
    /// Create a new queue based on a word set
    pub fn new() -> Self {
        let mut s = Self {
            data: RefCell::new(VecDeque::with_capacity(DATA.words.len())),
            three_others: Cell::new([""; 3]),
        };
        s.update_word_set();
        s
    }

    /// Update with current word set
    pub fn update_word_set(&self) {
        let mut data = self.data.borrow_mut();
        if data.len() != 0 {
            return;
        }
        for (word, _) in DATA.words.iter() {
            data.push_back(PlayerWordData {
                text: word,
                level: 0,
            });
        }
        let others = data
            .iter()
            .skip(1)
            .choose_multiple(&mut thread_rng(), 3)
            .iter()
            .map(|x| x.text)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        self.three_others.set(others);
    }

    /// Get the currently shown card
    pub fn current(&self) -> PlayerWordData {
        self.data.borrow()[0]
    }

    /// Get the current bad dumn stupid words
    pub fn fakes(&self) -> [&'static str; 3] {
        self.three_others.get()
    }

    /// For debugging
    pub fn all_as_vec(&self) -> Vec<PlayerWordData> {
        self.data.borrow().iter().copied().collect()
    }

    /// Send the current card back based on how well the player did
    ///
    /// `nonce` is the current word. This protects against double-submission.
    pub fn submit(&self, mistakes: i32, nonce: &str) {
        let mut data = self.data.borrow_mut();
        if data[0].text == nonce {
            let mut word_data = data.pop_front().unwrap();
            word_data.level = (word_data.level + 1 - mistakes).max(0);
            let new_position =
                ((word_data.level - mistakes) * 10).max(10) + thread_rng().gen_range(-2..3);
            data.insert(new_position as usize, word_data);
            let others = data
                .iter()
                .skip(1)
                .choose_multiple(&mut thread_rng(), 3)
                .iter()
                .map(|x| x.text)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            self.three_others.set(others);
        }
    }
}
pub type WordQueueContext = UseReducerHandle<WordQueue>;

pub enum WordQueueAction {
    Submit { attempts: i32, nonce: &'static str },
}

impl Reducible for WordQueue {
    type Action = WordQueueAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            WordQueueAction::Submit { attempts, nonce } => {
                self.submit(attempts, nonce);
            }
        }
        self
    }
}
