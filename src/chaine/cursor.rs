use super::chaine::Chaine;

struct SemiCursor<'a, T> {
    target: &'a mut Chaine<T>,
    index: usize,
}

struct Cursor<'a, T> {
    left: SemiCursor<'a, T>,
    right: SemiCursor<'a, T>,
}

impl<'a, T> SemiCursor<'a, T> {
    pub fn new(target: &'a mut Chaine<T>, index: usize) -> Self {
        Self { target, index }
    }
}
