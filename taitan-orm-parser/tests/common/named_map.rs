use std::borrow::Cow;
use std::collections::hash_map::IntoValues;
use std::collections::HashMap;

pub trait Named {
    fn name<'a>(&'a self) -> Cow<'a, str>;
}



#[derive(Debug, Default, Clone)]
pub struct NamedMap<T: Named> {
    inputs: HashMap<String, T>,
}

impl <T: Named> NamedMap<T> {
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
        }
    }
    pub fn insert(&mut self, input: T) {
        if self.inputs.contains_key(input.name().as_ref()) {
            panic!("named input name already taken")
        }
        self.inputs.insert(input.name().to_string(), input);
    }

    pub fn get<N: AsRef<str>>(&self, name: N) -> Option<&T> {
        self.inputs.get(name.as_ref())
    }
}

pub struct NamedMapIterator<T: Named> {
    inner: IntoValues<String, T>,
}


impl <T: Named> Iterator for NamedMapIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl <T: Named>IntoIterator for NamedMap<T> {
    type Item = T;
    type IntoIter = NamedMapIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        NamedMapIterator {
            inner: self.inputs.into_values(),
        }
    }
}


