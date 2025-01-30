# 🚀 Stellar Balance Converter

**Stellar Balance Converter** é um projeto introdutório em Rust com foco em aprendizado. Ele consulta o saldo XML de uma carteira Stellar e realiza a conversão para USD, BRL, EUR e BTC utilizando APIs de câmbio e criptomoedas. 🔥

---

## 📌 Funcionalidades

✅ Consulta automática do saldo XML da carteira Stellar  
✅ Conversão do saldo XML para **USD, BRL, EUR e BTC**  
✅ Obtenção de cotações atualizadas via **Exchangerate API** e **CoinGecko API**  
✅ Exibição clara dos valores convertidos  

---

## 🛠️ Requisitos

- **Rust** instalado ([Instalar Rust](https://rustup.rs/))
- Dependências do projeto:

  ```toml
  [dependencies]
  reqwest = { version = "0.11", features = ["blocking", "json"] }
  serde_json = "1.0"
  ```

---

## 🚀 Como usar

1. **Clone o repositório** e acesse o diretório:

   ```sh
   git clone https://github.com/lucenfort/stellar_balance_converter.git
   cd stellar_balance_converter
   ```

2. **Adicione sua chave pública Stellar** ao arquivo `stellar_key.txt` na raiz do projeto, como no exemplo:

   ```
   GABCD123456789EXEMPLODECHAVESTELLARXYZ
   ```

3. **Compile e execute o projeto:**

   ```sh
   cargo run
   ```

---

## ⚙️ Como funciona?

1️⃣ Lê a chave pública do arquivo `stellar_key.txt`  
2️⃣ Obtém o saldo XML da conta Stellar via **Horizon API**  
3️⃣ Obtém cotações de XML/USD, USD/BRL, USD/EUR e BTC/USD  
4️⃣ Converte e exibe os valores em **diferentes moedas**  

---

## 📝 Exemplo de saída

```sh
Saldo XML: 1250.6789000 XML
Valor em USD: 137.8900000 USD
Valor em BRL: 690.4500000 BRL
Valor em EUR: 125.6700000 EUR
Valor em BTC: 0.0023456 BTC
```

---

## 📚 Propósito do Projeto

Este é um projeto introdutório com foco em aprendizado. Ele demonstra como utilizar **Rust** para acessar APIs, manipular JSON e realizar conversões de valores financeiros.

---

Aqui estão os links das documentações oficiais das APIs utilizadas no projeto:

1. **Stellar Horizon API** (Consulta de saldo na rede Stellar):  
   🔗 [https://developers.stellar.org/docs/data/horizon](https://developers.stellar.org/docs/data/horizon)

2. **ExchangeRate-API** (Conversão de moedas FIAT - USD, BRL, EUR):  
   🔗 [https://www.exchangerate-api.com/docs/overview](https://www.exchangerate-api.com/docs/overview)

3. **CoinGecko API** (Cotação de criptomoedas - XML, BTC para USD):  
   🔗 [https://www.coingecko.com/en/api/documentation](https://www.coingecko.com/en/api/documentation)

