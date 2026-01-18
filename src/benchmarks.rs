use ext_php_rs::prelude::*;
use std::time::Instant;
use rayon::prelude::*;

#[php_function]
pub fn meu_ola(name: String) -> String {
    format!("Opa, {}!", name)
}

#[php_function]
pub fn rust_benchmark(iterations: f64) -> PhpResult<Vec<(String, f64)>> {
    let n = iterations as i64;
    let start = Instant::now();

    let mut total_valor = 0.0;
    let mut total_impostos = 0.0;
    let mut total_frete = 0.0;
    let mut total_desconto = 0.0;
    let mut total_final = 0.0;

    for i in 1..=n {
        let valor = (((i % 4951) + 50) as f64) / 10.0;
        let peso = (((i % 500) + 1) as f64) / 10.0;
        let idx = i % 5;
        let prime = (i % 100) < 20;

        let icms = match idx {
            0 => 0.18, // 'SP'
            1 => 0.20, // 'RJ'
            2 => 0.18, // 'MG'
            3 => 0.17, // 'RS'
            4 => 0.19, // 'BA'
            _ => 0.19,
        };

        let impostos = valor * (icms + 0.0925);
        
        let dist = match idx {
            0 => 50.0,   // 'SP'
            1 => 430.0,  // 'RJ'
            2 => 586.0,  // 'MG'
            3 => 1130.0, // 'RS'
            4 => 1446.0, // 'BA'
            _ => 1446.0,
        };

        let mut frete = (peso * 2.5) + (dist * 0.15) + 8.5;
        
        if prime && valor >= 100.0 {
            frete = 0.0;
        }

        let desc = valor * if prime { 0.05 } else { 0.02 };

        let total = valor + impostos + frete - desc;

        total_valor += valor;
        total_impostos += impostos;
        total_frete += frete;
        total_desconto += desc;
        total_final += total;
    }

    let duration = start.elapsed().as_secs_f64();

    Ok(vec![
        ("execution_time".to_string(), duration),
        ("total_valor".to_string(), total_valor),
        ("total_impostos".to_string(), total_impostos),
        ("total_frete".to_string(), total_frete),
        ("total_desconto".to_string(), total_desconto),
        ("total_final".to_string(), total_final),
        ("ops_per_sec".to_string(), n as f64 / duration),
    ])
}

#[inline]
fn processar_pedido(i: i64) -> (f64, f64, f64, f64, f64) {
    let valor = (((i % 4951) + 50) as f64) / 10.0;
    let peso = (((i % 500) + 1) as f64) / 10.0;
    let idx = i % 5;
    let prime = (i % 100) < 20;

    let icms = match idx {
        0 => 0.18, 1 => 0.20, 2 => 0.18,
        3 => 0.17, _ => 0.19,
    };

    let impostos = valor * (icms + 0.0925);
    
    let dist = match idx {
        0 => 50.0, 1 => 430.0, 2 => 586.0,
        3 => 1130.0, _ => 1446.0,
    };

    let mut frete = (peso * 2.5) + (dist * 0.15) + 8.5;
    if prime && valor >= 100.0 {
        frete = 0.0;
    }

    let desc = valor * if prime { 0.05 } else { 0.02 };
    let total = valor + impostos + frete - desc;

    (valor, impostos, frete, desc, total)
}

#[php_function]
pub fn rust_benchmark_parallel(iterations: f64) -> PhpResult<Vec<(String, f64)>> {
    let n = iterations as i64;
    let start = Instant::now();

    let (total_valor, total_impostos, total_frete, total_desconto, total_final) = 
        (1..=n)
            .into_par_iter()
            .map(|i| processar_pedido(i))
            .reduce(
                || (0.0, 0.0, 0.0, 0.0, 0.0),
                |acc, item| {
                    (
                        acc.0 + item.0, // valor
                        acc.1 + item.1, // impostos
                        acc.2 + item.2, // frete
                        acc.3 + item.3, // desconto
                        acc.4 + item.4, // total
                    )
                }
            );

    let duration = start.elapsed().as_secs_f64();

    Ok(vec![
        ("execution_time".to_string(), duration),
        ("total_valor".to_string(), total_valor),
        ("total_impostos".to_string(), total_impostos),
        ("total_frete".to_string(), total_frete),
        ("total_desconto".to_string(), total_desconto),
        ("total_final".to_string(), total_final),
        ("ops_per_sec".to_string(), n as f64 / duration),
        ("cores_used".to_string(), rayon::current_num_threads() as f64),
    ])
}

pub fn register(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(meu_ola))
        .function(wrap_function!(rust_benchmark))
        .function(wrap_function!(rust_benchmark_parallel))
}
