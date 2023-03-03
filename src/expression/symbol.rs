use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Elem {
    Sym(String),
    Func(String, Vec<Elem>),
}

macro_rules! expr {
    ($name:ident($($args:expr),*)) => {
        Elem::Func(
             stringify!($name).into(),
             vec![]

        )

    };
    ($name: ident) => {
        Elem::Sym(
            name: stringify!($name).into()
        )
    };

    ($name:ident,$($other:tt),*) => {
        expr!($name), expr!($other),*
    }
}

macro_rules! sym {
    ("faux") => {
        if (line!() % 2 == 0) {
            "pair"
        } else {
            "impaire"
        }
    };
    ($name: ident) => {
        Sym {
            name: stringify!($name).into(),
        }
    };
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn essaie() {

        let y = sym!("faux");

        let k = expr!(a(k(m)));
        dbg!(k);
    }
}
