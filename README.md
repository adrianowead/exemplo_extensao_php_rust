# Extensões de Alto Desempenho em PHP

Este repositório faz parte do material de apoio do livro **"Extensões PHP de Alto Desempenho - Rust + PHP: Performance Nativa Sem Complexidade de C/C++"**, ISBN (Livro Digital): **978-65-01-89387-7**.

O principal objetivo do livro é popularizar o desenvolvimento de extensões para PHP usando Rust e demonstrar o real ganho de desempenho, comparando diretamente:

- PHP Puro
- PHP + Rust (single thread)
- PHP + Rust (multi thread)

Este repositório fornece um exemplo totalmente funcional dos códigos comentados no livro, permitindo que o desenvolvedor execute os testes de forma prática e evite erros de digitação ao replicar os exemplos.

## Demonstração de Automação

Embora o desenvolvimento de extensões exija uma configuração de ambiente rigorosa, é possível automatizar todo o processo de build (Windows e Linux) utilizando Docker.

Confira no vídeo abaixo a automação em funcionamento, gerando binários para múltiplas versões do PHP em segundos:

👉 **[Assista ao vídeo de demonstração no YouTube](https://youtu.be/nNU7lcM5-To)**

## Próximos Passos: Pack de Automação

Em breve, será disponibilizado para venda o **Pack de Automação Profissional**, contendo todos os scripts de infraestrutura, Dockerfile e receitas de build demonstrados no vídeo.

Este pacote foi desenhado para quem deseja pular a etapa burocrática de configuração de compiladores e linkers (especialmente no Windows) e focar exclusivamente na lógica de negócio em Rust.

# Conhecimento

O livro não é essencial para compreender este repositório; entretanto, a leitura é altamente recomendada para se familiarizar com os conceitos de baixo nível, a Zend Engine e os motivos de segurança de memória que tornam o Rust a escolha ideal para estender o PHP. Especialmente se você é um programador PHP e ainda não tem familiaridade com o ecossistema Rust.