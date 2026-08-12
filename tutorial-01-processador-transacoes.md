# Tutorial Passo a Passo — Desafio 1: Processador de Transações Bancárias

> Guia de estudo com a explicação de cada etapa da solução em Rust.
> Baseado no livro oficial: https://rust-br.github.io/rust-book-pt-br/

## Código de partida

```rust
use std::io;
fn main() {
    // Lê a linha de entrada do usuário
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
    // Divide a entrada em partes e faz o parse dos valores
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    // TODO: Verifique se a entrada possui exatamente 3 partes (saldo, operação, valor)
    // Dica: Use match para tratar as operações "deposit" e "withdraw"
    // Se for "deposit", some o valor ao saldo e imprima o resultado
    // Se for "withdraw", verifique se há saldo suficiente antes de subtrair e imprimir
    // Caso contrário, imprima "Insufficient funds"
}
```

---

## Passo 1 — Entendendo a leitura de entrada

```rust
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
let parts: Vec<&str> = input.trim().split_whitespace().collect();
```

- **`let mut input = String::new();`** — cria uma `String` vazia e **mutável**. Em Rust, variáveis são imutáveis por padrão; o `mut` é necessário porque vamos escrever a entrada do usuário dentro dela.
- **`io::stdin().read_line(&mut input)`** — lê uma linha digitada e guarda em `input`.
  - `&mut input` é uma **referência mutável**: em vez de "dar" a posse (*ownership*) da variável para a função, apenas **emprestamos** ela temporariamente para que a função escreva o valor lido ali dentro. Depois da chamada, `input` continua sendo nossa.
- **`.expect("Erro ao ler entrada")`** — trata um possível erro na leitura. Rust obriga a lidar com operações que podem falhar; `expect` interrompe o programa com a mensagem informada caso algo dê errado.
- **`input.trim()`** — remove espaços/quebras de linha do início e fim (como o `\n` do Enter).
- **`.split_whitespace()`** — quebra a string em pedaços, separando por espaços.
- **`.collect()`** — junta os pedaços em um vetor (`Vec`) de fatias de string (`&str`).
- Resultado: para a entrada `"100 deposit 50"`, `parts` vira `["100", "deposit", "50"]`.

---

## Passo 2 — Convertendo os valores de `String` para números

```rust
let saldo: i32 = parts[0].parse().expect("Saldo inválido");
let operacao = parts[1];
let valor: i32 = parts[2].parse().expect("Valor inválido");
```

- `parts[0]`, `parts[1]`, `parts[2]` acessam os elementos do vetor pelo índice.
- **`.parse()`** tenta converter uma string para o tipo indicado (`i32` no caso). Como a conversão pode falhar (ex: texto que não é número), `.parse()` retorna um `Result`, tratado aqui com `.expect(...)`.
- `operacao` permanece como `&str`, pois será usada para comparação de texto (`"deposit"` / `"withdraw"`), não para cálculo.
- Nenhuma dessas variáveis tem `mut`, porque seus valores não precisam mudar depois de criados — reforçando a imutabilidade como padrão em Rust.

---

## Passo 3 — Verificando se a entrada tem 3 partes

```rust
if parts.len() != 3 {
    println!("Entrada inválida");
    return;
}
```

- **`parts.len()`** retorna a quantidade de elementos do vetor.
- Essa verificação precisa vir **antes** das linhas de `.parse()` (Passo 2), porque, se a entrada tiver menos de 3 partes, tentar acessar `parts[2]` causaria um erro de execução (*index out of bounds*) antes mesmo de conseguirmos validar.
- **`return;`** interrompe a execução da função `main` imediatamente.

---

## Passo 4 — Tratando as operações com `match`

```rust
match operacao {
    "deposit" => {
        let novo_saldo = saldo + valor;
        println!("{}", novo_saldo);
    }
    "withdraw" => {
        if valor > saldo {
            println!("Insufficient funds");
        } else {
            let novo_saldo = saldo - valor;
            println!("{}", novo_saldo);
        }
    }
    _ => {
        println!("Operação inválida");
    }
}
```

- **`match operacao { ... }`** compara o valor de `operacao` com cada padrão listado e executa o bloco correspondente.
- **`"deposit" => { ... }`** soma `valor` ao `saldo` e imprime o resultado com `println!("{}", novo_saldo)` — o `{}` é substituído pelo valor da variável.
- **`"withdraw" => { ... }`** verifica primeiro se `valor > saldo`; se for, imprime `"Insufficient funds"`; senão, subtrai e imprime o novo saldo.
- **`_ => { ... }`** é o "coringa": cobre qualquer caso não previsto. O `match` em Rust é **exaustivo** — o compilador obriga a tratar todos os casos possíveis (ou usar `_`), evitando bugs de casos esquecidos.

---

## Código final completo

```rust
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
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
                println!("Insufficient funds");
            } else {
                let novo_saldo = saldo - valor;
                println!("{}", novo_saldo);
            }
        }
        _ => {
            println!("Operação inválida");
        }
    }
}
```

## Conceitos de Rust praticados

- Mutabilidade (`mut`) e imutabilidade por padrão
- Ownership e Borrowing (`&mut`)
- Tratamento de erros com `.expect()`
- Vetores (`Vec<&str>`)
- Parsing de strings para números (`.parse()`)
- Controle de fluxo com `if` e `match`
