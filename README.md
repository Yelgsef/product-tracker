# Product Origin Tracker

## Project Description

**Product Origin Tracker** is a Stellar Soroban smart contract written in Rust for tracking the origin and movement history of physical products on-chain.

The project helps manufacturers, suppliers, distributors, and customers verify where a product came from and how it moved through the supply chain. Each product has a unique `product_id`, basic origin information, a current holder, a current status, and a full transaction history stored in contract storage.

Every successful contract interaction is recorded on the Stellar blockchain, while every product movement is also saved inside the contract as a `ProductTransaction`. This makes the product history transparent, traceable, and easy to query.

This contract is suitable for supply chain tracking, food traceability, luxury goods verification, medicine logistics, agricultural products, or any situation where product provenance matters.

## Core Features

### Product Registration (`register_product`)

A manufacturer can register a new product on-chain by providing:

- Product ID
- Manufacturer wallet address
- Product name
- Product origin
- Batch ID

The contract requires authentication from the `manufacturer` address using `manufacturer.require_auth()`. This prevents someone else from registering a product on behalf of the manufacturer.

When a product is registered, the contract creates:

- A `Product` record
- The first `ProductTransaction` record
- A product history list
- On-chain events for the product and first transaction

The first transaction is automatically marked as:

```text
REGISTERED
```

The product's first holder is the manufacturer.

### Product Transaction Recording (`record_transaction`)

The contract allows a product actor to record a new supply-chain transaction for an existing product.

Each transaction stores:

- Transaction ID
- Product ID
- Sender address
- Receiver address
- Action
- Location
- Notes
- Ledger timestamp

Example actions include:

- `HARVESTED`
- `PACKED`
- `SHIPPED`
- `RECEIVED`
- `SOLD`
- `DELIVERED`

The `from` address must sign the transaction using `from.require_auth()`. After a transaction is recorded, the contract updates the product's:

- Current holder
- Current status
- Transaction count

This creates a clear chain of custody for the product.

### Current Product Query (`get_product`)

Anyone can query the latest state of a product by using its `product_id`.

The returned `Product` includes:

- Product ID
- Name
- Origin
- Batch ID
- Manufacturer
- Current holder
- Current status
- Created timestamp
- Transaction count

This is useful when users only need the latest product status instead of the full history.

### Full History Query (`get_history`)

Anyone can query the full transaction history of a product.

The returned history is a list of `ProductTransaction` records. This makes it possible to verify the complete journey of the product from registration to its latest movement.

### Single Transaction Query (`get_transaction`)

Anyone can query one specific transaction from a product history by providing:

- Product ID
- Transaction ID

Transaction IDs start from `1`. If the transaction ID is `0` or does not exist, the contract returns `InvalidTransactionId`.

### Product Existence Check (`product_exists`)

The contract provides a simple read-only function to check whether a product has already been registered.

It returns:

```text
true
```

or

```text
false
```

This is useful before calling `register_product`, especially from a frontend.

### Error Handling

The contract defines three custom errors:

| Error | Meaning |
| --- | --- |
| `ProductAlreadyExists` | The product ID has already been registered |
| `ProductNotFound` | The product ID does not exist in contract storage |
| `InvalidTransactionId` | The requested transaction ID is invalid or missing |

### Event Logging

The contract emits events when important actions happen.

When a product is registered, it emits:

- A `product` event
- A `tx` event for the first transaction

When a new transaction is recorded, it emits:

- A `tx` event with the product ID and transaction ID

This makes the contract easier to track using blockchain explorers, indexers, or a future frontend.

## Contract Link

Contract on Stellar Lab or Stellar Expert Testnet:

```text
https://stellar.expert/explorer/testnet/tx/8a416b83f0700f3dd37efe9742bbb087281a62f60f3cd6f4a4d7b2c92a13f87a
```

Example format:

```text
https://stellar.expert/explorer/testnet/contract/CCRDUHAWNVEGSYJJ7GFMG6LIIM2635XUFLBVV6LYT7CMCBXJ4DR4JXMY?filter=history
```

## Interaction Screenshots

Add screenshots of the following actions before submitting:

- Successful contract deployment
- Successful `register_product` invocation
- Successful `record_transaction` invocation
- Successful `get_product` result
- Successful `get_history` result

Example filenames:

- `screenshot_deploy.png`
- `screenshot_register_product.png`
- `screenshot_record_transaction.png`
- `screenshot_get_product.png`
- `screenshot_get_history.png`

## How to Build

```bash
stellar contract build
```

## How to Deploy

Replace the WASM path with the actual generated WASM file in your project.

```bash
stellar contract deploy --wasm /app/target/wasm32v1-none/release/hello_world.wasm --source-account alice --network testnet --alias stellar-track
```

## Example Interactions

### Register a product

This creates a new product and automatically creates the first `REGISTERED` transaction.

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

### Record a product transaction

This records a new supply-chain transaction and updates the current holder and status of the product.

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
  --notes "Moved from manufacturer warehouse to distributor"
```

### Get current product information

```bash
stellar contract invoke \
  --id YOUR_CONTRACT_ID_HERE \
  --source-account alice \
  --network testnet \
  -- get_product \
  --product_id 1
```

### Get full product history

```bash
stellar contract invoke \
  --id YOUR_CONTRACT_ID_HERE \
  --source-account alice \
  --network testnet \
  -- get_history \
  --product_id 1
```

### Get one transaction

```bash
stellar contract invoke \
  --id YOUR_CONTRACT_ID_HERE \
  --source-account alice \
  --network testnet \
  -- get_transaction \
  --product_id 1 \
  --transaction_id 1
```

### Check if a product exists

```bash
stellar contract invoke \
  --id YOUR_CONTRACT_ID_HERE \
  --source-account alice \
  --network testnet \
  -- product_exists \
  --product_id 1
```

## Data Model

### Product

```rust
pub struct Product {
    pub product_id: u64,
    pub name: String,
    pub origin: String,
    pub batch_id: String,
    pub manufacturer: Address,
    pub current_holder: Address,
    pub current_status: String,
    pub created_at: u64,
    pub transaction_count: u32,
}
```

### ProductTransaction

```rust
pub struct ProductTransaction {
    pub transaction_id: u32,
    pub product_id: u64,
    pub from: Address,
    pub to: Address,
    pub action: String,
    pub location: String,
    pub notes: String,
    pub timestamp: u64,
}
```

## Future Scope

### Frontend Integration

Build a React.js frontend and integrate it with Freighter Wallet so users can register products, record product movements, and view product history without using terminal commands.

### QR Code Product Lookup

Generate a QR code for each product ID so customers can scan the product and view its origin, batch ID, current status, and full history.

### Role-Based Supply Chain Actors

Add roles such as manufacturer, distributor, retailer, and inspector so that each actor has controlled permissions.

### Verification and Certification

Allow trusted inspectors or certification agencies to add verification records to prove product authenticity or quality.

### Advanced Search and Indexing

Build an indexer or backend service to search products by batch ID, manufacturer, origin, or current status.

## Author Profile

- Full Name: Trung Nguyen
- Role: Student / Blockchain Builder
- GitHub: https://github.com/Yelgsef