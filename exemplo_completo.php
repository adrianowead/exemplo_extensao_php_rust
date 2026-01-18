<?php
// Exemplo Completo da Extensão Rust para PHP
// Autor: Adriano Maciel <wead.tech>

// Verifica se a extensão está carregada
if (!extension_loaded('hello_php')) {
    echo "ERRO: A extensão 'hello_php' não está carregada.\n";
    echo "Verifique seu php.ini ou compile a extensão com 'cargo build --release'.\n";
    exit(1);
}

echo "\n================================================================\n";
echo "       DEMONSTRAÇÃO COMPLETA: EXTENSÃO RUST EM PHP              \n";
echo "================================================================\n\n";

// ---------------------------------------------------------
// 1. Funções Básicas
// ---------------------------------------------------------
echo "[1] Testando Função Básica (meu_ola)...\n";
echo "    Retorno: " . meu_ola('Adriano') . "\n\n";

// ---------------------------------------------------------
// 2. Funções Auxiliares (Rust puro)
// ---------------------------------------------------------
echo "[2] Testando Funções Auxiliares...\n";

$email = "teste@exemplo.com";
$emailInvalido = "teste.com";
echo "    - Validar Email '$email': " . (Wead\validar_email($email) ? "OK" : "Inválido") . "\n";
echo "    - Validar Email '$emailInvalido': " . (Wead\validar_email($emailInvalido) ? "OK" : "Inválido") . "\n";

$telefone = "(11) 9.8765-4321";
echo "    - Formatar Telefone '$telefone': " . Wead\formatar_telefone($telefone) . "\n\n";

// ---------------------------------------------------------
// 3. Sistema CRUD de Pessoas (Pessoa + Storage)
// ---------------------------------------------------------
echo "[3] Testando Sistema CRUD (Persistência em CSV)...\n";

try {
    // Definir arquivo temporário para teste
    $arquivoCsv = sys_get_temp_dir() . DIRECTORY_SEPARATOR . 'pessoas_rust.csv';
    echo "    - Arquivo de armazenamento: $arquivoCsv\n";
    
    // Instanciar Storage
    $storage = new Wead\Storage($arquivoCsv);
    
    // Limpar estado anterior para o teste
    echo "    - Limpando registros anteriores... ";
    $storage->limparTodos();
    echo "OK\n";
    
    // Criar nova pessoa
    echo "    - Criando nova pessoa... ";
    $pessoa1 = new Wead\Pessoa(
        'Maria Silva',
        'maria@email.com',
        '11999998888'
    );
    
    // Validar antes de salvar
    if ($pessoa1->validar()) {
        $id1 = $storage->criar($pessoa1);
        echo "OK (ID: $id1)\n";
    }
    
    // Criar segunda pessoa
    echo "    - Criando segunda pessoa... ";
    $pessoa2 = new Wead\Pessoa(
        'João Santos',
        'joao@email.com',
        '21988887777'
    );
    $id2 = $storage->criar($pessoa2);
    echo "OK (ID: $id2)\n";
    
    echo "    - Total de pessoas cadastradas: " . $storage->contar() . "\n";
    
    // Listar todas
    echo "    - Listando todas as pessoas:\n";
    $todas = $storage->listarTodas();
    foreach ($todas as $p) {
        echo "      > " . Wead\resumo_pessoa($p) . " (Objeto)\n";
        // Demonstrar conversão para array
        $arr = $p->paraArray();
        echo "        Array: " . json_encode($arr) . "\n";
    }
    
    // Buscar por ID
    echo "    - Buscando ID $id1... ";
    $encontrada = $storage->buscarPorId($id1);
    if ($encontrada) {
        echo "Encontrada: " . $encontrada->obterNome() . "\n";
        
        // Atualizar
        echo "    - Atualizando email... ";
        $encontrada->definirEmail('maria.novo@email.com');
        $storage->atualizar($encontrada);
        echo "OK\n";
    } else {
        echo "Não encontrada!\n";
    }
    
    // Buscar por Nome
    echo "    - Buscando por nome 'João'... ";
    $resultados = $storage->buscarPorNome('João');
    echo count($resultados) . " encontrados.\n";
    
    // Deletar
    echo "    - Deletando ID $id2... ";
    $storage->deletar($id2);
    echo "OK\n";
    
    echo "    - Total final: " . $storage->contar() . "\n\n";
    
    // Mostrar conteúdo final do arquivo
    echo "    - Conteúdo do arquivo CSV:\n";
    echo "      " . str_replace("\n", "\n      ", trim(file_get_contents($arquivoCsv))) . "\n\n";
    
} catch (Exception $e) {
    echo "ERRO CRUD: " . $e->getMessage() . "\n";
}

// ---------------------------------------------------------
// 4. Benchmarks (Serial vs Parallel)
// ---------------------------------------------------------
echo "[4] Executando Benchmarks de Performance...\n";

$iterations = 10000000;
echo "    Iterações: " . number_format($iterations, 0, ',', '.') . "\n";

// Serial
echo "    > Executando Rust Serial... ";
$start = microtime(true);
$serialResult = rust_benchmark((float)$iterations);
$timeSerial = microtime(true) - $start;
echo number_format($serialResult['execution_time'], 4) . "s (Kernel) / " . number_format($timeSerial, 4) . "s (Total)\n";

// Parallel
echo "    > Executando Rust Parallel... ";
$start = microtime(true);
$parallelResult = rust_benchmark_parallel((float)$iterations);
$timeParallel = microtime(true) - $start;
echo number_format($parallelResult['execution_time'], 4) . "s (Kernel) / " . number_format($timeParallel, 4) . "s (Total)\n";

// Comparativo
if (isset($parallelResult['execution_time']) && isset($serialResult['execution_time'])) {
    $speedup = $serialResult['execution_time'] / $parallelResult['execution_time'];
    echo "    > Speedup (Paralelo vs Serial): " . number_format($speedup, 2) . "x\n";
    echo "    > Cores Utilizados: " . $parallelResult['cores_used'] . "\n";
}

echo "\n================================================================\n";
echo "                   FIM DA DEMONSTRAÇÃO                          \n";
echo "================================================================\n";
