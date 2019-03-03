use rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
pub struct Star {
    pub x : f64,
    pub y : f64,
    pub z : f64
}

impl Star {

    pub fn new() -> Self {
        let x: f64 = thread_rng().gen_range(-0., 1.);
        let y: f64 = thread_rng().gen_range(-0., 1.);
        let z: f64 = thread_rng().gen_range(-0., 1.);

        Star { x : x, y: y, z : z }
    }
}

pub struct StarField {
    pub stars: Vec<Star>

}

impl StarField {
    pub fn new_star_field() -> StarField {

        let mut stars = Vec::new();
        for i in 0..1024 {
            stars.push(Star::new());
        }
        return StarField { stars : stars };
    }

}

