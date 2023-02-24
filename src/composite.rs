#[derive(Debug)]
pub struct Corps {
    pub poids: f64,
}

pub struct Caillou(pub f64);

pub struct Carton(Box<dyn Massif>, Box<dyn Massif>);

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
    pub fn new<A, B>(a: A, b: B) -> Self
    where
        A: 'static + Massif,
        B: 'static + Massif,
    {
        Carton(Box::new(a), Box::new(b))
    }
}
