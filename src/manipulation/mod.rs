use regex::Regex;
use std::borrow::Cow;

lazy_static! {
    pub static ref PUNCT: Regex = Regex::new("[[:punct:]]").unwrap();
}

pub trait Manipulation {
    fn average_length(&self) -> f64;
    fn char_count(&self) -> usize;
    fn remove_punct<'t>(&'t self) -> Vec<Cow<'t, str>>;
    fn punct_count(&self) -> usize;
    fn words_count(&self) -> usize;
    fn words(&self) -> Vec<&str>;
}

impl<'s> Manipulation for Vec<Cow<'s, str>> {
    fn char_count(&self) -> usize {
        self.iter().fold(0, |sum, i| sum + i.chars().count())
    }

    fn words_count(&self) -> usize {
        self.len()
    }

    fn average_length(&self) -> f64 {
        let total = self.char_count();
        if total == 0 {
            return 0.0;
        }
        total as f64 / self.len() as f64
    }

    fn remove_punct<'t>(&'t self) -> Vec<Cow<'t, str>> {
        self.iter()
            .map(|s| PUNCT.replace_all(s, ""))
            .filter(|f| !f.is_empty())
            .collect()
    }

    fn punct_count(&self) -> usize {
        self.iter().fold(0, |sum, text| sum + PUNCT.find_iter(text).count())
    }

    fn words(&self) -> Vec<&str> {
        self.iter().flat_map(|s| s.split_whitespace()).collect()
    }
}

impl<'s> Manipulation for Vec<&'s str> {
    fn char_count(&self) -> usize {
        self.iter().fold(0, |sum, i| sum + i.chars().count())
    }

    fn words_count(&self) -> usize {
        self.len()
    }

    fn average_length(&self) -> f64 {
        let total = self.char_count();
        if total == 0 {
            return 0.0;
        }
        total as f64 / self.words_count() as f64
    }

    fn remove_punct<'t>(&'t self) -> Vec<Cow<'t, str>> {
        self.iter()
            .map(|s| PUNCT.replace_all(s, ""))
            .filter(|f| !f.is_empty())
            .collect()
    }

    fn punct_count(&self) -> usize {
        self.iter().fold(0, |sum, text| sum + PUNCT.find_iter(text).count())
    }

    fn words(&self) -> Vec<&str> {
        self.iter().flat_map(|s| s.split_whitespace()).collect()
    }
}

impl Manipulation for String {
    fn average_length(&self) -> f64 {
        self.len() as f64
    }

    fn char_count(&self) -> usize {
        self.chars().count()
    }

    fn remove_punct<'t>(&'t self) -> Vec<Cow<'t, str>> {
        vec![PUNCT.replace_all(self, "")]
    }

    fn punct_count(&self) -> usize {
        PUNCT.find_iter(self).count()
    }

    fn words_count(&self) -> usize {
        self.words().len()
    }

    fn words(&self) -> Vec<&str> {
        self.split_whitespace().collect()
    }
}

impl Manipulation for Vec<String> {
    fn words_count(&self) -> usize {
        self.len()
    }

    fn char_count(&self) -> usize {
        self.iter().fold(0, |sum, i| sum + i.chars().count())
    }

    fn average_length(&self) -> f64 {
        let total = self.iter().fold(0, |sum, i| sum + i.chars().count());
        if total == 0 {
            return 0.0;
        }
        total as f64 / self.len() as f64
    }

    fn remove_punct<'t>(&'t self) -> Vec<Cow<'t, str>> {
        self.iter()
            .map(|s| PUNCT.replace_all(s, ""))
            .filter(|f| !f.is_empty())
            .collect()
    }

    fn punct_count(&self) -> usize {
        self.iter().fold(0, |sum, text| sum + PUNCT.find_iter(text).count())
    }

    fn words(&self) -> Vec<&str> {
        self.iter().flat_map(|s| s.split_whitespace()).collect()
    }
}
