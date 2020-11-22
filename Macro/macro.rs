use std::collections::HashMap;
macro_rules! vet{
	($di: ident -> [$start: expr ; $end: expr], $cond: expr) => {
		{
			let mut map:HashMap<i32,i32>=HashMap::new();
			for num in $start..$end{
				if $cond(num){
					map.insert(num-1,num);
				}
			}
			map
		}
	};
}
fn par(x:i32) -> bool{
	x%2==0
}
fn imp(x:i32) -> bool{
	x%2!=0
}
fn main(){
	let v = vet![x -> [1;10], par];
	print!("HashPar: {:?}\n", v);
	let a = vet![y -> [1;10], imp];
	print!("HashImpar: {:?}",a);
}