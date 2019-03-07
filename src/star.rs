use rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
pub struct Star {
    pub x : f64,
    pub y : f64,
    pub z : f64
}

impl Star {

    pub fn random(spread : f64) -> Self {
        let x: f64 = thread_rng().gen_range(-1.0, 1.) * spread;
        let y: f64 = thread_rng().gen_range(-1.0, 1.) * spread;
        let z: f64 = thread_rng().gen_range(0.0001, 1.) * spread;

        Star { x : x, y: y, z : z }
    }
}

pub struct StarField {
    pub stars: Vec<Star>

}

impl StarField {
    pub fn new_star_field(nb_stars : u32, spread : f64) -> StarField {

        let mut stars = Vec::new();
        for i in 0..nb_stars {
            stars.push(Star::random(spread));
        }
        return StarField { stars : stars };
    }

    // fn move_stars(&mut self, starfield : &mut star::StarField) {
    //     for i in 0..starfield.stars.len() {
    //         let star = starfield.stars[i];
    //         let mut new_star = star::Star { x : star.x, y : star.y, z: star.z - 0.8 };
    //         if new_star.z <= 0.0 {
    //             new_star = star::Star::random();
    //         }
    //         starfield.stars[i] = new_star;
    //     }
    // }

}

