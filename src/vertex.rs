use glm::vec4;


#[derive(Copy, Clone)]
pub struct Vertex {
	pos : glm::Vec4
}

impl Vertex {
	pub fn new(x : f32, y : f32, z : f32) -> Self {
		Vertex {
			pos : vec4(x, y, z, 1.0)
		}
	}
	pub fn from_vec(vec : glm::Vec4) -> Self {
		Vertex {
			pos : vec
		}
	}

	pub fn x(&self) -> f32{
		self.pos.x as f32
	}

	pub fn y(&self) -> f32 {
		self.pos.y as f32
	}

	pub fn z(&self) -> f32 {
		self.pos.z as f32
	}


	pub fn perspective_divide(&self) -> Self {
		let v = glm::vec4(
			self.pos.x / self.pos.w,
			self.pos.y / self.pos.w,
			self.pos.z / self.pos.w,
			self.pos.w
		);
		Self::from_vec(v)
	}

	pub fn transform(&self, m : glm::Mat4) -> Self {
		Self::from_vec(m * (self.pos))	
	}
}