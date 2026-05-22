// lib.rs
#![no_std]

use soroban_sdk::{#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, String, Vec,
};

/// ProductOriginTracker
///
/// A simple Stellar Soroban smart contract for product provenance.
/// It stores one immutable-style history list per product. Every successful
/// contract invocation is written to Stellar, and every product transaction is
/// also saved inside the contract storage for easy querying.
///
/// Typical flow:
/// 1. register_product(...)
/// 2. record_transaction(...)
/// 3. get_product(...)
/// 4. get_history(...)
#[contract]
pub struct ProductOriginTracker;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Product(u64),
    History(u64),
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
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

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
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

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    ProductAlreadyExists = 1,
    ProductNotFound = 2,
    InvalidTransactionId = 3,
}

#[contractimpl]
impl ProductOriginTracker {
    /// Registers a new product and creates the first provenance transaction.
    ///
    /// The manufacturer must sign this transaction with their wallet.
    pub fn register_product(
        env: Env,
        product_id: u64,
        manufacturer: Address,
        name: String,
        origin: String,
        batch_id: String,
    ) -> Result<Product, Error> {
        manufacturer.require_auth();

        let product_key = DataKey::Product(product_id);
        if env.storage().persistent().has(&product_key) {
            return Err(Error::ProductAlreadyExists);
        }

        let now = env.ledger().timestamp();
        let registered = String::from_str(&env, "REGISTERED");

        let product = Product {
            product_id,
            name,
            origin,
            batch_id,
            manufacturer: manufacturer.clone(),
            current_holder: manufacturer.clone(),
            current_status: registered.clone(),
            created_at: now,
            transaction_count: 1,
        };

        let first_tx = ProductTransaction {
            transaction_id: 1,
            product_id,
            from: manufacturer.clone(),
            to: manufacturer.clone(),
            action: registered,
            location: product.origin.clone(),
            notes: String::from_str(&env, "Product registered at origin"),
            timestamp: now,
        };

        let mut history = Vec::new(&env);
        history.push_back(first_tx.clone());

        env.storage().persistent().set(&product_key, &product);
        env.storage()
            .persistent()
            .set(&DataKey::History(product_id), &history);

        env.events()
            .publish((symbol_short!("product"), product_id), product.clone());
        env.events()
            .publish((symbol_short!("tx"), product_id, 1u32), first_tx);

        Ok(product)
    }

    /// Records a new product transaction / supply-chain event.
    ///
    /// Example actions: "HARVESTED", "PACKED", "SHIPPED", "RECEIVED", "SOLD".
    /// The `from` address must sign, proving that the current actor approved this record.
    pub fn record_transaction(
        env: Env,
        product_id: u64,
        from: Address,
        to: Address,
        action: String,
        location: String,
        notes: String,
    ) -> Result<ProductTransaction, Error> {
        from.require_auth();

        let product_key = DataKey::Product(product_id);
        let history_key = DataKey::History(product_id);

        let mut product: Product = env
            .storage()
            .persistent()
            .get(&product_key)
            .ok_or(Error::ProductNotFound)?;

        let mut history: Vec<ProductTransaction> = env
            .storage()
            .persistent()
            .get(&history_key)
            .ok_or(Error::ProductNotFound)?;

        let transaction_id = history.len() + 1;
        let new_tx = ProductTransaction {
            transaction_id,
            product_id,
            from,
            to: to.clone(),
            action: action.clone(),
            location,
            notes,
            timestamp: env.ledger().timestamp(),
        };

        history.push_back(new_tx.clone());

        product.current_holder = to;
        product.current_status = action;
        product.transaction_count = transaction_id;

        env.storage().persistent().set(&product_key, &product);
        env.storage().persistent().set(&history_key, &history);

        env.events()
            .publish((symbol_short!("tx"), product_id, transaction_id), new_tx.clone());

        Ok(new_tx)
    }

    /// Returns current product state.
    pub fn get_product(env: Env, product_id: u64) -> Result<Product, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Product(product_id))
            .ok_or(Error::ProductNotFound)
    }

    /// Returns the complete transaction history of a product.
    pub fn get_history(env: Env, product_id: u64) -> Result<Vec<ProductTransaction>, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::History(product_id))
            .ok_or(Error::ProductNotFound)
    }

    /// Returns one transaction from a product's history.
    /// transaction_id starts at 1.
    pub fn get_transaction(
        env: Env,
        product_id: u64,
        transaction_id: u32,
    ) -> Result<ProductTransaction, Error> {
        if transaction_id == 0 {
            return Err(Error::InvalidTransactionId);
        }

        let history: Vec<ProductTransaction> = env
            .storage()
            .persistent()
            .get(&DataKey::History(product_id))
            .ok_or(Error::ProductNotFound)?;

        history
            .get(transaction_id - 1)
            .ok_or(Error::InvalidTransactionId)
    }

    /// Returns true if the product has already been registered.
    pub fn product_exists(env: Env, product_id: u64) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Product(product_id))
    }
}
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, String, Vec,
};

/// ProductOriginTracker
///
/// A simple Stellar Soroban smart contract for product provenance.
/// It stores one immutable-style history list per product. Every successful
/// contract invocation is written to Stellar, and every product transaction is
/// also saved inside the contract storage for easy querying.
///
/// Typical flow:
/// 1. register_product(...)
/// 2. record_transaction(...)
/// 3. get_product(...)
/// 4. get_history(...)
#[contract]
pub struct ProductOriginTracker;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Product(u64),
    History(u64),
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
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

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
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

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    ProductAlreadyExists = 1,
    ProductNotFound = 2,
    InvalidTransactionId = 3,
}

#[contractimpl]
impl ProductOriginTracker {
    /// Registers a new product and creates the first provenance transaction.
    ///
    /// The manufacturer must sign this transaction with their wallet.
    pub fn register_product(
        env: Env,
        product_id: u64,
        manufacturer: Address,
        name: String,
        origin: String,
        batch_id: String,
    ) -> Result<Product, Error> {
        manufacturer.require_auth();

        let product_key = DataKey::Product(product_id);
        if env.storage().persistent().has(&product_key) {
            return Err(Error::ProductAlreadyExists);
        }

        let now = env.ledger().timestamp();
        let registered = String::from_str(&env, "REGISTERED");

        let product = Product {
            product_id,
            name,
            origin,
            batch_id,
            manufacturer: manufacturer.clone(),
            current_holder: manufacturer.clone(),
            current_status: registered.clone(),
            created_at: now,
            transaction_count: 1,
        };

        let first_tx = ProductTransaction {
            transaction_id: 1,
            product_id,
            from: manufacturer.clone(),
            to: manufacturer.clone(),
            action: registered,
            location: product.origin.clone(),
            notes: String::from_str(&env, "Product registered at origin"),
            timestamp: now,
        };

        let mut history = Vec::new(&env);
        history.push_back(first_tx.clone());

        env.storage().persistent().set(&product_key, &product);
        env.storage()
            .persistent()
            .set(&DataKey::History(product_id), &history);

        env.events()
            .publish((symbol_short!("product"), product_id), product.clone());
        env.events()
            .publish((symbol_short!("tx"), product_id, 1u32), first_tx);

        Ok(product)
    }

    /// Records a new product transaction / supply-chain event.
    ///
    /// Example actions: "HARVESTED", "PACKED", "SHIPPED", "RECEIVED", "SOLD".
    /// The `from` address must sign, proving that the current actor approved this record.
    pub fn record_transaction(
        env: Env,
        product_id: u64,
        from: Address,
        to: Address,
        action: String,
        location: String,
        notes: String,
    ) -> Result<ProductTransaction, Error> {
        from.require_auth();

        let product_key = DataKey::Product(product_id);
        let history_key = DataKey::History(product_id);

        let mut product: Product = env
            .storage()
            .persistent()
            .get(&product_key)
            .ok_or(Error::ProductNotFound)?;

        let mut history: Vec<ProductTransaction> = env
            .storage()
            .persistent()
            .get(&history_key)
            .ok_or(Error::ProductNotFound)?;

        let transaction_id = history.len() + 1;
        let new_tx = ProductTransaction {
            transaction_id,
            product_id,
            from,
            to: to.clone(),
            action: action.clone(),
            location,
            notes,
            timestamp: env.ledger().timestamp(),
        };

        history.push_back(new_tx.clone());

        product.current_holder = to;
        product.current_status = action;
        product.transaction_count = transaction_id;

        env.storage().persistent().set(&product_key, &product);
        env.storage().persistent().set(&history_key, &history);

        env.events()
            .publish((symbol_short!("tx"), product_id, transaction_id), new_tx.clone());

        Ok(new_tx)
    }

    /// Returns current product state.
    pub fn get_product(env: Env, product_id: u64) -> Result<Product, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Product(product_id))
            .ok_or(Error::ProductNotFound)
    }

    /// Returns the complete transaction history of a product.
    pub fn get_history(env: Env, product_id: u64) -> Result<Vec<ProductTransaction>, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::History(product_id))
            .ok_or(Error::ProductNotFound)
    }

    /// Returns one transaction from a product's history.
    /// transaction_id starts at 1.
    pub fn get_transaction(
        env: Env,
        product_id: u64,
        transaction_id: u32,
    ) -> Result<ProductTransaction, Error> {
        if transaction_id == 0 {
            return Err(Error::InvalidTransactionId);
        }

        let history: Vec<ProductTransaction> = env
            .storage()
            .persistent()
            .get(&DataKey::History(product_id))
            .ok_or(Error::ProductNotFound)?;

        history
            .get(transaction_id - 1)
            .ok_or(Error::InvalidTransactionId)
    }

    /// Returns true if the product has already been registered.
    pub fn product_exists(env: Env, product_id: u64) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Product(product_id))
    }
}