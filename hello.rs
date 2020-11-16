macro_rules! hello{//declaração da macro
	()=>{
			print!("Hello World\n");
		}
}

fn main(){
	hello!();
}
