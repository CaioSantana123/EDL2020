pub struct Circulo{//criação da estrutura círculo
	raio : f32,
}

impl Circulo{
	pub fn new(raio: f32) -> Circulo{//construtor
		Circulo{
			raio: raio,
		}
	}
	pub fn calcula_raio(&self) -> f32{//cálculo do raio
		return 2.0 * self.raio * 3.14;
	}
}


fn main(){
	let c = Circulo::new(4.2);//criação do objeto
	print!("Area: {}", c.calcula_raio());
}

