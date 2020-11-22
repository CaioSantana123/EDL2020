fn sum(x: f32, y: f32) -> f32{//função que recebe 2 variáveis do tipo f32(float de 32 bits) e realiza a operação
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
	print!("Soma 10.5 + 3.4: {}\n", sum(10.5,3.4)); //chamada das funções
	print!("Subt 10.5 - 3.4: {}\n", sub(10.5,3.4));
	print!("Mult 10.5 * 3.4: {}\n", mult(10.5,3.4));
	print!("Divisao 10 / 3.4: {}\n", div(10.5,3.4));
}
