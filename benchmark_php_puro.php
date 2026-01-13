<?php

$n = 1000000000;
$start = microtime(true);

$totais = ['valor' => 0, 'impostos' => 0, 'frete' => 0, 'desconto' => 0, 'final' => 0];

for ($i = 1; $i <= $n; $i++) {
    $valor  = (($i % 4951) + 50) / 10;
    $peso   = (($i % 500) + 1) / 10;
    $idx    = $i % 5;
    $prime  = ($i % 100) < 20;

    $estados = ['SP','RJ','MG','RS','BA'];
    $estado = $estados[$idx];
    
    $icms = match($estado) {
        'SP' => 0.18, 'RJ' => 0.20, 'MG' => 0.18,
        'RS' => 0.17, default => 0.19
    };
    
    $impostos = $valor * ($icms + 0.0925);
    
    $dist = match($estado) {
        'SP' => 50, 'RJ' => 430, 'MG' => 586,
        'RS' => 1130, 'BA' => 1446
    };

    $frete = ($peso * 2.5) + ($dist * 0.15) + 8.5;
    if ($prime && $valor >= 100) $frete = 0;
    
    $desc = $valor * ($prime ? 0.05 : 0.02);
    $total = $valor + $impostos + $frete - $desc;
    
    $totais['valor']    += $valor;
    $totais['impostos'] += $impostos;
    $totais['frete']    += $frete;
    $totais['desconto'] += $desc;
    $totais['final']    += $total;
    
    if ($i % 100000000 == 0) {
        $decorrido = microtime(true) - $start;
        printf("%s processados | %.02fs\n", number_format($i, 0, ',', '.'), $decorrido);
    }
}

$tempo = microtime(true) - $start;

printf(
    "Processados: %s pedidos em %.02fs (%.0f/s)\n\n", 
    number_format($n, 0, ',', '.'), $tempo, $n/$tempo
);

printf("Valor produtos: R$ %s\n", number_format($totais['valor'], 2, ',', '.'));
printf("Impostos:       R$ %s\n", number_format($totais['impostos'], 2, ',', '.'));
printf("Frete:          R$ %s\n", number_format($totais['frete'], 2, ',', '.'));
printf("Descontos:      R$ %s\n", number_format($totais['desconto'], 2, ',', '.'));
printf("TOTAL FINAL:    R$ %s\n", number_format($totais['final'], 2, ',', '.'));

/*
100.000.000 processados | 17.14s
200.000.000 processados | 34.45s
300.000.000 processados | 51.92s
400.000.000 processados | 69.57s
500.000.000 processados | 86.87s
600.000.000 processados | 103.71s
700.000.000 processados | 120.37s
800.000.000 processados | 137.43s
900.000.000 processados | 154.50s
1.000.000.000 processados | 208.31s
Processados: 1.000.000.000 pedidos em 208.31s (4800451/s)
Valor produtos: R$ 252.499.706.518,10
Impostos: R$ 69.816.168.851,88
Frete: R$ 152.846.727.856,57
Descontos: R$ 6.564.992.713,12
TOTAL FINAL: R$ 468.597.611.478,61
*/
