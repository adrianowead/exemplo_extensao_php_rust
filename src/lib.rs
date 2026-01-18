#![cfg_attr(windows, feature(abi_vectorcall))]

mod benchmarks;
mod wead;

use ext_php_rs::{
    prelude::*,
    zend::ModuleEntry,
};

pub extern "C" fn php_module_info(_module: *mut ModuleEntry) {
    use ext_php_rs::{info_table_start, info_table_row, info_table_end, info_table_header};

    info_table_start!();

    info_table_header!("Metadata", "Value");
    info_table_row!("Extension", "meu_hello_world"); 
    info_table_row!("Version", env!("CARGO_PKG_VERSION"));
    info_table_row!("Author", "Adriano Maciel");
    info_table_row!("Author GitHub", "https://github.com/adrianowead");
    info_table_row!("Description", "Mãe, tô no phpinfo!");

    info_table_end!();
}

/// Função de registro do módulo PHP
/// Registra classes, interfaces e funções no PHP
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    let module = module.info_function(php_module_info);
    
    let module = benchmarks::register(module);
    let module = wead::register(module);
    
    module
}