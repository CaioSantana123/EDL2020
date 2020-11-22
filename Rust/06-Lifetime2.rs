struct Pessoa{
	nome: String,
	idade: u16,
}

fn main(){
	let  mut v : Vec<Pessoa> = Vec::new();
	v.push(Pessoa{nome : "Fernando".to_string(),idade : 39,});
	v.push(Pessoa{nome : "Bianca".to_string(),idade : 45,});
	v.push(Pessoa{nome : "Joao".to_string(),idade : 48,});
	let maiorIdade = pegar_maior_idade(&v);
	println!("{}",maiorIdade);
	}

fn pegar_maior_idade(v: &Vec<Pessoa>) -> u16{ 
	let mut i : Vec<u16> = Vec::new();
	for p in v{
		i.push(p.idade);
			   }	   
	return maior(&i);
}

fn maior(v: &Vec<u16>) -> u16{
	let mut m: &u16 = v.first().unwrap();
	for i in v{
		if (i > m){
			m = i;
		}
	}
	return *m;
}
