use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;
use std::fs;

const STELLAR_HORIZON_URL: &str = "https://horizon.stellar.org/accounts/";
const EXCHANGE_RATE_API: &str = "https://api.exchangerate-api.com/v4/latest/USD";
const CRYPTO_API: &str =
    "https://api.coingecko.com/api/v3/simple/price?ids=stellar%2Cbitcoin&vs_currencies=usd";

fn main() -> Result<(), Box<dyn Error>> {
    // Lendo a chave pública do arquivo
    let key_path = "stellar_key.txt";
    let public_key = fs::read_to_string(key_path)?.trim().to_string();

    // Obtendo saldo de XML
    let balance = get_stellar_balance(&public_key)?;
    println!("Saldo XML: {:.7} XML", balance);

    // Obtendo cotações
    let (usd_to_brl, usd_to_eur, xml_to_usd, btc_to_usd) = get_exchange_rates()?;

    // Convertendo os valores
    let value_in_usd = balance * xml_to_usd;
    let value_in_brl = value_in_usd * usd_to_brl;
    let value_in_eur = value_in_usd * usd_to_eur;
    let value_in_btc = value_in_usd / btc_to_usd;

    println!("Valor em USD: {:.7} USD", value_in_usd);
    println!("Valor em BRL: {:.7} BRL", value_in_brl);
    println!("Valor em EUR: {:.7} EUR", value_in_eur);
    println!("Valor em BTC: {:.7} BTC", value_in_btc);

    Ok(())
}

fn get_stellar_balance(public_key: &str) -> Result<f64, Box<dyn Error>> {
    let url = format!("{}{}", STELLAR_HORIZON_URL, public_key);
    let client = Client::new();
    let response = client.get(&url).send()?;
    let json: Value = response.json()?;

    let balance = json["balances"]
        .as_array()
        .ok_or("Erro ao processar saldo")?
        .iter()
        .find(|b| b["asset_type"] == "native")
        .ok_or("Nenhum saldo XML encontrado")?;

    let xml_balance: f64 = balance["balance"].as_str().unwrap_or("0").parse()?;
    Ok(xml_balance)
}

fn get_exchange_rates() -> Result<(f64, f64, f64, f64), Box<dyn Error>> {
    let client = Client::new();

    // Pegando taxa de câmbio do dólar para outras moedas
    let response = client.get(EXCHANGE_RATE_API).send()?;
    let json: Value = response.json()?;
    let usd_to_brl = json["rates"]["BRL"].as_f64().ok_or("Erro na cotação BRL")?;
    let usd_to_eur = json["rates"]["EUR"].as_f64().ok_or("Erro na cotação EUR")?;

    // Pegando cotação do XML e BTC em relação ao USD
    let response = client.get(CRYPTO_API).send()?;
    let json: Value = response.json()?;
    let xml_to_usd = json["stellar"]["usd"]
        .as_f64()
        .ok_or("Erro na cotação XML")?;
    let btc_to_usd = json["bitcoin"]["usd"]
        .as_f64()
        .ok_or("Erro na cotação BTC")?;

    Ok((usd_to_brl, usd_to_eur, xml_to_usd, btc_to_usd))
}
