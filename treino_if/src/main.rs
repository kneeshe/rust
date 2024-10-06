//codigo para treinar if e match
//comparei strings que vieram via input
//usei tanto o metodo if e match - if let sera estudado mais para frente

use std::io; //lib para ler o input

fn main() {
    let mut nome = String::new(); //cria uma var de string vazia

    println!("diga seu nome: ");
    io::stdin().read_line(&mut nome).expect("i"); //le o input e associa a var nome
    
    let nome = nome.as_str().trim(); //var que converte a string para str e recorta os espaços

    //chamada das funções
    com_if(nome);
    com_match(nome);
    com_match_aninhado(nome);
    
}

fn com_if(name: &str) { //faz a comparação usando if. o paramentro precisa ser &str, logo, é preciso fazer o trim na String
    if name == "optimus" { //compara a variavel name, ja convertida e recortada
        println!("oi, autobot");
    }
    else {
        println!("oi, {name}");
    }
}

fn com_match(name: &str) { //comparação com match. é preciso fazer o trim na variavel que vai no parametro
    match name { //compara a variavel name, ja convertida e recortada
        "optimus" => println!("oi, autobot"), //se bater com o termo, imprime
        _ => println!("oi, {name}") //se não bater com o termo acima e for qualquer outra coisa, imprime
    }
}

fn com_match_aninhado(name: String) { //não faz o trim fora da função, mas sim dentro
    let comparacao = match name.as_str().trim() { //converte o string em &str, faz o trim e compara
        "optimus" => println!("oi, autobot"),
        _ => println!("oi, {name}"),
    };
}
