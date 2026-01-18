# Extensões de Alto Desempenho em PHP

Este repositório faz parte do material de apoio do livro **"Extensões PHP de Alto Desempenho - Rust + PHP: Performance Nativa Sem Complexidade de C/C++"**, ISBN (Livro Digital): **978-65-01-89387-7**.

O principal objetivo do livro é popularizar o desenvolvimento de extensões para PHP usando Rust e demonstrar o real ganho de desempenho, comparando diretamente:

- PHP Puro
- PHP + Rust (single thread)
- PHP + Rust (multi thread)

Este repositório fornece um exemplo totalmente funcional dos códigos comentados no livro, permitindo que o desenvolvedor execute os testes de forma prática e evite erros de digitação ao replicar os exemplos.

## Download dos Binários (Releases)

Para facilitar o teste imediato, todos os binários desta extensão de demonstração já estão compilados e disponíveis na **[aba de Releases deste repositório](https://github.com/adrianowead/exemplo_extensao_php_rust/releases)**.

Os arquivos estão organizados da seguinte forma:

* **Linux (x64):** Arquivos comprimidos em `.tgz`.
* **Windows (x64):** Arquivos comprimidos em `.zip`.

As compilações são compatíveis com as versões **PHP 8.1, 8.2, 8.3, 8.4 e 8.5**, abrangendo tanto as variantes **NTS** (Non-Thread Safe) quanto **TS** (Thread Safe). Basta baixar a versão correspondente ao seu ambiente e configurá-la no seu `php.ini`.

## Estrutura de Testes no Repositório

Após instalar a extensão, você pode utilizar os scripts disponíveis neste repositório para validar o funcionamento e a performance:

```text
.
├── src/                        # Código-fonte Rust (Exemplos do livro/repositório)
├── benchmark_*.php             # Scripts de teste de performance (Puro vs Rust)
└── exemplo_completo.php        # Script de validação da extensão

```

## Demonstração de Automação

Embora o desenvolvimento de extensões exija uma configuração de ambiente rigorosa, é possível automatizar todo o processo de build (Windows e Linux) utilizando Docker.

Confira no vídeo abaixo a automação em funcionamento, gerando binários para múltiplas versões do PHP em segundos:

**[![Assista ao vídeo de demonstração no YouTube](https://img.youtube.com/vi/nNU7lcM5-To/0.jpg)](https://youtu.be/nNU7lcM5-To)**

## Próximos Passos: Pack de Automação

Em breve, será disponibilizado para venda o **Pack de Automação Profissional**, contendo todos os scripts de infraestrutura, Dockerfile e receitas de build demonstrados no vídeo.

Este pacote foi desenhado para quem deseja pular a etapa burocrática de configuração de compiladores e linkers (especialmente no Windows) e focar exclusivamente na lógica de negócio em Rust.

Não há nenhuma obrigação de adquirir o pack, ele será apenas uma facilidade adicional para quem não quer, ou não pode, ter o trabalho manual de automatizar todo o processo.

# Conhecimento

O livro não é essencial para compreender este repositório; entretanto, a leitura é altamente recomendada para se familiarizar com os conceitos de baixo nível, a Zend Engine e os motivos de segurança de memória que tornam o Rust a escolha ideal para estender o PHP. Especialmente se você é um programador PHP e ainda não tem familiaridade com o ecossistema Rust.
