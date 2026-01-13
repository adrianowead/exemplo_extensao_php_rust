<?php

echo "TESTE: EXTENSÃO RUST (E-COMMERCE BENCHMARK)\n";
echo str_repeat("─", 60) . "\n";

if (!function_exists('rust_benchmark')) {
    echo "ERRO: Função rust_benchmark() não encontrada!\n";
    echo "    Certifique-se de que a extensão foi compilada com 'cargo build --release'\n";
    echo "    e o arquivo .so/.dll está habilitado no seu php.ini.\n\n";
    exit(1);
}

$iterations = 1000000000.0; 

echo "Processando " . number_format($iterations, 0, ',', '.') . " pedidos em Rust... ";

$start = microtime(true);
$rustResults = rust_benchmark($iterations);
$phpTotalTime = microtime(true) - $start;

echo "✓ Concluído!\n\n";

echo "Resultados da Extensão Nativa:\n";
echo str_repeat("-", 30) . "\n";
echo "  - Tempo de execução (Kernel): " . number_format($rustResults['execution_time'], 4, ',', '.') . "s\n";
echo "  - Overhead total PHP:        " . number_format($phpTotalTime, 4, ',', '.') . "s\n";
echo "  - Pedidos por segundo:       " . number_format($rustResults['ops_per_sec'], 0, ',', '.') . "\n";

echo "\nTotais Financeiros Processados:\n";
echo "  - Valor Produtos:  R$ " . number_format($rustResults['total_valor'], 2, ',', '.') . "\n";
echo "  - Impostos:        R$ " . number_format($rustResults['total_impostos'], 2, ',', '.') . "\n";
echo "  - Frete:           R$ " . number_format($rustResults['total_frete'], 2, ',', '.') . "\n";
echo "  - Descontos:       R$ " . number_format($rustResults['total_desconto'], 2, ',', '.') . "\n";
echo "  - TOTAL FINAL:     R$ " . number_format($rustResults['total_final'], 2, ',', '.') . "\n";

echo str_repeat("─", 60) . "\n";

$phpBaseline = 208.31;
$ganho = $phpBaseline / $rustResults['execution_time'];

echo "VEREDITO FINAL:\n";
echo "O Rust foi " . number_format($ganho, 1, ',', '.') . "x mais rápido que o PHP puro.\n";

/*
TESTE: EXTENSÃO RUST (E-COMMERCE BENCHMARK)
────────────────────────────────────────────────────────────
Processando 1.000.000.000 pedidos em Rust... ✓ Concluído!
Resultados da Extensão Nativa:
------------------------------
- Tempo de execução (Kernel): 3,4023s
- Overhead total PHP: 3,4023s
- Pedidos por segundo: 293.919.632
Totais Financeiros Processados:
- Valor Produtos: R$ 252.499.706.518,10
- Impostos: R$ 69.816.168.851,88
- Frete: R$ 152.846.727.856,57
- Descontos: R$ 6.564.992.713,12
- TOTAL FINAL: R$ 468.597.611.478,61
────────────────────────────────────────────────────────────
VEREDITO FINAL:
O Rust foi 61,2x mais rápido que o PHP puro.
*/
