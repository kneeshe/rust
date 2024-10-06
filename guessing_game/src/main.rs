use std::io; //invoca a lib padrão de input, output
use rand::Rng; //chama a lib da dependencia rand
use std::cmp::Ordering; //comparação entre dois valores
use colored::*; //lib para cores

fn main() {
    println!("Guess the number!");
    
    let secret_number:i32 = rand::thread_rng().gen_range(1..101); //gera um numero aleatorio de 1 a 100
    println!("The secret number is {secret_number}");

    loop { //inicia um loop

    println!("Please, input your guess: ");
    let mut guess = String::new(); //variavel para ler um input de string. new é uma função associada a função String - retorna uma string vazia
    io::stdin().read_line(&mut guess).expect("Failed to read line"); //le o valor digitado pelo usuario, joga na variavel guess e, caso de algum erro, retorna a mensagem

    let guess: i32 = match guess.trim().parse() { //converte a var guess, que é string, para integer
        Ok(num) => num, //valida que o usuario insere um numero
        Err(_) => continue, //se for texto, ou qualquer outra entrada que não seja numero, ele ignora
    };

    println!("You guessed: {guess}"); //imprime na tela a variavel

    match guess.cmp(&secret_number) { //compara a variavel guess com a secret number
        Ordering::Less => println!("{}", "Too small".red()), //red é a cor vermelha. se o usuario errar, o texto sai vermelho
        Ordering::Greater => println!("{}", "Too big".red()),
        Ordering::Equal => {
            println!("{}", "You win".green()); //se ele acertar, sai verde
            break; //encerra o loop
        }
    }   
    }
}