use std::borrow::Cow;

pub trait Unique {
    fn unique(&mut self) -> Vec<&str>;
}

impl<'s> Unique for Vec<&'s str> {
    fn unique(&mut self) -> Vec<&str> {
        self.dedup_by(|a, b| a.to_lowercase() == b.to_lowercase());
        self.to_vec()
    }
}

impl Unique for Vec<String> {
    fn unique(&mut self) -> Vec<&str> {
        self.dedup_by(|a, b| a.to_lowercase() == b.to_lowercase());
        self.iter().map(AsRef::as_ref).collect()
    }
}

impl<'t> Unique for Vec<Cow<'t, str>> {
    fn unique(&mut self) -> Vec<&str> {
        self.dedup_by(|a, b| a.to_lowercase() == b.to_lowercase());
        self.iter().map(AsRef::as_ref).collect()
    }
}
