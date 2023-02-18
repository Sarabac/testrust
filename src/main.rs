#[derive(Debug)]
struct Corps {
    poids: f64,
}

struct Caillou(f64);

struct Carton(Box<dyn Massif>, Box<dyn Massif>);

pub trait Massif {
    fn get_masse(&self) -> f64;
}

impl Massif for Caillou {
    fn get_masse(&self) -> f64 {
        self.0
    }
}

impl Into<f64> for Caillou {
    fn into(self) -> f64 {
        self.0
    }
}

impl Massif for Corps {
    fn get_masse(&self) -> f64 {
        self.poids
    }
}

impl Massif for Carton {
    fn get_masse(&self) -> f64 {
        self.0.get_masse() + self.1.get_masse()
    }
}

impl Carton {
    fn new<A, B>(a: A, b: B) -> Self
    where
        A: 'static + Massif,
        B: 'static + Massif
    {
        Carton(Box::new(a), Box::new(b))
    }
}

fn main() {
    println!("Hello, world!");

    let vache = Corps { poids: 50f64 };
    let banane = Corps { poids: 60f64 };
    let rocher = Caillou(145f64);

    let boite = Carton::new(banane, rocher);
    let camion = Carton::new(boite, vache);

    let total = camion.get_masse();
    dbg!(total);
}
