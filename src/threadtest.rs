use rand::Rng;
use std::{iter::Sum, thread};

pub fn thread_add(mesures: &Vec<f64>) -> f64 {
    mesures
        .chunks(100)
        .map(Vec::<f64>::from)
        .map(|chunk| thread::spawn(move || chunk.iter().sum::<f64>()))
        .map(|joinhandler| joinhandler.join().unwrap_or(0f64))
        .sum::<f64>()
}

pub fn threadtest(mesures: &Vec<f64>) -> f64 {
    mesures.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mainfn() {
        let mut rng = rand::thread_rng();
        let mesures: Vec<f64> = (1..100).map(|_| rng.gen()).collect();
        let normal: f64 = mesures.iter().sum();
        dbg!(normal);
        let avec_thread = thread_add(&mesures);
        dbg!(avec_thread);
    }
}
