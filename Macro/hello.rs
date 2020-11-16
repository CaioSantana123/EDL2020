macro_rules! hello{//declaração da macro
	()=>{	   //começo do bloco de instruções
			print!("Hello World\n");
		}
}

fn main(){
	hello!();//chamada da macro na main
}
