use testrust::composite::*;

fn main() {
    println!("Hello, world!");

    let vaches = Corps { poids: 50f64 };
    let banane = Corps { poids: 60f64 };
    let rocher = Caillou(145f64);

    let boite = Carton::new(banane, rocher);
    let camion = Carton::new(boite, vaches);

    let total = camion.get_masse();
    dbg!(total);
}
