use ext_php_rs::{
    prelude::*,
    exception::PhpException,
};
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;

// ============================================================================
// INTERFACE: InterfacePersistivel
// Define o contrato para classes que podem ser persistidas
// ============================================================================

/// Interface que define métodos para persistência de dados
/// Equivalente PHP:
/// ```php
/// namespace Wead;
/// interface InterfacePersistivel {
///     public function carregar(): array;
///     public function salvar(array $dados): bool;
/// }
/// ```
#[php_class]
#[php(name = "Wead\\InterfacePersistivel")]
pub struct InterfacePersistivel;

// ============================================================================
// CLASSE ABSTRATA: EntidadeBase
// Fornece funcionalidades comuns para todas as entidades
// ============================================================================

/// Classe abstrata base para entidades do sistema
/// Fornece ID e métodos utilitários comuns
#[php_class]
#[php(name = "Wead\\EntidadeBase")]
#[derive(Debug, Clone)]
pub struct EntidadeBase {
    /// ID único da entidade
    pub id: Option<i64>,
}

#[php_impl]
impl EntidadeBase {
    /// Construtor padrão da entidade base
    pub fn __construct() -> Self {
        Self { id: None }
    }

    /// Obtém o ID da entidade
    /// @return int|null
    pub fn obter_id(&self) -> Option<i64> {
        self.id
    }

    /// Define o ID da entidade
    /// @param int $id
    pub fn definir_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    /// Verifica se a entidade está persistida (tem ID)
    /// @return bool
    pub fn esta_persistida(&self) -> bool {
        self.id.is_some()
    }
}

// ============================================================================
// CLASSE: Pessoa
// Representa uma pessoa no sistema com herança de EntidadeBase
// ============================================================================

/// Classe Pessoa que herda de EntidadeBase
/// Representa uma pessoa com nome, email e telefone
#[php_class]
#[php(name = "Wead\\Pessoa")]
#[derive(Debug, Clone, Default)]
pub struct Pessoa {
    /// ID da pessoa (herdado conceitualmente)
    pub id: Option<i64>,

    /// Nome completo da pessoa
    pub nome: String,

    /// Email da pessoa
    pub email: String,

    /// Telefone da pessoa
    pub telefone: String,
}

#[php_impl]
impl Pessoa {
    /// Construtor da classe Pessoa
    /// @param string $nome Nome completo
    /// @param string $email Endereço de email
    /// @param string $telefone Número de telefone
    pub fn __construct(nome: String, email: String, telefone: String) -> Self {
        Self {
            id: None,
            nome,
            email,
            telefone,
        }
    }

    /// Obtém o nome da pessoa
    /// @return string
    pub fn obter_nome(&self) -> String {
        self.nome.clone()
    }

    /// Define o nome da pessoa
    /// @param string $nome
    pub fn definir_nome(&mut self, nome: String) {
        self.nome = nome;
    }

    /// Obtém o email da pessoa
    /// @return string
    pub fn obter_email(&self) -> String {
        self.email.clone()
    }

    /// Define o email da pessoa
    /// @param string $email
    /// @throws Exception Se o email for inválido
    pub fn definir_email(&mut self, email: String) -> PhpResult {
        if !email.contains('@') {
            return Err(PhpException::default("Email inválido: deve conter @".into()));
        }
        self.email = email;
        Ok(())
    }

    /// Obtém o telefone da pessoa
    /// @return string
    pub fn obter_telefone(&self) -> String {
        self.telefone.clone()
    }

    /// Define o telefone da pessoa
    /// @param string $telefone
    pub fn definir_telefone(&mut self, telefone: String) {
        self.telefone = telefone;
    }

    /// Obtém o ID da pessoa
    /// @return int|null
    pub fn obter_id(&self) -> Option<i64> {
        self.id
    }

    /// Define o ID da pessoa
    /// @param int $id
    pub fn definir_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    /// Converte a pessoa para array associativo
    /// @return array
    pub fn para_array(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("id".to_string(), self.id.unwrap_or(0).to_string());
        map.insert("nome".to_string(), self.nome.clone());
        map.insert("email".to_string(), self.email.clone());
        map.insert("telefone".to_string(), self.telefone.clone());
        map
    }

    /// Valida os dados da pessoa
    /// @return bool
    /// @throws Exception Se algum campo estiver inválido
    pub fn validar(&self) -> PhpResult<bool> {
        if self.nome.trim().is_empty() {
            return Err(PhpException::default("Nome não pode ser vazio".into()));
        }
        if self.email.trim().is_empty() || !self.email.contains('@') {
            return Err(PhpException::default("Email inválido".into()));
        }
        if self.telefone.trim().is_empty() {
            return Err(PhpException::default("Telefone não pode ser vazio".into()));
        }
        Ok(true)
    }

    /// Representação em string da pessoa
    /// @return string
    pub fn __to_string(&self) -> String {
        format!(
            "Pessoa[id={}, nome={}, email={}, telefone={}]",
            self.id.map_or("null".to_string(), |id| id.to_string()),
            self.nome,
            self.email,
            self.telefone
        )
    }
}

// ============================================================================
// CLASSE: Storage
// Gerencia persistência de dados em arquivo CSV
// Implementa o padrão Repository
// ============================================================================

/// Classe Storage para operações CRUD em arquivo CSV
/// Gerencia o armazenamento e recuperação de pessoas
#[php_class]
#[php(name = "Wead\\Storage")]
#[derive(Debug, Clone)]
pub struct Storage {
    /// Caminho do arquivo CSV
    caminho_arquivo: String,

    /// Último ID gerado (para auto-incremento)
    ultimo_id: i64,
}

#[php_impl]
impl Storage {
    /// Construtor do Storage
    /// @param string $caminho_arquivo Caminho completo do arquivo CSV
    /// @throws Exception Se o caminho for vazio
    pub fn __construct(caminho_arquivo: String) -> PhpResult<Self> {
        if caminho_arquivo.trim().is_empty() {
            return Err(PhpException::default(
                "Caminho do arquivo não pode ser vazio".into()
            ));
        }

        let mut storage = Self {
            caminho_arquivo,
            ultimo_id: 0,
        };

        // Inicializa o arquivo se não existir
        storage.inicializar_arquivo()?;
        // Carrega o último ID
        storage.carregar_ultimo_id()?;

        Ok(storage)
    }



    /// Cria (insere) uma nova pessoa no CSV
    /// @param Pessoa $pessoa Pessoa a ser inserida
    /// @return int ID da pessoa criada
    /// @throws Exception Se houver erro na validação ou escrita
    pub fn criar(&mut self, pessoa: &mut Pessoa) -> PhpResult<i64> {
        // Valida antes de inserir
        pessoa.validar()?;

        // Atribui novo ID
        let novo_id = self.proximo_id();
        pessoa.definir_id(novo_id);

        // Abre arquivo em modo append
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.caminho_arquivo)
            .map_err(|e| PhpException::default(
                format!("Erro ao abrir arquivo: {}", e)
            ))?;

        // Escreve linha CSV
        let linha = format!(
            "{},{},{},{}\n",
            pessoa.id.unwrap(),
            Self::escapar_csv(&pessoa.nome),
            Self::escapar_csv(&pessoa.email),
            Self::escapar_csv(&pessoa.telefone)
        );

        file.write_all(linha.as_bytes())
            .map_err(|e| PhpException::default(
                format!("Erro ao escrever no arquivo: {}", e)
            ))?;

        Ok(novo_id)
    }

    /// Busca uma pessoa por ID
    /// @param int $id ID da pessoa
    /// @return Pessoa|null Pessoa encontrada ou null
    pub fn buscar_por_id(&self, id: i64) -> Option<Pessoa> {
        match self.listar_todas() {
            Ok(pessoas) => {
                pessoas.into_iter().find(|p| p.id == Some(id))
            }
            Err(_) => None
        }
    }

    /// Lista todas as pessoas cadastradas
    /// @return array Array de Pessoa
    /// @throws Exception Se houver erro na leitura
    pub fn listar_todas(&self) -> PhpResult<Vec<Pessoa>> {
        let file = File::open(&self.caminho_arquivo)
            .map_err(|e| PhpException::default(
                format!("Erro ao abrir arquivo: {}", e)
            ))?;

        let reader = BufReader::new(file);
        let mut pessoas = Vec::new();

        for (i, line) in reader.lines().enumerate() {
            if i == 0 { continue; } // Pula cabeçalho

            let line = line.map_err(|e| PhpException::default(
                format!("Erro ao ler linha: {}", e)
            ))?;

            if line.trim().is_empty() {
                continue;
            }

            // Parse da linha CSV
            let campos: Vec<&str> = line.split(',').collect();
            if campos.len() >= 4 {
                let id = campos[0].parse::<i64>().ok();
                let nome = Self::desescapar_csv(campos[1]);
                let email = Self::desescapar_csv(campos[2]);
                let telefone = Self::desescapar_csv(campos[3]);

                let pessoa = Pessoa {
                    id,
                    nome,
                    email,
                    telefone,
                };

                pessoas.push(pessoa);
            }
        }

        Ok(pessoas)
    }

    /// Atualiza uma pessoa existente
    /// @param Pessoa $pessoa Pessoa com dados atualizados (deve ter ID)
    /// @return bool true se atualizado com sucesso
    /// @throws Exception Se pessoa não tiver ID ou houver erro
    pub fn atualizar(&self, pessoa: &Pessoa) -> PhpResult<bool> {
        if pessoa.id.is_none() {
            return Err(PhpException::default(
                "Pessoa deve ter ID para ser atualizada".into()
            ));
        }

        pessoa.validar()?;

        let mut pessoas = self.listar_todas()?;
        let id_busca = pessoa.id.unwrap();
        
        // Busca e atualiza a pessoa
        let mut encontrada = false;
        for p in pessoas.iter_mut() {
            if p.id == Some(id_busca) {
                p.nome = pessoa.nome.clone();
                p.email = pessoa.email.clone();
                p.telefone = pessoa.telefone.clone();
                encontrada = true;
                break;
            }
        }

        if !encontrada {
            return Err(PhpException::default(
                format!("Pessoa com ID {} não encontrada", id_busca)
            ));
        }

        // Reescreve o arquivo completo
        self.reescrever_arquivo(&pessoas)?;
        Ok(true)
    }

    /// Deleta uma pessoa por ID
    /// @param int $id ID da pessoa a ser deletada
    /// @return bool true se deletado com sucesso
    /// @throws Exception Se pessoa não for encontrada
    pub fn deletar(&self, id: i64) -> PhpResult<bool> {
        let mut pessoas = self.listar_todas()?;
        let tamanho_original = pessoas.len();

        // Remove a pessoa da lista
        pessoas.retain(|p| p.id != Some(id));

        if pessoas.len() == tamanho_original {
            return Err(PhpException::default(
                format!("Pessoa com ID {} não encontrada", id)
            ));
        }

        // Reescreve o arquivo sem a pessoa deletada
        self.reescrever_arquivo(&pessoas)?;
        Ok(true)
    }

    /// Busca pessoas por nome (busca parcial, case-insensitive)
    /// @param string $nome Nome ou parte do nome
    /// @return array Array de Pessoa
    pub fn buscar_por_nome(&self, nome: String) -> PhpResult<Vec<Pessoa>> {
        let todas = self.listar_todas()?;
        let nome_lower = nome.to_lowercase();
        
        let resultado: Vec<Pessoa> = todas
            .into_iter()
            .filter(|p| p.nome.to_lowercase().contains(&nome_lower))
            .collect();

        Ok(resultado)
    }

    /// Conta o total de pessoas cadastradas
    /// @return int Total de pessoas
    pub fn contar(&self) -> PhpResult<i64> {
        let pessoas = self.listar_todas()?;
        Ok(pessoas.len() as i64)
    }


    /// Limpa todos os registros do arquivo (mantém cabeçalho)
    /// @return bool true se sucesso
    pub fn limpar_todos(&self) -> PhpResult<bool> {
        self.reescrever_arquivo(&[])?;
        Ok(true)
    }

    /// Obtém o caminho do arquivo
    /// @return string
    pub fn obter_caminho(&self) -> String {
        self.caminho_arquivo.clone()
    }
}

impl Storage {
    /// Inicializa o arquivo CSV com cabeçalhos se não existir
    fn inicializar_arquivo(&self) -> PhpResult {
        let path = Path::new(&self.caminho_arquivo);
        
        if !path.exists() {
            // Cria diretório se não existir
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| PhpException::default(
                            format!("Erro ao criar diretório: {}", e)
                        ))?;
                }
            }

            // Cria arquivo com cabeçalho
            let mut file = File::create(path)
                .map_err(|e| PhpException::default(
                    format!("Erro ao criar arquivo: {}", e)
                ))?;

            writeln!(file, "id,nome,email,telefone")
                .map_err(|e| PhpException::default(
                    format!("Erro ao escrever cabeçalho: {}", e)
                ))?;
        }
        Ok(())
    }

    /// Carrega o último ID do arquivo para gerar próximo ID
    fn carregar_ultimo_id(&mut self) -> PhpResult {
        let file = File::open(&self.caminho_arquivo)
            .map_err(|e| PhpException::default(
                format!("Erro ao abrir arquivo: {}", e)
            ))?;

        let reader = BufReader::new(file);
        let mut max_id = 0i64;

        // Pula a primeira linha (cabeçalho) e busca o maior ID
        for (i, line) in reader.lines().enumerate() {
            if i == 0 { continue; } // Pula cabeçalho

            let line = line.map_err(|e| PhpException::default(
                format!("Erro ao ler linha: {}", e)
            ))?;

            if let Some(id_str) = line.split(',').next() {
                if let Ok(id) = id_str.parse::<i64>() {
                    if id > max_id {
                        max_id = id;
                    }
                }
            }
        }

        self.ultimo_id = max_id;
        Ok(())
    }

    /// Gera o próximo ID disponível
    fn proximo_id(&mut self) -> i64 {
        self.ultimo_id += 1;
        self.ultimo_id
    }

    /// Reescreve o arquivo CSV completamente com nova lista
    fn reescrever_arquivo(&self, pessoas: &[Pessoa]) -> PhpResult {
        let mut file = File::create(&self.caminho_arquivo)
            .map_err(|e| PhpException::default(
                format!("Erro ao criar arquivo: {}", e)
            ))?;

        // Escreve cabeçalho
        writeln!(file, "id,nome,email,telefone")
            .map_err(|e| PhpException::default(
                format!("Erro ao escrever cabeçalho: {}", e)
            ))?;

        // Escreve cada pessoa
        for pessoa in pessoas {
            let linha = format!(
                "{},{},{},{}\n",
                pessoa.id.unwrap_or(0),
                Self::escapar_csv(&pessoa.nome),
                Self::escapar_csv(&pessoa.email),
                Self::escapar_csv(&pessoa.telefone)
            );

        file.write_all(linha.as_bytes())
                .map_err(|e| PhpException::default(
                    format!("Erro ao escrever no arquivo: {}", e)
                ))?;
        }

        Ok(())
    }

    /// Escapa string para formato CSV (adiciona aspas se necessário)
    fn escapar_csv(s: &str) -> String {
        if s.contains(',') || s.contains('"') || s.contains('\n') {
            format!("\"{}\"", s.replace('"', "\"\""))
        } else {
            s.to_string()
        }
    }

    /// Remove escape de string CSV
    fn desescapar_csv(s: &str) -> String {
        let trimmed = s.trim();
        if trimmed.starts_with('"') && trimmed.ends_with('"') {
            trimmed[1..trimmed.len()-1].replace("\"\"", "\"")
        } else {
            trimmed.to_string()
        }
    }

}

// ============================================================================
// FUNÇÕES AUXILIARES DO NAMESPACE
// Funções utilitárias disponíveis globalmente no namespace Wead
// ============================================================================

/// Formata um telefone brasileiro (remove caracteres especiais)
/// @param string $telefone Telefone com ou sem formatação
/// @return string Telefone apenas com números
#[php_function]
#[php(name = "Wead\\formatar_telefone")]
pub fn formatar_telefone(telefone: String) -> String {
    telefone.chars()
        .filter(|c| c.is_ascii_digit())
        .collect()
    }

/// Valida formato de email básico
/// @param string $email Email a validar
/// @return bool true se válido
#[php_function]
#[php(name = "Wead\\validar_email")]
pub fn validar_email(email: String) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5
}

/// Gera um resumo de uma pessoa em formato legível
/// @param Pessoa $pessoa
/// @return string
#[php_function]
#[php(name = "Wead\\resumo_pessoa")]
pub fn resumo_pessoa(pessoa: &Pessoa) -> String {
    format!(
        "ID: {} | Nome: {} | Email: {} | Tel: {}",
        pessoa.id.map_or("N/A".to_string(), |id| id.to_string()),
        pessoa.nome,
        pessoa.email,
        pessoa.telefone
    )
}

pub fn register(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<InterfacePersistivel>()
        .class::<EntidadeBase>()
        .class::<Pessoa>()
        .class::<Storage>()
        .function(wrap_function!(formatar_telefone))
        .function(wrap_function!(validar_email))
        .function(wrap_function!(resumo_pessoa))
}
