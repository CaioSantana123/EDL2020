fn main(){
	let mut x: i32=1;
	while x < 1000{
		println!("The number is {}", x);
		x+=1;
		if x==100{
			break;
		}
	}
}