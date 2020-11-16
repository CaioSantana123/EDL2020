use std::collections::BTreeMap;
use std::collections::HashMap;
macro_rules! create{//declaração da macro
	//$ é uma daclaração de uma variável que será usada na macro
	//o ty seguido pelos : indica um tipo, o expr indica uma expressão
	($typ: ty; $($key: expr => $value: expr),*) => {// a => indica o início do bloco de instruções
		{
			let mut s = <$typ>::new();
			$(s.insert($key, $value);)*
			s
		}
	};
}

fn main(){
	let map = create!(BTreeMap<&str, i32>; "Lucas" => 12, "Leonardo" => 21, "Marcos" => 24, "Ana" => 10);// chamada da macro na main
	for(key,values) in &map{
		println!("Name: {} -> Age: {}", key, values);  
	}
}
