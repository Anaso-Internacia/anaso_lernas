use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    rc::Rc,
};

use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng, Rng,
};
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
    next_others: Cell<[&'static str; 3]>,
    stats: Cell<Option<[i32; 8]>>,
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
            next_others: Cell::new([""; 3]),
            stats: Cell::new(None),
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
        let mut vdata = DATA.words.iter().collect::<Vec<_>>();
        vdata.shuffle(&mut thread_rng());

        for (word, _) in vdata.iter() {
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
        let others = data
            .iter()
            .skip(2)
            .choose_multiple(&mut thread_rng(), 3)
            .iter()
            .map(|x| x.text)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        self.next_others.set(others);
        self.stats.set(None);
    }

    /// Get statistics about how well the user is doing
    pub fn stats(&self) -> [i32; 8] {
        if let Some(stats) = self.stats.get() {
            stats
        } else {
            let mut stats = [0; 8];
            for data in self.data.borrow().iter() {
                stats[data.level.min(7) as usize] += 1;
            }
            self.stats.set(Some(stats));
            stats
        }
    }

    /// Get the number of cards in the queue
    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }

    /// Get the currently shown card
    pub fn current(&self) -> PlayerWordData {
        self.data.borrow()[0]
    }

    /// Get the next shown card
    pub fn next(&self) -> PlayerWordData {
        self.data.borrow()[1]
    }

    /// Get the current bad dumb stupid words
    pub fn fakes(&self) -> [&'static str; 3] {
        self.three_others.get()
    }

    /// Get the next set of bad dumb stupid words
    pub fn next_fakes(&self) -> [&'static str; 3] {
        self.next_others.get()
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
            let pow = (word_data.level - mistakes).max(1).pow(2);
            let new_position = thread_rng().gen_range((pow * 2)..(pow * 3)).max(5);
            let len = data.len();
            data.insert((new_position as usize).min(len - 1), word_data);
            let others = data
                .iter()
                .skip(2)
                .filter(|x| x.level > 0)
                .choose_multiple(&mut thread_rng(), 3)
                .iter()
                .map(|x| x.text)
                .chain(["pomo", "banano", "kivo"].into_iter().cycle())
                .take(3)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            self.three_others.set(self.next_others.get());
            self.next_others.set(others);
        }
        self.stats.set(None);
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
