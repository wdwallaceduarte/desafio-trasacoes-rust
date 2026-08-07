use std::io;

fn main() {
    // Lê a linha de entrada do usuário
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler entrada");

    // Divide a entrada em partes e faz o parse dos valores
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 {
        println!("Entrada inválida");
        return;
    }

    let saldo: i32 = parts[0].parse().expect("Saldo inválido");
    let operacao = parts[1];
    let valor: i32 = parts[2].parse().expect("Valor inválido");

    match operacao {
        "deposit" => {
            let novo_saldo = saldo + valor;
            println!("{}", novo_saldo);
        }
        "withdraw" => {
            if valor > saldo {
                println!("Saldo insuficiente");
            } else {
                let novo_saldo = saldo - valor;
                println!("{}", novo_saldo);
            }
        }
        _ => {
            println!("Operação inválida");
        }
    }

    // TODO: Verifique se a entrada possui exatamente 3 partes (saldo, operação, valor)

    // Dica: Use match para tratar as operações "deposit" e "withdraw"
    // Se for "deposit", some o valor ao saldo e imprima o resultado
    // Se for "withdraw", verifique se há saldo suficiente antes de subtrair e imprimir
    // Caso contrário, imprima "Insufficient funds"
}
