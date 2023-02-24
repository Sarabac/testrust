use std::mem;

struct Nb(pub i32);

impl Nb {
    pub fn add1(&mut self) -> &mut Self {
        self.0 += 1;
        self
    }

    pub fn autre_add(&mut self) -> &mut Self {
        let autre = self;
        autre.0 += 1;
        autre
    }

    pub fn change(&mut self) -> &mut Self {
        mem::swap(self, &mut Nb(10));
        self
    }
}

#[cfg(test)]
mod test {
    use super::Nb;

    #[test]
    fn prems() {
        let mut a = Nb(52);
        a.change().add1();
        assert_eq!(11, a.0);
    }
}
