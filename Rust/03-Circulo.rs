struct Circulo{
	raio : f32,                 //Campos da classe círculo
}

impl Circulo{
	pub fn calcula_raio(&self) -> f32{
		return 2.0 * self.raio * 3.14;      //Métodos da classe círculo
	}
}


fn main(){
	let c = Circulo{raio : 4.2};
	println!("{} ",c.calcula_raio());
}

//Este exemplo mostra como é a sintaxe de criar uma classe com métodos em Rust
