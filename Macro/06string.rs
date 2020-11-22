use std::io;
fn main(){
	let mut input=String::new();
	let i;
	let c1;
	let c2;
	match io::stdin().read_line(&mut input){
		Ok(__)=>{},
		Err(_e)=>{println!("Erro na leitura!!!")},
	}
	i=input;
	input=String::new();
	match io::stdin().read_line(&mut input){
		Ok(__)=>{},
		Err(_e)=>{println!("Erro na leitura!!!")},
	}
	c1=input;
	input=String::new();
	match io::stdin().read_line(&mut input){
		Ok(__)=>{},
		Err(_e)=>{println!("Erro na leitura!!!")},
	}
	c2=input;
	print!("{} {} {}",i,c1,c2);
	for c in i.chars(){
		print!("{}",c);
		if c1==c{
			break;
		}
	}
}
