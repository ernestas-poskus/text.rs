use std::collections::HashMap;

pub trait Repetitions {
    fn repetitions(&self) -> Vec<(&str, usize)>;
}

impl<'t> Repetitions for HashMap<&'t str, usize> {
    fn repetitions(&self) -> Vec<(&str, usize)> {
        let mut repetitions: Vec<(&str, usize)> = Vec::new();

        for (k, v) in self.iter() {
            repetitions.push((k, *v));
        }

        repetitions.sort_by(|a, b| b.1.cmp(&a.1));
        repetitions
    }
}
