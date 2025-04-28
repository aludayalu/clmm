#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, log, Address, Env, IntoVal, Map, Symbol, TryFromVal, Val
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
pub struct PoolState {
    current_price: u128, //  number of XLM per a also price is price * 1e6 so we can represent upto 6 decimal places
    current_tick: i32,
    base_price_at_tick: u128,
    current_liquidity_a: u128,
    current_liquidity_b: u128,
    ticks: Map<i32, Tick>,
    positions: Map<u128, Position>,
    balances: Map<Address, u128>
}

#[contracttype]
pub struct Tick {
    index: i32,
    delta_a: i128,
    delta_b: i128
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
    pub fn init(env: Env, user: Address, starting_price: u128) -> bool {
        if key_exists(&env, "state") {
            return false;
        }

        let mut state = PoolState{
            current_price: starting_price,
            current_tick: 0,
            base_price_at_tick: starting_price,
            current_liquidity_a: 0,
            current_liquidity_b: 0,
            ticks: Map::new(&env),
            positions: Map::new(&env),
            balances: Map::new(&env),
        };

        state.balances.set(user, 1000000);

        let tick_0 = Tick{
            index: 0,
            delta_a: 0,
            delta_b: 0
        };

        state.ticks.set(0, tick_0);

        set_value(&env, "state", &state);

        finish_function(env);

        return true;
    }

    pub fn open_position(env: Env, amount_a_in: u128, tick_offset: u32, user: Address, random_position_id: u128) -> bool {
        let mut state: PoolState = get_value(&env, "state");

        let amount_b_in = (amount_a_in * 1000000) / state.current_price; // as the current price is multiplied by 1e6

        let mut user_b_balance = state.balances.try_get(user.clone()).unwrap().expect("bruh");

        if user_b_balance < amount_b_in {
            return false;
        }

        user_b_balance = user_b_balance - amount_b_in;

        state.balances.set(user.clone(), user_b_balance);

        let lower_tick_index = state.current_tick - (tick_offset as i32);
        let upper_tick_index = state.current_tick + (tick_offset as i32);

        if !state.ticks.contains_key(lower_tick_index) {
            state.ticks.set(lower_tick_index, Tick { index: lower_tick_index, delta_a: 0, delta_b: 0 });
        }

        let mut lower_tick = state.ticks.get(lower_tick_index).unwrap();

        if !state.ticks.contains_key(upper_tick_index) {
            state.ticks.set(upper_tick_index, Tick { index: upper_tick_index, delta_a: 0, delta_b: 0 });
        }

        let mut upper_tick = state.ticks.get(upper_tick_index).unwrap();

        lower_tick.delta_a += (amount_a_in / (tick_offset as u128)) as i128;
        lower_tick.delta_b += (amount_b_in / (tick_offset as u128)) as i128;
        
        upper_tick.delta_a -= (amount_a_in / (tick_offset as u128)) as i128;
        upper_tick.delta_b -= (amount_b_in / (tick_offset as u128)) as i128;

        state.ticks.set(lower_tick_index, lower_tick);
        state.ticks.set(upper_tick_index, upper_tick);

        state.positions.set(random_position_id, Position { user: user, starting_tick: state.current_tick, tick_offset: tick_offset, amount_a: amount_a_in, amount_b: amount_b_in });

        state.current_liquidity_a += (amount_a_in / (tick_offset as u128));
        state.current_liquidity_b += (amount_b_in / (tick_offset as u128));

        set_value(&env, "state", &state);

        finish_function(env);

        return true;
    }

    pub fn get_balance(env: Env, user: Address) -> u128 {
        let state: PoolState = get_value(&env, "state");
        return state.balances.try_get(user).unwrap().expect("bruh")
    }

    pub fn swap(env: Env, amountIn: u128, isXLM: bool, user: Address) -> i128 {
        let mut state: PoolState = get_value(&env, "state");
        let mut amountRemaining = amountIn;
        let mut toGive = 0;
        env.events().publish((), "1");
        if isXLM {
            let mut iterations = 0;
            while iterations < 100 {
                iterations += 1;
                if !state.ticks.contains_key(state.current_tick) {
                    state.ticks.set(state.current_tick, Tick { index: state.current_tick, delta_a: 0, delta_b: 0 });
                }
                env.events().publish((), "2");
                let mut current_tick = state.ticks.get(state.current_tick).unwrap();
                let max_can_provide_swap = (state.current_price * state.current_liquidity_b) / 1000000;
                env.events().publish((), amountIn);
                env.events().publish((), max_can_provide_swap);
                if max_can_provide_swap > amountRemaining {
                    env.events().publish((), "4");
                    let mut initialRatioThousand = state.current_liquidity_b * 1000;
                    if state.current_liquidity_a != 0 {
                        initialRatioThousand /= state.current_liquidity_a;
                    }
                    env.events().publish((), "5");
                    state.current_liquidity_a += amountRemaining;
                    current_tick.delta_a += amountRemaining as i128;
                    env.events().publish((), "6");
                    toGive = (amountRemaining * 1000000) / state.current_price;
                    state.current_liquidity_b-=toGive;
                    env.events().publish((), "7");
                    current_tick.delta_b -= toGive as i128;
                    state.ticks.set(state.current_tick, current_tick);
                    env.events().publish((), "8");
                    let priceLower = state.base_price_at_tick;
                    let priceUpper = (state.base_price_at_tick * 110) / 100;
                    let newRatioThousand = (state.current_liquidity_b * 1000 / state.current_liquidity_a);
                    let netChange = ((state.current_price) * (newRatioThousand * 100) / initialRatioThousand) / (100 * 100);
                    state.current_price = state.current_price + netChange;
                    env.events().publish((), "9");
                    amountRemaining = 0;
                    break;
                } else {
                    state.current_liquidity_a += max_can_provide_swap;
                    current_tick.delta_a += max_can_provide_swap as i128;
                    state.current_price = (state.base_price_at_tick * 110) / 100;
                    toGive += state.current_liquidity_b;
                    current_tick.delta_b -= state.current_liquidity_b as i128;
                    state.current_liquidity_b = 0;
                    amountRemaining -= max_can_provide_swap;
                    state.base_price_at_tick = state.current_price;
                    state.current_tick += 1;
                }
            }
            if iterations == 2 {
                state.current_price = (state.base_price_at_tick * 110) / 100;
                state.base_price_at_tick = state.current_price;
                state.current_tick += 1;
                if !state.ticks.contains_key(state.current_tick) {
                    state.ticks.set(state.current_tick, Tick { index: state.current_tick, delta_a: 0, delta_b: 0 });
                }
                set_value(&env, "state", &state);
                return 0;
            }
        } else {
            let mut iterations = 0;
            while iterations < 100 {
                iterations += 1;
                if !state.ticks.contains_key(state.current_tick) {
                    state.ticks.set(state.current_tick, Tick { index: state.current_tick, delta_a: 0, delta_b: 0 });
                }
                let mut current_tick = state.ticks.get(state.current_tick).unwrap();
                let max_can_provide_swap = (state.current_liquidity_a * 1000000) / state.current_price;
                if max_can_provide_swap > amountRemaining {
                    let mut initialRatioThousand = state.current_liquidity_b * 1000;
                    if state.current_liquidity_a != 0 {
                        initialRatioThousand /= state.current_liquidity_a;
                    }
                    state.current_liquidity_b += amountRemaining;
                    current_tick.delta_b += amountRemaining as i128;
                    toGive = (state.current_price * amountRemaining) / 1000000;
                    amountRemaining = 0;
                    state.current_liquidity_a-=toGive;
                    current_tick.delta_a -= toGive as i128;
                    state.ticks.set(state.current_tick, current_tick);
                    let priceBaseCurrentTick = state.base_price_at_tick;
                    let priceUpper = (state.base_price_at_tick * 110) / 100 ;
                    let newRatioThousand = (state.current_liquidity_b * 1000 / state.current_liquidity_a);
                    let netChange = ((state.current_price) * (newRatioThousand * 100) / initialRatioThousand) / (100 * 100);
                    state.current_price = state.current_price - netChange;
                    break;
                } else {
                    state.current_liquidity_b += max_can_provide_swap;
                    current_tick.delta_b += max_can_provide_swap as i128;
                    state.current_price = (state.base_price_at_tick * 90) / 100;
                    toGive += state.current_liquidity_a;
                    current_tick.delta_a -= state.current_liquidity_a as i128;
                    state.current_liquidity_a = 0;
                    amountRemaining -= max_can_provide_swap;
                    state.base_price_at_tick = state.current_price;
                    state.current_tick -= 1;
                }
            }
            if iterations == 2 {
                state.current_price = (state.base_price_at_tick * 90) / 100;
                state.base_price_at_tick = state.current_price;
                state.current_tick += 1;
                if !state.ticks.contains_key(state.current_tick) {
                    state.ticks.set(state.current_tick, Tick { index: state.current_tick, delta_a: 0, delta_b: 0 });
                }
                set_value(&env, "state", &state);
                return 0;
            }
        }

        let bal = state.balances.get(user.clone()).unwrap();
        state.balances.set(user.clone(), bal + toGive);

        set_value(&env, "state", &state);

        finish_function(env);
        return toGive as i128;
    }
}