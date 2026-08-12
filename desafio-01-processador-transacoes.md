# Desafio 1 — Processador de Transações Bancárias

## Contexto

Você foi contratado como desenvolvedor júnior para o **Banco Digital Futuro**, uma fintech inovadora que busca automatizar o processamento de operações financeiras básicas.

Seu primeiro desafio é criar um módulo simples para processar transações bancárias, garantindo que depósitos e saques sejam corretamente aplicados ao saldo de uma conta digital. O sistema deve receber uma operação (depositar ou sacar) e um valor, atualizar o saldo inicial e informar o novo saldo ou, caso o saque seja maior que o saldo disponível, exibir uma mensagem de erro apropriada.

Este desafio é fundamental para garantir a segurança e a precisão das operações financeiras do banco.

## Tarefa

Implemente um programa que leia três valores:

1. O saldo inicial da conta (um número inteiro não negativo)
2. O tipo de operação (`"deposit"` ou `"withdraw"`)
3. O valor da operação (um número inteiro positivo)

- Se a operação for `"deposit"`, adicione o valor ao saldo.
- Se for `"withdraw"`, subtraia o valor do saldo **apenas se houver saldo suficiente**; caso contrário, exiba `"Insufficient funds"`.

Não utilize bibliotecas externas, apenas a biblioteca padrão da linguagem.

## Entrada

Três valores separados por espaço em uma única linha:

- Saldo inicial (inteiro não negativo)
- Tipo de operação (`"deposit"` ou `"withdraw"`)
- Valor da operação (inteiro positivo)

## Saída

- Se a operação for válida, imprima o novo saldo (um inteiro).
- Se o saque não puder ser realizado por falta de saldo, imprima `"Insufficient funds"`.

## Exemplos

| Entrada           | Saída                |
|--------------------|-----------------------|
| `100 deposit 50`   | `150`                 |
| `200 withdraw 80`  | `120`                 |
| `50 withdraw 100`  | `Insufficient funds`  |
| `0 deposit 30`     | `30`                  |
