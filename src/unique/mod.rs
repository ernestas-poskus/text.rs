use std::collections::HashMap;

pub trait Unique {
    fn unique(&self) -> Vec<&str>;
}

impl<'t> Unique for HashMap<&'t str, usize> {
    fn unique(&self) -> Vec<&str> {
        self.keys().map(AsRef::as_ref).collect()
    }
}
