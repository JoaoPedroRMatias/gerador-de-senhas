use std::io;
use rand::Rng;

fn pass_len() -> i32 {
    loop {
        println!("Digite o tamanho que deseja a senha: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse::<i32>() {
            Ok(num) => {
                if num > 11 {
                    return num / 2;
                }
                println!("Entrada inválida. Deve ser um NUMERO e ACIMA DE 12 caracteres.");
            }
            Err(_) => println!("Entrada inválida! Por favor, digite um número inteiro."),
        }
    }
}

fn main() {
    let array: [&str; 62] = [
        "A", "a", "B", "b", "C", "c", "D", "d", "E", "e", "F", "f", "G", "g", "H", "h", "I", "i", 
        "J", "j", "K", "k", "L", "l", "M", "m", "N", "n", "O", "o", "P", "p", "Q", "q", "R", "r", 
        "S", "s", "T", "t", "U", "u", "V", "v", "W", "w", "X", "x", "Y", "y", "Z", "z", 
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];
    let array_special: [&str; 10] = ["#", "$", "%", "&", "@", "!", "?", "*", "+", "-"];
    let mut password = String::new();

    let amount = pass_len();

    for _i in 0..amount {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..array.len());  
        let letter = array[random_number];

        let random_special = rng.gen_range(0..array_special.len());
        let special = array_special[random_special];

        password.push_str(letter); 
        password.push_str(special); 
    }

    println!("Senha gerada: {}", password);
}
