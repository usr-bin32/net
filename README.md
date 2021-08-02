# Echo
Este projeto consiste em uma aplicação cliente-servidor que implementa a função de [`echo`](https://en.wikipedia.org/wiki/Echo_(command)).

## Projeto
Projeto

## Compilação

> [Binários pré-compilados para Windows x64 podem ser encontrados na página de _releases_](https://github.com/usr-bin32/net/releases).

O projeto foi implementado utilizando a linguagem [Rust](https://www.rust-lang.org/). Para compilar os binários de saída, é necessário executar o `rustup` através [deste link](https://www.rust-lang.org/learn/get-started), que instalará o compilador do Rust e seu gerenciador de build e pacotes (`cargo`), bem como outros componentes essenciais. 

Após isso, basta executar o seguinte comando na pasta raíz do projeto:
```
cargo build
```
Para compilar com otimizações, basta executar:
```
cargo build --release
```

Os binários são gerados na pasta `target`. Para mais informações, basta consultar o a página do Rust e do Cargo no link anterior. 

### Testes
Alguns testes de software foram implementados no projeto. Para verificá-los, basta executar o seguinte comando do Cargo:
```
cargo test
```

## Uso
Para utilizar a aplicação, é necessário, primeiro, executar o processo servidor através do binário `server`. O servidor escutará e servirá um ou mais clientes locais, imprimindo na saída quando houver conexão ou desconexão de novos clientes. Ao executá-lo, deve-se visualizar a seguinte mensagem.
```
Server started at 127.0.0.1:7878
```

Em seguida, basta rodar os programas clientes através do binário `client`. O cliente fornece uma interface via console e fornece dois comandos: `echo [message]` e `exit`. A interação com o programa é ilustrada a seguir.

```
Connected to the server
echo Hello!
>> Hello!
echo How are you?
>> How are you?
exit
Finishing connection...
```
