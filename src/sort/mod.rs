use std::collections::HashMap;
use occurrence::Occurrence;

pub trait Sort {
    fn sort_by_occurrence(&self) -> Vec<String>;
    fn sort_by_occurrence_with_map(&self, &HashMap<&str, usize>) -> Vec<String>;
    fn repetitions(&self) -> Vec<(String, usize)>;
}

impl Sort for Vec<String> {
    fn sort_by_occurrence_with_map(&self, occurred: &HashMap<&str, usize>) -> Vec<String> {
        let default: usize = 0;

        let mut sorted: Vec<String> = self.clone();

        sorted.sort_by(|a, b| {
            let found_a = occurred.get(a.as_str()).unwrap_or(&default);
            let found_b = occurred.get(b.as_str()).unwrap_or(&default);

            found_b.cmp(found_a)
        });
        sorted
    }

    fn sort_by_occurrence(&self) -> Vec<String> {
        self.sort_by_occurrence_with_map(&self.occurrence())
    }

    fn repetitions(&self) -> Vec<(String, usize)> {
        let occurred = self.occurrence();
        let mut sorted: Vec<String> = self.clone().sort_by_occurrence_with_map(&occurred);
        let mut repetitions: Vec<(String, usize)> = Vec::new();

        sorted.dedup();

        for s in sorted {
            let times = *occurred.get(s.as_str()).unwrap();
            repetitions.push((s, times));
        }

        repetitions
    }
}
