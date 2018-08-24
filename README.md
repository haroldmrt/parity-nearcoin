# Parity - fast, light, and robust Ethereum client - Nearcoin fork

Parity is one of the main Ethereum client.
This is a fork of this client implementing local and timed demurrage.

## Build instructions

### Install Rust

```bash
curl https://sh.rustup.rs -sSf | sh
```

### Compile and run Nearcoin node

```bash
cargo run -- --chain nearcoin --jsonrpc-cors all --jsonrpc-apis all --bootnodes enode://712e630b133189ef4a7976d4a9fc26c41014527919d9c63bd4f6a4be70c92f28cee3932835798d950f055a9321e69aaef2e1d3f4cee5159c185f1f4ccdf1a3fe@IP_ADDRESS:30303
```

- `cargo run` build and execute the project. Arguments after `--` are passed to the compiled binary.
- `--bootnodes` is followed by an enode address list.
- You can mine on the chain with the option `--engine-signer 0xADDRESS --password ./pass.txt` (`pass.txt` contains passphrase of the 0xADDRESS account). 0xADDRESS has to be an authority on the chain (authorities are set in `ethcore/res/ethereum/nearcoin.json`)

## How to interact with node

### Option 1: use <https://haroldmrt.github.io/parity-nearcoin/>

Enter commands in developer tools 
Ouvrir les outils dévelopeurs du navigateur avec F12 en rentrer les commandes depuis l'onglet Console.

Cette page permet d'intéragir avec un noeud Nearcoin lancé en local sur l'ordinateur.

### Option 2: installer nodejs et le module web3

```html
https://nodejs.org/en/download/package-manager/
```

Install web3 module with this command (local install):

```bash
npm install web3
```

Run nodejs console with `node`.

Import module and connect to the running Nearcoin node:

```js
var Web3 = require('web3')
web3 = new Web3(new Web3.providers.HttpProvider("http://localhost:8545"))
```

## Interaction avec le noeud Nearcoin: commandes

- Create an account

```js
web3.eth.personal.newAccount(passphrase)
```

- Get personal accounts

```js
var accounts = []
web3.eth.personal.getAccounts().then(res => accounts=res)
```

- Make transaction

```js
web3.eth.personal.sendTransaction({from: accounts[0], to: accounts[1], value: 1000}, passphrase_de_accounts[0])
```

- Get balance

```js
web3.eth.getBalance(account).then(res => console.log(res))
```

- Get current block number

```js
web3.eth.getBlockNumber().then(n => console.log(n));
```

- Set location (need to be an endorser)

```js
web3.eth.personal.sendTransaction({from: endorser, to: "0xffffffffffffffffffffffffffffffffffffffff", data: '0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALLLLLLLL'}, endorser_passphrase)
```

First 20 bytes are the address and last 4 are the geographic coordinates

## Notes

- Config files are located in `~/.local/share/io.parity.ethereum/`. For examples private keys are in `~/.local/share/io.parity.ethereum/keys/nearcoin/`.
- .