#![no_std]
use core::ops::Add;

use soroban_sdk::{contract, contractimpl, log, token, Address, Env, IntoVal, Symbol, TryFromVal, Val, U256, contracttype
};

#[contract]
pub struct CLMM;


fn finish_function(env: Env) {
    env.storage().instance().extend_ttl(50, 100);
}

fn get_value<T>(env: &Env, key: &str) -> T where T: TryFromVal<Env, Val> + Default {
    env.storage().instance().get::<Symbol, T>(&Symbol::new(env, key)).unwrap_or_default()
}

fn set_value<T>(env: &Env, key: &str, value: &T) where T: IntoVal<Env, soroban_sdk::Val> {
    env.storage().instance().set(&Symbol::new(env, key), value);
}

pub fn key_exists(env: &Env, key: &str) -> bool {
    env.storage().instance().has(&Symbol::new(env, key))
}

#[contracttype]
pub struct Config {
    mint_a: Address,
    mint_b: Address
}

#[contracttype]
pub struct PoolState {
    current_price: U256,
    current_tick: i32,
    current_liquidity: i32
}

#[contracttype]
pub struct Tick {
    index: i32,
    liquidity_delta: i32
}

#[contractimpl]
impl CLMM {
    pub fn init(env: Env, mint_a: Address, mint_b: Address, starting_price: U256) {
        if key_exists(&env, "config") {
            return;
        }

        let config = Config {
            mint_a, mint_b
        };

        set_value(&env, "config", &config);

        let state = PoolState{
            current_price: starting_price,
            current_tick: 0,
            current_liquidity: 0
        };

        set_value(&env, "state", &state);
        
        let tick_0 = Tick{
            index: 0,
            liquidity_delta: 0
        };
        
        set_value(&env, "tick_0", &tick_0);

        finish_function(env);
    }
}