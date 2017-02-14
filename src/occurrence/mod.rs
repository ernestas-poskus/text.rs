use std::collections::HashMap;
use std::borrow::Cow;

pub trait Occurrence {
    fn occurrence(&self) -> HashMap<&str, usize>;
}

impl<'t> Occurrence for Vec<&'t str> {
    fn occurrence(&self) -> HashMap<&str, usize> {
        let mut got: HashMap<&str, usize> = HashMap::new();
        for t in self {
            let occurred = got.entry(t).or_insert(0);
            *occurred += 1;
        }
        got
    }
}

impl Occurrence for Vec<String> {
    fn occurrence(&self) -> HashMap<&str, usize> {
        let mut got: HashMap<&str, usize> = HashMap::new();
        for t in self {
            let occurred = got.entry(t).or_insert(0);
            *occurred += 1;
        }
        got
    }
}

impl<'t> Occurrence for Vec<Cow<'t, str>> {
    fn occurrence(&self) -> HashMap<&str, usize> {
        let mut got: HashMap<&str, usize> = HashMap::new();
        for t in self {
            let occurred = got.entry(t).or_insert(0);
            *occurred += 1;
        }
        got
    }
}
