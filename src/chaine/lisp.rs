use std::mem;

#[derive(Debug, Default)]

pub enum Lisp<T> {
    Cons(T, Box<Lisp<T>>),
    #[default]
    Nils,
}

impl<T> Lisp<T> {
    pub fn new(value: T) -> Self {
        Lisp::Cons(value, Box::new(Lisp::Nils))
    }

    pub fn append(&mut self, elem: T) -> &mut Self {
        match self {
            Lisp::Cons(_, ref mut next) => next.append(elem),
            Lisp::Nils => {
                let mut new_obj = Lisp::new(elem);
                mem::swap(self, &mut new_obj);
                self
            }
        }
    }

    pub fn prepend(self, elem: T) -> Self {
        Lisp::Cons(elem, Box::new(self))
    }

    pub fn iter(&self) -> LispIter<T> {
        LispIter(Box::new(&self))
    }
}

impl<T> IntoIterator for Lisp<T> {
    type IntoIter = LispIntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        LispIntoIter(Box::new(self))
    }
}

impl<A> FromIterator<A> for Lisp<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut result: Lisp<A> = Lisp::Nils;
        iter.into_iter().for_each(|elem| {
            result.append(elem);
        });
        result
    }
}

pub struct LispIntoIter<T>(Box<Lisp<T>>);

impl<T> Iterator for LispIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match mem::take(self.0.as_mut()) {
            Lisp::Cons(val, next_lisp) => {
                self.0 = next_lisp;
                Some(val)
            }
            Lisp::Nils => None,
        }
    }
}

pub struct LispIter<'a, T>(Box<&'a Lisp<T>>);

impl<'a, T> Iterator for LispIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match *self.0 {
            Lisp::Cons(val, next_lisp) => {
                self.0 = Box::new(next_lisp.as_ref());
                Some(val)
            }
            Lisp::Nils => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    struct Setup {
        l1: Lisp<String>,
        l2: Lisp<String>,
        l3: Lisp<String>,
        l4: Lisp<String>,
        elem1: String,
        elem2: String,
    }

    impl Default for Setup {
        fn default() -> Self {
            Self {
                l1: Lisp::new("hello".into()),
                l2: Lisp::new("word".into()),
                l3: Lisp::new("!".into()),
                l4: Lisp::default(),
                elem1: "quoi".into(),
                elem2: "Ha".into(),
            }
        }
    }

    #[test]
    fn value_borrow() {
        let s: Setup = Default::default();

        let mut vecteur = vec![s.l1, s.l2];

        let res: Result<_, ()> = Ok(s.l3);

        let mut k = [11, 5, 3];
        let p = k.as_mut();
        //vecteur.int
    }

    #[test]
    fn append() {
        let mut s = Setup::default();
        s.l1.append(s.elem1).append(s.elem2);
        let result: Vec<String> = s.l1.into_iter().collect();
        assert_eq!(vec!["hello", "quoi", "Ha"], result);
    }
}
