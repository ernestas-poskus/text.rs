use std::borrow::Cow;

pub trait Intersect {
    fn intersect(&self, &Vec<&str>) -> Vec<&str>;
}

impl Intersect for Vec<String> {
    fn intersect(&self, other: &Vec<&str>) -> Vec<&str> {
        let mut intersected: Vec<&str> = Vec::new();
        if self.len() == 0 || other.len() == 0 {
            return intersected;
        }
        let mut other_iter = other.iter();
        for v1 in self.iter() {
            if other_iter.position(|v2| v1 == v2).is_some() {
                intersected.push(v1);
            }
        }
        intersected
    }
}

impl<'s> Intersect for Vec<&'s str> {
    fn intersect(&self, other: &Vec<&str>) -> Vec<&str> {
        let mut intersected: Vec<&str> = Vec::new();
        if self.len() == 0 || other.len() == 0 {
            return intersected;
        }
        let mut other_iter = other.iter();
        for v1 in self.iter() {
            if other_iter.position(|v2| v1 == v2).is_some() {
                intersected.push(v1);
            }
        }
        intersected
    }
}

impl<'s> Intersect for Vec<Cow<'s, str>> {
    fn intersect(&self, other: &Vec<&str>) -> Vec<&str> {
        let mut intersected: Vec<&str> = Vec::new();
        if self.len() == 0 || other.len() == 0 {
            return intersected;
        }
        let mut other_iter = other.iter();
        for v1 in self.iter() {
            if other_iter.position(|v2| v1 == v2).is_some() {
                intersected.push(v1);
            }
        }
        intersected
    }
}
