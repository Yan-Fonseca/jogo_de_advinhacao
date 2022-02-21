
extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1,101); // Gera um número entre 1(incluído) e 101(não incluído).

    loop {
        println!("Digite o seu palpite.");
        
        let mut palpite = String::new();

        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler a entrada!");
        
        
        //trim() irá eliminar todos os espaços em branco da String.
        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Você disse {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            },
        }
    }
    
}
