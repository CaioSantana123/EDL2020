pub struct Circulo{
	raio : f32,
}

impl Circulo{
	pub fn new(raio: f32) -> Circulo{
		Circulo{
			raio: raio,
		}
	}
	pub fn calcula_raio(&self) -> f32{
		return 2.0 * self.raio * 3.14;
	}
}


fn main(){
	let c = Circulo::new(4.2);
	print!("Area: {}", c.calcula_raio());
}

