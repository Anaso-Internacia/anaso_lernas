use rand::seq::SliceRandom;
use yew::prelude::*;

use crate::WordSet;

#[hook]
pub fn use_shuffled_words(target: WordSet, fakes: [WordSet; 3]) -> (usize, [WordSet; 4]) {
    let (correct, full_sort) = *use_memo((target, fakes), |(target, fakes)| {
        let mut full_sort = [*target, fakes[0], fakes[1], fakes[2]];
        full_sort.shuffle(&mut rand::thread_rng());

        let mut correct = 0;
        for (i, w) in full_sort.iter().enumerate() {
            if w.0 == target.0 {
                correct = i;
            }
        }

        (correct, full_sort)
    });

    (correct, full_sort)
}
