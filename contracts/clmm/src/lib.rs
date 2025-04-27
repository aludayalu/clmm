#![no_std]
use soroban_sdk::{contract, contractimpl, token, Address, Env, IntoVal, Symbol, TryFromVal, Val, contracttype,
    Map
};

#[contract]
pub struct CLMM;


fn finish_function(env: Env) {
    env.storage().instance().extend_ttl(50, 100);
}

fn get_value<T>(env: &Env, key: &str) -> T where T: TryFromVal<Env, Val> {
    env.storage().instance().get::<Symbol, T>(&Symbol::new(env, key)).expect("Required storage key not found")
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
    current_price: u128, //  number of a per b also price is price * 1e6 so we can represent upto 6 decimal places
    current_tick: i32,
    current_liquidity_a: i32,
    current_liquidity_b: i32,
    ticks: Map<i32, Tick>,
    positions: Map<u128, Position>

}

#[contracttype]
pub struct Tick {
    index: i32,
    delta_a: u128,
    delta_b: u128
}

#[contracttype]
pub struct Position {
    user: Address,
    starting_tick: i32,
    tick_offset: u32,
    amount_a: u128,
    amount_b: u128
}

#[contractimpl]
impl CLMM {
    pub fn init(env: Env, mint_a: Address, mint_b: Address, starting_price: u128) {
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
            current_liquidity_a: 0,
            current_liquidity_b: 0,
            ticks: Map::new(&env),
            positions: Map::new(&env)
        };

        set_value(&env, "state", &state);
        
        let tick_0 = Tick{
            index: 0,
            delta_a: 0,
            delta_b: 0
        };
        
        set_value(&env, "tick_0", &tick_0);

        finish_function(env);
    }

    pub fn open_position(env: Env, amount_a_in: u128, tick_offset: u32, user: Address, random_position_id: u128) {
        let config: Config = get_value(&env, "config");
        let mut state: PoolState = get_value(&env, "state");

        let amount_b_in = amount_a_in * 1000000 / state.current_price; // as the current price is multiplied by 1e6

        let token_client_a = token::Client::new(&env, &config.mint_a);
        let token_client_b = token::Client::new(&env, &config.mint_b);

        token_client_a.transfer(&user, &env.current_contract_address(), &(amount_a_in as i128));
        token_client_b.transfer(&user, &env.current_contract_address(), &(amount_b_in as i128));

        let lower_tick_index = state.current_tick - (tick_offset as i32);
        let upper_tick_index = state.current_tick + (tick_offset as i32);

        if !state.ticks.contains_key(lower_tick_index) {
            state.ticks.set(lower_tick_index, Tick { index: lower_tick_index, delta_a: 0, delta_b: 0 });
        }

        let mut lower_tick = state.ticks.get(lower_tick_index).unwrap();

        if !state.ticks.contains_key(upper_tick_index) {
            state.ticks.set(upper_tick_index, Tick { index: lower_tick_index, delta_a: 0, delta_b: 0 });
        }

        let mut upper_tick = state.ticks.get(upper_tick_index).unwrap();

        lower_tick.delta_a += amount_a_in / (tick_offset as u128);
        lower_tick.delta_b += amount_b_in / (tick_offset as u128);
        
        upper_tick.delta_a -= amount_a_in / (tick_offset as u128);
        upper_tick.delta_b -= amount_b_in / (tick_offset as u128);

        state.ticks.set(lower_tick_index, lower_tick);
        state.ticks.set(upper_tick_index, upper_tick);

        state.positions.set(random_position_id, Position { user: user, starting_tick: state.current_tick, tick_offset: tick_offset, amount_a: amount_a_in, amount_b: amount_b_in });

        set_value(&env, "state", &state);

        finish_function(env);

    }
}