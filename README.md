[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](docs.interstate.so)
[![Chat](https://img.shields.io/endpoint?color=neon&logo=telegram&label=chat&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2F%2BPcs9bykxK3BiMzk5)]([https://t.me/+Pcs9bykxK3BiMzk5](https://t.me/+-i4dP7U2BggxMzAx))
[![X](https://img.shields.io/twitter/follow/interstatefdn)](https://x.com/interstatefdn)

# Interstate sidecar to send preconfirmation requests to commit-boost.

![Multiparty Preconfirmation Flow](static/multipartypreconf.png)
Interstate's design allows multiple preconfirmations sidecars to run on the same validator. This design promotes decentralization: https://docs.interstate.so/research/multiparty-preconf. 

## How it works
![Interstate protocol sequence chart](static/interstate-sequence.png)


## How to install
`
git clone https://github.com/interstate-labs/proposer-commitment-network.git
cd proposer-commitment-network
`

## Using cargo
`
cargo build
cargo run
`

## Using docker
`
docker build -t whereasjovially/interstate-boost:1.1.0-dev .
docker compose up -d
`

# Preconfirmation request RPC interface
## Endpoint
`{SIDECAR_URL}/api/v1/preconfirmation`
## Method
`Post`
## Headers
`Content-Type:application/json`
## Body
```
{
  tx: signed raw transaction,
  slot: target slot number,
  sender: signer address
}
```
## Response
`ok:true`
