macro_rules! tvec{
	($($x:expr),*) => {//criação da macro tvec
		{
			let mut t_vec = Vec::new();//criação do objeto Vec dentro da macro
			$(t_vec.push($x);)*  //repetição da instrução push(função dentro da classe Vec)
			t_vec  //retorna t_vec
		}
	};
}

fn main(){
	let v = tvec!(1, 2, 3, 4, 5); //chamada da macro
	print!("Vetor criado: {:?}", v);
}
