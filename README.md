# Gerador de Senha Aleatória

Este é um projeto simples em Rust que gera uma senha aleatória com base no tamanho fornecido pelo usuário. A senha gerada será composta por letras (maiúsculas e minúsculas), números e caracteres especiais.

<div align="center">
    <img src=https://github.com/JoaoPedroRMatias/Gerador_De_Senhas_ShellScript/assets/100814579/4d41a246-ea05-4dff-b285-81fad7daa1d9>
</div>

## Funcionalidades

- O programa solicita ao usuário o tamanho da senha que deseja gerar (deve ser superior a 12 caracteres).
- A senha gerada é composta por:
  - Letras maiúsculas e minúsculas (A-Z, a-z)
  - Números (0-9)
  - Caracteres especiais (`#`, `$`, `%`, `&`, `@`, `!`, `?`, `*`, `+`, `-`)
- A senha gerada é exibida no console.

## Como rodar

1. Compile o projeto:
   ```bash
   cargo build --release
   ```

2. Execute o programa:
   ```bash
   cargo run
   ```

3. O programa irá pedir que você insira o tamanho da senha (deve ser maior que 12). Após isso, a senha será gerada e exibida no terminal.

### Exemplo de execução
   ```bash
    Digite o tamanho que deseja a senha: 
    20
    Senha gerada: C8*dA+8hG5#1fXnK9!
   ```
