# 💰 Desafio - Processamento de Transações Bancárias

## 📖 Descrição

Você foi contratado como desenvolvedor júnior para o **Banco Digital Futuro**, uma fintech inovadora que busca automatizar o processamento de operações financeiras básicas.

Seu primeiro desafio é criar um módulo simples para processar transações bancárias, garantindo que depósitos e saques sejam corretamente aplicados ao saldo de uma conta digital.

O sistema deve receber uma operação (`deposit` ou `withdraw`) e um valor, atualizar o saldo inicial e informar o novo saldo. Caso o saque seja maior que o saldo disponível, o programa deve exibir uma mensagem de erro apropriada.

> **Importante:** Utilize apenas a biblioteca padrão da linguagem, sem bibliotecas externas.

---

## 🎯 Objetivo

Implemente um programa que leia três valores:

- Saldo inicial da conta (inteiro não negativo);
- Tipo da operação (`deposit` ou `withdraw`);
- Valor da operação (inteiro positivo).

### Regras

- Se a operação for **`deposit`**, some o valor ao saldo.
- Se a operação for **`withdraw`**, subtraia o valor apenas se houver saldo suficiente.
- Caso contrário, exiba:

```text
Insufficient funds
```

---

## 📥 Entrada

Uma única linha contendo três valores separados por espaço:

```text
<saldo_inicial> <operacao> <valor>
```

Onde:

- `saldo_inicial` → número inteiro não negativo.
- `operacao` → `deposit` ou `withdraw`.
- `valor` → número inteiro positivo.

### Exemplo

```text
100 deposit 50
```

---

## 📤 Saída

- Se a operação for realizada com sucesso, imprima o novo saldo.
- Caso um saque seja maior que o saldo disponível, imprima:

```text
Insufficient funds
```

---

## 📝 Exemplos

A tabela abaixo apresenta exemplos de entrada e saída:

| Entrada | Saída |
|---------|-------|
| `100 deposit 50` | `150` |
| `200 withdraw 80` | `120` |
| `50 withdraw 100` | `Insufficient funds` |
| `0 deposit 30` | `30` |

---

## ✔️ Resumo da Lógica

| Operação | Resultado |
|----------|-----------|
| `deposit` | Soma o valor ao saldo. |
| `withdraw` | Subtrai o valor apenas se houver saldo suficiente. |
| Saldo insuficiente | Exibe `Insufficient funds`. |

---

## 🚀 Exemplo

### Entrada

```text
200 withdraw 80
```

### Saída

```text
120
```

---

## Testes

A tabela abaixo demonstra o funcionamento do programa com os exemplos do enunciado:

| Entrada           | Caminho no código                                      | Saída                |
|--------------------|--------------------------------------------------------|-----------------------|
| `100 deposit 50`   | `parts.len() == 3` ✓ → match `"deposit"` → 100 + 50    | `150` ✓               |
| `200 withdraw 80`  | match `"withdraw"` → 80 ≤ 200 → 200 - 80                | `120` ✓               |
| `50 withdraw 100`  | match `"withdraw"` → 100 > 50                           | `Insufficient funds` ✓ |
| `0 deposit 30`     | match `"deposit"` → 0 + 30                              | `30` ✓                |

Todos os casos de teste do enunciado foram validados com sucesso ✅