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
$rustResults = rust_benchmark_parallel($iterations);
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
- Tempo de execução (Kernel): 0,2519s
- Overhead total PHP: 0,2519s
- Pedidos por segundo: 3.969.974.290
Totais Financeiros Processados:
- Valor Produtos: R$ 252.499.706.518,10
- Impostos: R$ 69.816.168.852,06
- Frete: R$ 152.846.728.820,67
- Descontos: R$ 6.564.992.713,08
- TOTAL FINAL: R$ 468.597.611.477,84
────────────────────────────────────────────────────────────
VEREDITO FINAL:
O Rust foi 827,0x mais rápido que o PHP puro.

*/