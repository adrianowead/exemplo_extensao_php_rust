# Extensões de Alto Desempenho em PHP

Este repositório faz parte do material de apoio do livro **"Extensões PHP de Alto Desempenho - Rust + PHP: Performance Nativa Sem Complexidade de C/C++"**, ISBN (Livro Digital): **978-65-01-89387-7**.

O principal objetivo do livro é popularizar o desenvolvimento de extensões para PHP usando Rust e demonstrar o real ganho de desempenho, comparando diretamente:

- PHP Puro
- PHP + Rust (single thread)
- PHP + Rust (multi thread)

Este repositório fornece um exemplo totalmente funcional dos códigos comentados no livro, permitindo que o desenvolvedor execute os testes de forma prática e evite erros de digitação ao replicar os exemplos.

## Adquira o Livro

O livro completo, contendo a fundamentação teórica e a integração de baixo nível entre PHP e Rust, **está atualmente disponível para aquisição na Hotmart**. Em breve, a obra também estará disponível em outras plataformas e livrarias.

**[Comprar Livro na Hotmart: Extensões PHP de Alto Desempenho](https://go.hotmart.com/N104023457L)**

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

## Pack de Automação Profissional (Disponível)

Já está disponível para venda o **Pack de Automação Profissional**, contendo toda a infraestrutura de scripts e receitas de build demonstradas no vídeo acima. Este pacote resolve a maior barreira no desenvolvimento de extensões: a configuração complexa de compiladores, linkers e headers, especialmente em ambientes Windows.

👉 **[Adquirir Pack de Automação na Hotmart](https://pay.hotmart.com/D104027815O)**

### O que você precisa saber antes de adquirir:

**1. Versões e Variantes Suportadas (x64):**
- **PHP:** Automação pronta para as versões 8.1, 8.2, 8.3, 8.4 e 8.5.
- **Variantes:** Suporte completo para NTS (*Non-Thread Safe*) e TS (*Thread Safe*).
- **Sistemas Alvo:** Geração de binários para Linux (`.so`) e Windows (`.dll`).


**2. Requisitos de Hardware e Software:**
- **Sistema Operacional:** Requer Windows 10 ou 11 (exigência do Docker para compilação de containers Windows).
- **Ferramentas:** Docker Desktop e um terminal Bash (como o do Git for Windows).
- **Memória:** Mínimo de 16GB de RAM.
- **Espaço em Disco:** Mínimo de 200GB (necessário para o cache das imagens e camadas do Windows SDK).


**3. Estrutura do Pacote:**

Ao adquirir, você terá acesso a uma estrutura organizada para desenvolvimento profissional:

- **`/docker`:** Contém os Dockerfiles e a engine de automação interna.
- **`/src`:** Pasta para o seu código-fonte em Rust.
- **`/release`:** Local onde os binários compilados são organizados automaticamente por versão.
- **Scripts de Entrada:** `./run_all_linux.sh` e `./run_all_windows.sh` para disparar os builds globais.


**4. Nota sobre a Primeira Execução:**

A primeira compilação para Windows é naturalmente demorada devido ao download das dependências oficiais da Microsoft. Uma vez que as imagens estão em cache no seu Docker, as compilações subsequentes tornam-se extremamente rápidas.


Não há nenhuma obrigação de adquirir o pack, ele será apenas uma facilidade adicional para quem não quer, ou não pode, ter o trabalho manual de automatizar todo o processo.

# Conhecimento

O livro não é essencial para compreender este repositório; entretanto, a leitura é altamente recomendada para se familiarizar com os conceitos de baixo nível, a Zend Engine e os motivos de segurança de memória que tornam o Rust a escolha ideal para estender o PHP. Especialmente se você é um programador PHP e ainda não tem familiaridade com o ecossistema Rust.
