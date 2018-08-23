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

### Option 1: utiliser la page web <https://dev.nas.mrtz.fr/nearcoin/>

Ouvrir les outils dévelopeurs du navigateur avec F12 en rentrer les commandes depuis l'onglet Console.

Cette page permet d'intéragir avec un noeud Nearcoin lancé en local sur l'ordinateur.

### Option 2: installer nodejs et le module web3

```html
https://nodejs.org/en/download/package-manager/
```

Installer le module web3 avec la commande (*installation dans le dossier courant*):

```bash
npm install web3
```

(pour une installation globale, utiliser `sudo npm -g install web3`)

Lancer la console nodejs avec `node`.

Importer le module et se connecter au noeud Nearcoin précédemment lancé (ici en local)

```js
var Web3 = require('web3')
web3 = new Web3(new Web3.providers.HttpProvider("http://localhost:8545"))
```

Le shell est maintenant utilisable.

### Option 3: utiliser la page web du repo stage_fonte

La page web de l'option 1 est disponible dans le repo stage_fonte, dans le dossier frontend.

Il faut servir le dossier frontend, par exemple ave `php -s localhost:8000`

## Interaction avec le noeud Nearcoin: commandes

- Créer un compte

```js
web3.eth.personal.newAccount(passphrase)
```

- Récupérer les comptes personnels

```js
var accounts = []
web3.eth.personal.getAccounts().then(res => accounts=res)
```

- Effectuer une transaction

```js
web3.eth.personal.sendTransaction({from: accounts[0], to: accounts[1], value: 1000}, passphrase_de_accounts[0])
```

- Obtenir la balance d'un compte

```js
web3.eth.getBalance(account).then(res => console.log(res))
```

- Obtenir le numéro du bloc courant

```js
web3.eth.getBlockNumber().then(n => console.log(n));
```

- Fixer une localisation

```js
web3.eth.personal.sendTransaction({from: validator, to: "0xffffffffffffffffffffffffffffffffffffffff", data: '0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALLLLLLLL'}, passphrase_de_validator)
```

- Envoyer une transaction de test

```js
web3.eth.personal.sendTransaction({from: accounts[0], to: "0xDf0B3cC84f07B06c1B87b629C4E0537e1AdD28BC", value: 100}, passphrase)
```

```js
web3.eth.getBalance("0xffffffffffffffffffffffffffffffffffffffff").then(res => console.log(res))
```

(les 20 premiers octets sont l'adresse et les 4 derniers les coordonnées géographiques)

## Notes

- Les fichiers de configuration sont situés dans `~/.local/share/io.parity.ethereum/`. Par exemple les clefs privées des comptes créés sont dans `~/.local/share/io.parity.ethereum/keys/nearcoin/`.
- .