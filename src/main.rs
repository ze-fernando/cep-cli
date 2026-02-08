use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    cep: String,
    logradouro: String,
    complemento: String,
    unidade: String,
    bairro: String,
    localidade: String,
    uf: String,
    estado: String,
    regiao: String,
    ibge: String,
    gia: String,
    ddd: String,
    siafi: String,
}

fn get_info(cep: Box<str>) -> Result<(), Box<dyn Error>> {
    let url = format!("https://viacep.com.br/ws/{}/json/", cep);
    let res = reqwest::blocking::get(url)?;

    let body = res.text()?;
    let data: Response = serde_json::from_str(&body)?;

    Command::new("clear").status().unwrap();

    println!("{}", "=".repeat(22));
    println!("Resultado");
    println!("{}", "=".repeat(22));
    println!("CEP: {:#?}", data.cep);
    println!("Logradouro: {:#?}", data.logradouro);
    println!("Complemento: {:#?}", data.complemento);
    println!("Unidade: {:#?}", data.unidade);
    println!("Bairro: {:#?}", data.bairro);
    println!("Localidade: {:#?}", data.localidade);
    println!("UF: {:#?}", data.uf);
    println!("Estado: {:#?}", data.estado);
    println!("Região: {:#?}", data.regiao);
    println!("IBGE: {:#?}", data.ibge);
    println!("GIA: {:#?}", data.gia);
    println!("DDD: {:#?}", data.ddd);
    println!("Siafi: {:#?}", data.siafi);
    println!("{}", "=".repeat(22));
    Ok(())
}

fn draw_menu() {
    println!("");
    println!("{}", "=".repeat(25));
    println!("{}{}{}", "=".repeat(5), "Buscador de CEP", "=".repeat(5));
    println!("{}", "=".repeat(25));
    println!("1) Buscar Cep");
    println!("0) Sair");
    println!("{}", "=".repeat(25));
}

fn main() {
    loop {
        let _ = draw_menu();

        print!("Digite uma opção: ");
        io::stdout().flush().unwrap();
        let mut op = String::new();
        io::stdin().read_line(&mut op).expect("Falha ao ler opção");

        if op.trim() == "1" {
            print!("Digite o cep: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Falha ao ler o cep");

            if let Err(e) = get_info(input.into()) {
                eprintln!("Ocorreu um erro: {}", e);
            }
        } else if op.trim() == "0" {
            println!("\n\tObrigado por usar!");
            break;
        }
    }
}
