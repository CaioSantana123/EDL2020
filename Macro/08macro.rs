use std::collections::HashMap;
macro_rules! vet{ //criação da macro
	($di: ident -> [$start: expr ; $end: expr], $cond: expr) => { //assinatura da macro (identificador -> [expressão; expressão], expressão)
		{
			let mut map:HashMap<i32,i32>=HashMap::new(); //criação da classe HashMap
			for num in $start..$end{ //repetições podem ser utilizadas dentro das macros
				if $cond(num){
					map.insert(num-1,num); //insert é uma função dentro da classe HashMap
				}
			}
			map
		}
	};
}
fn par(x:i32) -> bool{//função booleana que checa se um número é par ou não
	x%2==0
}
fn imp(x:i32) -> bool{//função booleana que checa se um número é ímpar ou não
	x%2!=0
}
fn main(){
	let v = vet!(x -> [1;10], par);//chamada da macro usando a função par para montar a hash
	print!("HashPar: {:?}\n", v);
	let a = vet!(y -> [1;10], imp);//chamada da macro usando a função imp para montar a hash
	print!("HashImpar: {:?}",a);
}
