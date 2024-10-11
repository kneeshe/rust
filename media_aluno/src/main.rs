//codigo para treinar calculo de media e if

use std::io;

fn main() {    
    println!("--- CALCULADORA DE NOTA --- ");

    //variaveis para o input de notas
    let mut nota1 = String::new();
    let mut nota2 = String::new();
    let mut nota3 = String::new();

    //variaveis que leem o input e associam ao var anterior
    println!("Digite a primeira nota: ");
    io::stdin().read_line(&mut nota1).expect("nao é numero valido");
    let nota1: i32 = nota1.trim().parse().expect("nao é numero");   
    if nota1 > 10 || nota1 < 0 {
        println!("Valor invalido");
    }

    

    println!("Digite a segunda nota: ");
    io::stdin().read_line(&mut nota2).expect("nao é numero");
    let nota2: i32 = nota2.trim().parse().expect("nao é numero");
    
    println!("Digite a terceira nota: ");
    io::stdin().read_line(&mut nota3).expect("nao é numero");
    let nota3: i32 = nota3.trim().parse().expect("nao é numero");
    
    let nota_final = (nota1 + nota2 + nota3)/3;

    println!("As notas foram: \n{}, \n{}, \n{} \ne a média final foi {}", nota1, nota2, nota3, nota_final);    

    if nota_final >= 7 {
        println!("Voce foi aprovado");
    }
    else if nota_final <= 6 {
        println!("Voce esta de recuperação");
    }
    else if nota_final < 5 {
        println!("Voce foi reprovado");
    }
}

/*fn calc_media(x: i32, y: i32, z: i32) -> i32 {
    let nota_final = (x + y + z)/3;
    nota_final
}*/