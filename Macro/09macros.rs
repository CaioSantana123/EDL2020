use std::collections::BTreeMap;
use std::collections::HashMap;
macro_rules! create{// declaração da macro
	//o $ indica as variáveis que serão utilizadas na macro. O ty depois dos : é o ty e expr indica que pode ser uma expressão
	($typ: ty; $($key: expr => $value: expr),*) => { 
		{
			let mut s = <$typ>::new();//Aqui um novo tipo será criado
			$(s.insert($key, $value);)* //insert é uma função dentro das classes BTreeMap e HashMap
			s
		}
	};
}

fn main(){
	//Aqui o tipo é BTreeMap, mas pode ser HashMap ou outro tipo de classe ou struct que possua uma função insert semelhante a que eu usei.
	let map = create!(BTreeMap<&str, i32>; "Lucas" => 12, "Leonardo" => 21, "Marcos" => 24, "Ana" => 10);
	for(key,values) in &map{
		println!("Name: {} -> Age: {}", key, values);
	}
}
