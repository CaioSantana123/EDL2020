fn fun() -> &i32{
    let y : i32 = 10;      //função que retorna endereço de memória que já foi desalocado
    return &y;
}

fn main(){
    let p = fun();
    println!("{}",p);
}
