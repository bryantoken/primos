use std::io;

fn main() {
    println!("Digite o número primo");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Falha ao ler a linha");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, digite um número inteiro válido!");
            return;
        }
    };
    if input < 2 {
        println!("O número primo deve ser maior ou igual a dois");
        return;
    }
    println!("Você digitou: {}", input);
    calcula_primo(input);
}

fn calcula_primo(n: i32){
    let mut vetor: Vec<i32> = Vec::new();
    for d in 1..=n{
        if n % d == 0 {
            vetor.push(d);
        }
    };
    if vetor.len() <= 2 {
        println!("O número {} é primo!", n);
    } else {
        println!("O número {} não é primo!", n);
    }
}
