$ near dev-deploy out/main.wasm 
Starting deployment. Account id: dev-1644749916032-36742738579718, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: out/main.wasm
Transaction Id 8okwhqNFRELYqecWWMX8Vmw7Kh3huLQXT717C9TMnDJr
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/8okwhqNFRELYqecWWMX8Vmw7Kh3huLQXT717C9TMnDJr
Done deploying to dev-1644749916032-36742738579718

$ near call dev-1644749916032-36742738579718 new '{"owner_id": "harrynguyen.testnet"}' --accountId harrynguyen.testnet
Scheduling a call: dev-1644749916032-36742738579718.new({"owner_id": "harrynguyen.testnet"})
Doing account.functionCall()
Transaction Id FqEmWCWBCg4QRQxrJ1pLj5ihkgiLsnCH76SW26G6xiwg
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/FqEmWCWBCg4QRQxrJ1pLj5ihkgiLsnCH76SW26G6xiwg
''

near call dev-1644749916032-36742738579718 add_campaign '{
    "campaign": {
        "sale_contract": "akux.sale.tokenhub.testnet",
        "reference": "https://akux.json",
        "reference_hash": "Cg=="
    }
}' --accountId harrynguyen.testnet --depositYocto 1

near call dev-1644749916032-36742738579718 add_campaign '{
    "campaign": {
        "sale_contract": "akux2.sale.tokenhub.testnet",
        "reference": "https://akux2.json",
        "reference_hash": "Cg=="
    }
}' --accountId harrynguyen.testnet --depositYocto 1

$ near view dev-1644749916032-36742738579718 get_campaign '{"id": 1}'
View call: dev-1644749916032-36742738579718.get_campaign({"id": 1})
{
  sale_contract: 'akux.sale.tokenhub.testnet',
  reference: 'https://akux.json',
  reference_hash: 'Cg=='
}

$ near view dev-1644749916032-36742738579718 get_campaign_list '{"from_index": 0, "limit": 2}'
View call: dev-1644749916032-36742738579718.get_campaign_list({"from_index": 0, "limit": 2})
[
  [
    1,
    {
      sale_contract: 'akux.sale.tokenhub.testnet',
      reference: 'https://akux.json',
      reference_hash: 'Cg=='
    }
  ],
  [
    2,
    {
      sale_contract: 'akux2.sale.tokenhub.testnet',
      reference: 'https://akux2.json',
      reference_hash: 'Cg=='
    }
  ]
]

$ near call dev-1644749916032-36742738579718 remove_campaign '{"id": 2}' --accountId harrynguyen.testnet --depositYocto 1

static files in github: https://raw.githubusercontent.com/user/repository/branch/filename
eg:
$ curl https://raw.githubusercontent.com/lightsea90/token-factory/main/package.json

get sha256
$ curl -s https://raw.githubusercontent.com/lightsea90/token-factory/main/package.json | sha256sum 2>/dev/null
403c409b69b67fcb15493d97c9b9fdcc2d1de25609b7ea46469ed582d8fc3693  -


sales.tokenhub.testnet
