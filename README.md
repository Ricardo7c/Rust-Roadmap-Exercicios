# Rust-Roadmap-Exercicios
Exercícios baseados no <a src="https://github.com/Ricardo7c/Rust-Roadmap">Roadmap</a>

Claro! Vou gerar 10 exercícios para cada tópico do roadmap de Rust, mantendo a complexidade e os conceitos em cada estágio do aprendizado.

---

## 🛠️ 1. **Fundamentos do Rust**

1. Crie um programa que imprima “Olá, Rust!” e uma mensagem de boas-vindas.
2. Implemente um programa que declare variáveis para nome, idade e cidade, e as exiba em uma frase completa.
3. Escreva uma função que receba dois números inteiros e retorne a soma deles.
4. Crie um programa que converta quilômetros em milhas, usando uma constante de conversão.
5. Implemente um programa que peça ao usuário para inserir um número e verifique se ele é par ou ímpar.
6. Desenvolva uma função para calcular o fatorial de um número.
7. Escreva um programa que exiba a tabuada de um número fornecido pelo usuário.
8. Crie um jogo de adivinhação onde o programa escolhe um número aleatório e o usuário tenta adivinhar.
9. Faça um programa que calcule a média de três notas inseridas e exiba a mensagem de aprovação ou reprovação.
10. Crie um programa que gere uma sequência de Fibonacci até o N-ésimo número, fornecido pelo usuário.
11. Crie uma função que receba dois números e retorne o maior entre eles.
12. Desenvolva um programa que calcule a área e o perímetro de um círculo, recebendo o raio como entrada.
13. Escreva uma função que verifique se um número é positivo, negativo ou zero.
14. Implemente um programa que conte de 1 a 50 e imprima "Fizz" para múltiplos de 3 e "Buzz" para múltiplos de 5.
15. Crie uma função que receba uma string e retorne-a em letras maiúsculas.
16. Escreva um programa que calcule a média e o desvio padrão de uma lista de números.
17. Implemente um programa que receba uma frase e conte o número de palavras.
18. Crie uma função que inverta os caracteres de uma string.
19. Desenvolva um programa que encontre o maior e o menor valor em um vetor de números inteiros.
20. Escreva um programa que converta um número inteiro em sua representação binária.


---

## 📦 2. **Entendendo a Gestão de Memória e Borrowing**

1. Escreva uma função que receba um `Vec<i32>` e retorne a soma de seus elementos usando referências.
2. Implemente uma função que devolva a quantidade de letras em uma `String`.
3. Crie um programa que simule uma pilha (stack) usando um `Vec`.
4. Desenvolva um programa que conte o número de vogais em uma string e utilize borrowing para evitar cópia de dados.
5. Escreva uma função que recebe uma referência a uma lista de números e retorna o maior valor.
6. Implemente um sistema básico de lista de tarefas (todos) usando um `Vec<String>` e borrowing para modificar a lista.
7. Crie uma função que calcule a soma dos quadrados dos elementos de uma lista usando referências.
8. Desenvolva um programa que simule um banco de dados básico usando borrowing para gerenciar clientes.
9. Escreva uma função que receba um `&mut Vec<i32>` e ordene os números na lista.
10. Implemente uma função que receba uma referência a uma string e devolva a última palavra.
11. Crie uma função que receba um vetor de strings e retorne a maior string.
12. Desenvolva uma função que receba uma string e remova os espaços em branco.
13. Implemente uma função que use borrowing para calcular a média dos valores em uma lista.
14. Crie um programa que simule uma fila (FIFO) usando um `Vec`.
15. Escreva uma função que receba uma lista de inteiros e retorne o segundo maior valor.
16. Desenvolva uma função que use borrowing para alterar o primeiro elemento de um vetor.
17. Implemente um programa que receba um vetor de strings e retorne uma nova lista sem duplicatas.
18. Crie uma função que recebe um `&mut Vec<i32>` e dobra o valor de cada elemento.
19. Escreva um programa que use borrowing para alternar entre letras maiúsculas e minúsculas em uma string.
20. Implemente uma função que usa `lifetimes` para retornar uma substring da string de entrada.

---

## 🧩 3. **Tipos de Dados Complexos**

1. Crie uma `struct` chamada `Livro` com atributos `título`, `autor` e `ano`, e exiba os dados de um livro.
2. Desenvolva uma estrutura `Retângulo` e implemente uma função que calcule sua área.
3. Crie uma `enum` chamada `Status` com opções `Ativo` e `Inativo` e implemente um método de status.
4. Implemente um programa para simular o lançamento de um dado usando `Option` e `Result`.
5. Crie um programa que gerencie um inventário de loja usando `Vec` e `struct`.
6. Escreva uma função que utilize `Option` para retornar o menor valor de um vetor, ou `None` se estiver vazio.
7. Crie um sistema de cadastro que armazene informações de clientes usando structs e enums.
8. Desenvolva uma função que use `Result` para verificar se um número é primo.
9. Implemente uma enum `Veiculo` com variantes `Carro` e `Moto` e crie um método para exibir o tipo de veículo.
10. Construa um programa que use `Option` para manipular uma lista de convidados, onde `None` representa ausência de convidados.
11. Crie uma `struct` `Triangulo` que contenha a base e a altura, e uma função que calcule a área.
12. Desenvolva um programa que utilize uma `struct` `Livro` e implemente uma função para alterar o autor.
13. Crie uma `enum` `StatusPedido` com variantes `Pendente`, `Processando` e `Concluído`, e um método para exibir o estado atual.
14. Implemente uma `struct` `Pessoa` com métodos para definir e exibir os dados.
15. Escreva uma função que utilize `Result` para dividir dois números e retornar um erro se o divisor for zero.
16. Desenvolva um programa que utilize uma `enum` para representar o nível de bateria de um dispositivo.
17. Crie uma função que utilize `Option` para verificar se uma lista está vazia.
18. Implemente uma `struct` `Usuario` com métodos para verificar se o usuário é maior de idade.
19. Escreva uma função que use `Result` para manipular erros em uma operação de leitura de arquivo.
20. Crie uma `enum` `Resposta` para um sistema de perguntas com variantes `Correto` e `Errado`.

---

## 🧵 4. **Funções, Closures e Iteradores**

1. Escreva uma função que receba um vetor de números e devolva apenas os pares.
2. Crie um programa que utilize `map` para transformar um `Vec<i32>` em um `Vec<i32>` onde cada valor é seu quadrado.
3. Implemente uma closure que multiplique dois números e a aplique em uma lista de valores.
4. Desenvolva uma função que use `filter` para remover todos os valores menores que um número X.
5. Crie um programa que use `collect` para converter um vetor de números em um `HashSet`.
6. Escreva uma função que receba uma lista de strings e devolva apenas aquelas que contêm uma substring específica.
7. Implemente uma função que use um iterador para somar os números pares de um vetor.
8. Desenvolva uma closure que filtre números ímpares e use um iterador para aplicar essa closure a uma lista.
9. Crie uma função que use `find` para localizar o primeiro número negativo em uma lista de inteiros.
10. Escreva um programa que use `fold` para calcular o produto de uma lista de inteiros.
11. Crie uma closure que calcule a raiz quadrada de um número e aplique a uma lista.
12. Implemente um programa que utilize uma closure para formatar strings em um vetor.
13. Escreva uma função que use `filter` para selecionar números divisíveis por 3 em uma lista.
14. Desenvolva uma closure que receba duas strings e retorne a concatenação delas.
15. Crie uma função que utilize `map` para converter um `Vec<i32>` para `Vec<String>`.
16. Implemente uma closure que filtre palavras em uma frase com base em uma lista de palavras proibidas.
17. Escreva uma função que use `fold` para calcular o produto dos valores de uma lista.
18. Desenvolva uma closure que converta uma lista de inteiros para uma lista de números elevados ao quadrado.
19. Crie uma função que use `filter` para remover todos os valores maiores que um número X.
20. Escreva um programa que utilize `map` para formatar uma lista de números como strings com duas casas decimais.

---

## 🧱 5. **Programação Orientada a Objetos com Rust**

1. Crie uma trait `Descritivo` com um método `descricao` e implemente-a para um `struct` `Produto`.
2. Implemente uma trait `Transporte` com um método `mover`, e aplique para as structs `Carro` e `Avião`.
3. Escreva um programa que usa polimorfismo para exibir uma descrição para diferentes tipos de `Item`.
4. Crie uma trait `Som` para sons de animais (`Cachorro`, `Gato`), com um método `emitir_som`.
5. Desenvolva uma trait `Operavel` para operações matemáticas básicas em `struct`s que representem formas geométricas.
6. Implemente uma trait `Desconto` com um método que calcule o preço com desconto para um `Produto`.
7. Escreva um programa que utilize trait objects para manipular diferentes tipos de veículos com métodos polimórficos.
8. Crie uma trait `Convertivel` com um método `converter` e aplique-a a `Temperatura` para converter entre Celsius e Fahrenheit.
9. Desenvolva uma trait `Calculavel` para realizar operações aritméticas em structs de tipos numéricos.
10. Construa um sistema de gerenciamento de inventário usando traits e polimorfismo para diferentes tipos de produtos.
11. Crie uma trait `Calculavel` com métodos para `area` e `perimetro`, e implemente para `Retangulo`.
12. Desenvolva uma trait `Taxa` com um método para calcular impostos, e implemente em um `Produto`.
13. Implemente uma trait `Transacao` para processar pagamentos em diferentes moedas.
14. Crie uma trait `Serializavel` que converte structs em JSON e implemente em `Produto`.
15. Escreva uma trait `Desenho` com métodos para `desenhar` e `mover` para figuras geométricas.
16. Implemente uma trait `Operacao` para uma `ContaBancaria` com métodos de depósito e retirada.
17. Crie uma trait `Conectavel` para diferentes tipos de dispositivos e implemente em `Computador` e `Telefone`.
18. Desenvolva uma trait `Desconto` para aplicar descontos variáveis e implemente em `Produto`.
19. Escreva uma trait `Notificavel` que envia notificações, e implemente em `Usuario` e `Administrador`.
20. Crie uma trait `Identificavel` com um método `id` e implemente em `Pessoa` e `Produto`.

---

## ⚙️ 6. **Trabalhando com Concurrency**

1. Crie um programa que inicie duas threads para realizar cálculos simultaneamente.
2. Implemente um sistema de contagem usando threads, onde cada thread conta até um valor fornecido.
3. Desenvolva uma função assíncrona que simule uma chamada de rede com `async` e `await`.
4. Escreva um programa que use threads para calcular a soma de uma lista dividida entre elas.
5. Implemente um canal (`mpsc`) para transmitir mensagens entre duas threads e exibi-las na principal.
6. Crie um programa onde threads diferentes imprimem mensagens em intervalos de tempo distintos.
7. Desenvolva uma aplicação que usa `join` para combinar o resultado de várias threads em uma única lista.
8. Escreva um programa que inicie uma thread para calcular números primos e retorne o resultado para a thread principal.
9. Implemente um sistema onde múltiplas threads acessam um contador protegido com um mutex.
10. Crie um programa que simule um sistema de fila de tarefas usando threads e um canal de mensagens.
11. Crie um programa que inicie três threads e faça cada uma imprimir uma mensagem de identificação.
12. Implemente uma função assíncrona que simule um cálculo com atraso.
13. Desenvolva um sistema de contagem que usa `join` para sincronizar threads.
14. Escreva um programa que utilize um canal para somar os números em uma lista em paralelo.
15. Implemente um sistema que cria threads para calcular o fatorial de um número.
16. Crie uma aplicação que usa múltiplas threads para verificar a primalidade de números.
17. Desenvolva um programa que cria várias threads e faz cada uma calcular o quadrado de um número.
18. Escreva um programa que usa um mutex para impedir o acesso simultâneo a uma variável global.
19. Implemente uma função que utilize threads para baixar dados simulados de uma API fictícia.
20. Crie um sistema onde cada thread calcula a potência de dois para um número específico.

---

## 🕸️ 7. **Rust Avançado e Manipulação de Memória**

1. Crie um macro que aceite uma lista de strings e imprima cada uma em uma nova linha.
2. Escreva uma função que aloque um valor em `Box` e exiba o valor armazenado.
3. Implemente uma estrutura de dados para um nó de árvore binária usando `Box`.
4. Desenvolva uma função que use `Rc` para compartilhar um valor entre múltiplas referências.
5. Crie um programa que utilize `Arc` para compartilhar dados entre threads de forma segura.
6. Escreva um sistema de armazenamento de dados usando um `HashMap` para armazenar informações.
7. Implemente um programa que crie uma árvore de diretórios usando `BTreeMap`.
8. Desenvolva um sistema de contagem onde múltiplas threads incrementam um valor usando `Arc<Mutex<i32>>`.
9. Crie um macro que aceite um vetor de números e calcule a média.
10. Construa um programa que use ponteiros `unsafe` para acessar diretamente a memória de uma variável.
11. Crie um macro que gere uma lista de números consecutivos.
12. Implemente uma função que use `unsafe` para manipular um ponteiro diretamente.
13. Desenvolva uma estrutura de dados para uma lista encadeada usando `Box`.
14. Crie uma função que use `Rc` para compartilhar uma `String` entre múltiplas variáveis.
15. Implemente um sistema que gerencie um cache em memória usando `HashMap`.
16. Crie um programa que utilize um `BTreeMap` para classificar dados de maneira ordenada.
17. Escreva uma função que utilize `Box` para armazenar uma estrutura e a retorne.
18. Desenvolva um sistema de histórico de transações com `Rc` para salvar dados de forma compartilhada.
19. Crie uma macro que aceite uma lista de números e retorne a soma dos valores.
20. Construa um programa que utilize `Arc` para permitir o compartilhamento de dados entre várias threads.


