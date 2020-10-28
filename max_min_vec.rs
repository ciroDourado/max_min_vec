use std::io::{ stdin, stdout, Write };
use std::process::Command;
const MAX: usize = 5;

fn main() {
    clear_terminal();
    let mut input = String::new();
    let mut numero_como_str: &str;
    
    let mut nao_quer_sair: bool = true;
    while nao_quer_sair {
    
        let mut vetor: [Option<i32>; MAX] = [None; MAX];
        let mut i: usize = 0;
        while i < vetor.len() {
            print!("Dê um número, ou 'sair' para finalizar o programa.\n");
            print!("{}. ", i+1); flush_this();
            ler_string(&mut input);
            clear_terminal();
            
            numero_como_str = match input.trim() {
                "sair" => {nao_quer_sair = false; break},
                retorna_string => retorna_string,
            };
            
            vetor[i] = match numero_como_str.parse::<i32>() {
                Ok(numero) => Some(numero),
                Err(_) => {print!("Um número, por favor\n"); continue},
            };
            i += 1;
        }
        // maneira mais eficiente, assumindo que nenhum valor ou slice
        // foi retirado (setado como None)
        match vetor[0] {
            Some(_) => {
                // dei preferência para o uso de unwrap, pois neste ponto 
                // nunca será None, sendo desnecessário tratamento de outros
                // erros. Caso use uma destas funções, atente aos casos de:
                // algum valor ser retirado antes do último, passar algum
                // vetor vazio, dentre outros.
                print!("Máximo do vetor = {}\n", maximo_ate_none(vetor).unwrap());
                print!("Mínimo do vetor = {}\n", minimo_ate_none(vetor).unwrap());
            },
            None => print!("Não foram dados valores\n"),
        }
    } // while nao_quer_sair
    print!("Programa finalizado.\n");
    print!("Até mais C: \n\n");
}

fn ler_string(input: &mut String) {
    print!(">>> "); flush_this();
    (*input).clear();
    stdin().read_line(input).expect("input!");
}

// caso encontre algo, retorna Some(valor); no contrário, None;
// percorre o vetor inteiro, parando apenas caso encontre uma casa "vazia"
fn maximo_ate_none(vetor: [Option<i32>; MAX]) -> Option<i32> {
    let mut maior_do_vetor: Option<i32> = vetor[0];
    let mut index: usize = 0;
    while index < vetor.len() && vetor[index] != None {
        if maior_do_vetor.unwrap() < vetor[index].unwrap() {
            maior_do_vetor = vetor[index];
        }
        index += 1;
    }
    maior_do_vetor
}

fn minimo_ate_none(vetor: [Option<i32>; MAX]) -> Option<i32> {
    let mut menor_do_vetor: Option<i32> = vetor[0];
    let mut index: usize = 0;
    while index < vetor.len() && vetor[index] != None {
        if menor_do_vetor.unwrap() > vetor[index].unwrap() {
            menor_do_vetor = vetor[index];
        }
        index += 1;
    }
    menor_do_vetor
}

fn flush_this() {
    stdout().flush().expect("output!");
}

fn clear_terminal() {
    let mut call = if cfg!(target_os = "windows") {
        Command::new("cls")
    } else { Command::new("clear") };
    call.status().expect("syscall!");
}
