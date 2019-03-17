use glm::vec4;


#[derive(Copy, Clone)]
pub struct Vertex {
	pos : glm::Vec4
}

impl Vertex {
	pub fn new(x : f32, y : f32) -> Self {
		Vertex {
			pos : vec4(x, y, 0.0, 1.0)
		}
	}

	pub fn x(&self) -> u32{
		self.pos.x.floor() as u32
	}

	pub fn y(&self) -> u32 {
		self.pos.y.floor() as u32
	}
}