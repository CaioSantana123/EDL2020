fn main(){
	let mut x=1;
	while x<10{
		if x%2==0{
			println!("Par: {}",x);
		}else{
			println!("Impar {}", x);
		}
		x+=1;
	}
}