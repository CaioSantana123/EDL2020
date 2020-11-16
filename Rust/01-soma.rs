use std::io;

fn main(){
	let n1 : u32;
	let n2 : u32;				//Declarando duas variáveis imutáveis
	let mut input = String::new();		//String mutável que servirá de auxilio para entrada e saída
	match io::stdin().read_line(&mut input){
		Ok(__) => {},
		Err(_e) => {println!("Erro na leitura de dados\n")},      //Verificando caso haja algum erro na leitura
											}
	n1 = input.trim().parse::<u32>().unwrap();
	input = String::new();
	match io::stdin().read_line(&mut input){
		Ok(__) => {},
		Err(_e) => {println!("Erro na leitura de dados\n")}
											}
	n2 = input.trim().parse::<u32>().unwrap();
	println!("O resultado é : {}",n1+n2);
}
