#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, Env, IntoVal, Map, Symbol, TryFromVal, Val, Vec, BytesN};

#[contract]
pub struct Lightning;

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
pub struct Channel {
    userA: Address,
    userB: Address,
    payloadsA: Bytes,
    signaturesA: Bytes,
    payloadsB: Bytes,
    signaturesB: Bytes,
    amount: u128,
    finalA: u128,
    finalB: u128,
    awaitingClosure: bool,
    unix: u64
}

#[contracttype]
pub struct LightningState {
    UserToChannels: Map<Address, Vec<u128>>,
    Channels: Map<u128, Channel>
}

#[contractimpl]
impl Lightning {
    pub fn __constructor(env: Env) {
        set_value(&env, "state", &LightningState{
            UserToChannels: Map::new(&env),
            Channels: Map::new(&env)
        });
    }

    pub fn start_channel(env: Env, user_a: Address, user_b: Address, amount: u128, channel_id: u128) {
        let userA = user_a.clone();
        let userB = user_b.clone();
        let channelId = channel_id;
        let mut state: LightningState = get_value(&env, "state");
        if !state.UserToChannels.contains_key(userA.clone()) {
            state.UserToChannels.set(userA.clone(), Vec::new(&env));
        }
        if !state.UserToChannels.contains_key(userB.clone()) {
            state.UserToChannels.set(userB.clone(), Vec::new(&env));
        }
        if state.Channels.contains_key(channelId) {
            return;
        }

        let mut userAChannels = state.UserToChannels.get(user_a.clone()).expect("hi");
        userAChannels.push_back(channel_id);
        state.UserToChannels.set(user_a.clone(), userAChannels);

        let mut userBChannels = state.UserToChannels.get(user_b.clone()).expect("hi");
        userBChannels.push_back(channel_id);
        state.UserToChannels.set(user_b.clone(), userBChannels);

        state.Channels.set(channelId, Channel {
            userA,
            userB,
            payloadsA: Bytes::new(&env),
            signaturesA: Bytes::new(&env),
            payloadsB: Bytes::new(&env),
            signaturesB: Bytes::new(&env),
            amount,
            finalA: amount,
            finalB: 0,
            awaitingClosure: false,
            unix: 0
        });
        set_value(&env, "state", &state);
        finish_function(env);
    }

    pub fn add_money(env: Env, channelId: u128, amount: u128) {
        let mut state: LightningState = get_value(&env, "state");
        let mut channel: Channel = state.Channels.get(channelId).unwrap();
        channel.amount += amount;
        state.Channels.set(channelId, channel);
        set_value(&env, "state", &state);
        finish_function(env);
    }

    pub fn provide_signatures(
        env: Env,
        channel_id: u128,
        payload: Bytes,
        pubkey: BytesN<32>,
        input_a: Bytes,
        input_b: Bytes,
    ) {
        const SIG_LEN: u32 = 64;
        let mut state: LightningState = get_value(&env, "state");
        let mut channel: Channel = state.Channels.get(channel_id).expect("Channel not found");
    
        let mut stored_a_payloads = channel.payloadsA.clone();
        let mut stored_a_sigs = channel.signaturesA.clone();
        if input_a.len() % SIG_LEN != 0 { panic!("Invalid total A-signatures length"); }

        let mut found = false;

        let mut offset = 0u32;
        while offset < input_a.len() {
            let chunk = input_a.slice(offset..offset + SIG_LEN);
            let mut seen = false;
            let mut j = 0u32;
            while j < stored_a_sigs.len() {
                if stored_a_sigs.slice(j..j + SIG_LEN) == chunk {
                    seen = true;
                    break;
                }
                j += SIG_LEN;
            }
            if !seen {
                let sig64: BytesN<64> = bytes_to_bytesn(&env, chunk.clone());
                let amount = verify_and_extract(env.clone(), payload.clone(), sig64, pubkey.clone());
                channel.finalA -= amount;
                channel.finalB += amount;
                stored_a_sigs.append(&chunk);
                stored_a_payloads.append(&payload.clone());
                found = true;
            }
            offset += SIG_LEN;
        }
        channel.signaturesA = stored_a_sigs;
        channel.payloadsA = stored_a_payloads;
    
        let mut stored_b_payloads = channel.payloadsB.clone();
        let mut stored_b_sigs = channel.signaturesB.clone();
        if input_b.len() % SIG_LEN != 0 { panic!("Invalid total B-signatures length"); }
        let mut offset_b = 0u32;
        while offset_b < input_b.len() {
            let chunk = input_b.slice(offset_b..offset_b + SIG_LEN);
            let mut seen = false;
            let mut j = 0u32;
            while j < stored_b_sigs.len() {
                if stored_b_sigs.slice(j..j + SIG_LEN) == chunk {
                    seen = true;
                    break;
                }
                j += SIG_LEN;
            }
            if !seen {
                let sig64: BytesN<64> = bytes_to_bytesn(&env, chunk.clone());
                let amount = verify_and_extract(env.clone(), payload.clone(), sig64, pubkey.clone());
                channel.finalB -= amount;
                channel.finalA += amount;
                stored_b_sigs.append(&chunk);
                stored_b_payloads.append(&payload.clone());
                found = true;
            }
            offset_b += SIG_LEN;
        }

        channel.awaitingClosure = true;

        if found {
            channel.unix = env.ledger().timestamp() + 86400;
        }

        channel.signaturesB = stored_b_sigs;
        channel.payloadsB = stored_b_payloads;

        state.Channels.set(channel_id, channel);
        set_value(&env, "state", &state);
        finish_function(env);
    }

    pub fn close_channel(env: Env, channel_id: u128, user_a: Address, user_B: Address) {
        let mut state: LightningState = get_value(&env, "state");
        let channel: Channel = state.Channels.get(channel_id).expect("Channel not found");
        if env.ledger().timestamp() > channel.unix {
            state.Channels.remove(channel_id);
        }
        set_value(&env, "state", &state);
        finish_function(env);
    }

    pub fn get_channels(env: Env, user: Address) -> Vec<u128> {
        let state: LightningState = get_value(&env, "state");
        state.UserToChannels.get(user).expect("hi")
    }

    pub fn get_channel(env: Env, channel_id: u128) -> Channel {
        let state: LightningState = get_value(&env, "state");
        state.Channels.get(channel_id).expect("hi")
    }
}

fn bytes_to_bytesn<const N: usize>(env: &Env, data: Bytes) -> BytesN<N> {
    if data.len() as usize != N {
        panic!("Expected {} bytes, got {}", N, data.len());
    }
    let mut buf = [0u8; N];
    data.copy_into_slice(&mut buf);
    BytesN::from_array(env, &buf)
}

pub fn verify_and_extract(
    env: Env,
    payload: Bytes,
    sig: BytesN<64>,
    pubkey: BytesN<32>,
) -> u128 {
    env.crypto().ed25519_verify(&pubkey, &payload, &sig);

    if payload.len() != 32 {
        panic!("Invalid payload length");
    }

    let mut prefix_bytes = [0u8; 16];
    payload.slice(0u32..16u32).copy_into_slice(&mut prefix_bytes);
    let prefix = u128::from_be_bytes(prefix_bytes);
    if prefix != 69696969 {
        panic!("Invalid payload prefix");
    }

    let mut amount_bytes = [0u8; 16];
    payload.slice(16u32..32u32).copy_into_slice(&mut amount_bytes);
    u128::from_be_bytes(amount_bytes)
}