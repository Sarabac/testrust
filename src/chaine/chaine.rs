use std::mem;
use std::ops::DerefMut;

pub struct Chaine<T> {
    pub next_chaine: Option<Box<Chaine<T>>>,
    pub content: T,
}

impl<T> Chaine<T> {
    pub fn new(obj: T) -> Self {
        Self {
            next_chaine: None,
            content: obj,
        }
    }

    pub fn next(&mut self) -> Option<&mut Self> {
        self.next_chaine.as_deref_mut()
    }

    pub fn reverse(self) -> Self {
        let mut left: Option<Box<Chaine<T>>> = None;
        let mut right: Option<Box<Chaine<T>>> = Some(Box::new(self));
        loop {
            match &mut right {
                None => break,
                Some(curr) => {
                    mem::swap(&mut curr.deref_mut().next_chaine, &mut left);
                    mem::swap(&mut left, &mut right);
                }
            }
        }
        *right.unwrap()
    }

    pub fn reverse2(self) -> Self {
        let mut left: Option<Box<Chaine<T>>> = None;
        let mut right: Option<Box<Chaine<T>>> = Some(Box::new(self));
        while let Some(curr) = &mut right {
            mem::swap(&mut curr.next_chaine, &mut left);
            mem::swap(&mut left, &mut right);
        }
        *right.unwrap()
    }

    pub fn len(&self) -> usize {
        let mut counter: usize = 1;
        let mut ref_chaine = self;
        while let Some(next_chaine) = ref_chaine.next_chaine.as_deref() {
            counter += 1;
            ref_chaine = next_chaine
        }
        counter
    }
}

impl<T: Default> Default for Chaine<T> {
    fn default() -> Self {
        Self {
            next_chaine: None,
            content: T::default(),
        }
    }
}
