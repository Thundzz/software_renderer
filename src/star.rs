
#[derive(Copy, Clone)]
pub struct Star {
    pub x : i32,
    pub y : i32,
    pub z : i32
}

impl Star {

    pub fn new() -> Self {
        Star { x : 0, y: 0, z : 0 }
    }
}

#[derive(Copy, Clone)]
pub struct StarField {
    pub stars: [Star; 1024]

}

impl StarField {
    pub fn new_star_field() -> StarField {
        return StarField { stars : [Star::new(); 1024] };
    }
}

