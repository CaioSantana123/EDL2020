fn sum(x: f32, y: f32) -> f32{
	x + y
}
fn sub(x: f32, y: f32) -> f32{
	x - y
}
fn mult(x: f32, y: f32) -> f32{
	x * y
}
fn div(x: f32, y: f32) -> f32{
	x / y
}
fn main(){
	let mut x = sum(10.5,3.4);
	let mut x = sub(10.5,3.4);
	let mut x = mult(10.5,3.4);
	let mut x = div(10.5,3.4);
	print!("Soma 10.5 + 3.4: {}\n", x);
	print!("Subt 10.5 - 3.4: {}\n", x);
	print!("Mult 10.5 * 3.4: {}\n", x);
	print!("Div 10 / 3.4: {}\n", x);
}