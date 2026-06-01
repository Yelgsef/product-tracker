# Product Origin Tracker

Product Origin Tracker is a Stellar Soroban smart contract, written in Rust, for tracking product origin and supply-chain movement on-chain. It stores each product's origin, batch, current holder, current status, and full transaction history.

Use cases include food traceability, luxury goods verification, medicine logistics, agricultural products, and other provenance workflows.

## Features

- Register a product with `register_product`
- Record custody or status changes with `record_transaction`
- Query current product data with `get_product`
- Query full movement history with `get_history`
- Query one transaction with `get_transaction`
- Check registration status with `product_exists`
- Emit events for product registration and transactions

## Errors

| Error | Meaning |
| --- | --- |
| `ProductAlreadyExists` | Product ID is already registered |
| `ProductNotFound` | Product ID does not exist |
| `InvalidTransactionId` | Transaction ID is invalid or missing |

## Contract Link

Testnet transaction:

```text
https://stellar.expert/explorer/testnet/tx/8a416b83f0700f3dd37efe9742bbb087281a62f60f3cd6f4a4d7b2c92a13f87a
```

Example contract page:

```text
https://stellar.expert/explorer/testnet/contract/CCRDUHAWNVEGSYJJ7GFMG6LIIM2635XUFLBVV6LYT7CMCBXJ4DR4JXMY?filter=history
```

## Build

```bash
stellar contract build
```

## Deploy

Replace the WASM path with your generated contract file.

```bash
stellar contract deploy --wasm /app/target/wasm32v1-none/release/hello_world.wasm --source-account alice --network testnet --alias stellar-track
```

## Example Interactions

### Register a product

```bash
stellar contract invoke \
  --id YOUR_CONTRACT_ID_HERE \
  --source-account alice \
  --network testnet \
  -- register_product \
  --product_id 1 \
  --manufacturer alice \
  --name "Vietnam Coffee" \
  --origin "Da Lat, Vietnam" \
  --batch_id "BATCH-2026-001"
```

### Record a transaction

```bash
stellar contract invoke \
  --id YOUR_CONTRACT_ID_HERE \
  --source-account alice \
  --network testnet \
  -- record_transaction \
  --product_id 1 \
  --from alice \
  --to bob \
  --action "SHIPPED" \
  --location "Ho Chi Minh City, Vietnam" \
  --notes "Moved to distributor"
```

### Query product data

```bash
stellar contract invoke --id YOUR_CONTRACT_ID_HERE --source-account alice --network testnet -- get_product --product_id 1
stellar contract invoke --id YOUR_CONTRACT_ID_HERE --source-account alice --network testnet -- get_history --product_id 1
stellar contract invoke --id YOUR_CONTRACT_ID_HERE --source-account alice --network testnet -- get_transaction --product_id 1 --transaction_id 1
stellar contract invoke --id YOUR_CONTRACT_ID_HERE --source-account alice --network testnet -- product_exists --product_id 1
```

## Author

Trung Nguyen  
Student / Blockchain Builder  
GitHub: https://github.com/Yelgsef
