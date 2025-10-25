use rand::seq::IndexedRandom;
use std::cell::LazyCell;

const NAME_ADJVS: LazyCell<Vec<&'static str>> =
    LazyCell::new(|| include_str!("adjectives").split("\n").collect());

const NAME_NOUNS: LazyCell<Vec<&'static str>> =
    LazyCell::new(|| include_str!("nouns").split("\n").collect());

const NAME_VERBS: LazyCell<Vec<&'static str>> =
    LazyCell::new(|| include_str!("verbs").split("\n").collect());

pub fn random_room_name() -> String {
    let mut rng = rand::rng();
    format!(
        "{}-{}-{}",
        NAME_ADJVS.choose(&mut rng).unwrap(),
        NAME_NOUNS.choose(&mut rng).unwrap(),
        NAME_VERBS.choose(&mut rng).unwrap()
    )
}
