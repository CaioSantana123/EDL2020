macro_rules! tvec{
	($($x:expr),*) => {
		{
			let mut t_vec = Vec::new();
			$(t_vec.push($x);)*
			t_vec
		}
	};
}

fn main(){
	let v = tvec!(1, 2, 3, 4, 5);
	print!("Vetor criado: {:?}", v);
}