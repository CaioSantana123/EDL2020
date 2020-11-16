use std::collections::BTreeMap;
use std::collections::HashMap;
macro_rules! create{
	($typ: ty; $($key: expr => $value: expr),*) => {
		{
			let mut s = <$typ>::new();
			$(s.insert($key, $value);)*
			s
		}
	};
}

fn main(){
	let map = create!(BTreeMap<&str, i32>; "Lucas" => 12, "Leonardo" => 21, "Marcos" => 24, "Ana" => 10);
	for(key,values) in &map{
		println!("Name: {} -> Age: {}", key, values);  
	}
}