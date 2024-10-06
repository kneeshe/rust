//codigo para treinar leituras de input, conversão de valor e função

use std::io; //chama a lib de input, output

fn main() {
    println!("Digite um numero: "); //solicita ao usuario um numero

    let mut num1 = String::new(); //cria uma string vazia
    io::stdin().read_line(&mut num1).expect("erro");  //le o input do usuario e refencia a variavel de string
    let num1: i32 = num1.trim().parse().expect("não é numero"); //converte a string para int. trim remove os espaços

    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("erro");
    let num2: i32 = num2.trim().parse().expect("não é numero");

    calc(num1, num2);

}

fn calc(x: i32, y:i32) { //função para calcular as entradas. criei parametros de int
    let calculo = x + y; //variavel que soma os valores de parametro
    println!("A soma de {x} e {y} é {calculo}"); //imprime a soma

    let calculo = x - y; //shadowing da variavel acima. alterei o comportamento dela
    println!("A subtração de {x} e {y} é {calculo}");

    let calculo = x * y;
    println!("A multiplicação de {x} e {y} é {calculo}");

    let calculo = x / y;
    println!("A divisão de {x} e {y} é {calculo}");

    let calculo = x % y;
    println!("O resto de {x} e {y} é {calculo}");
}
