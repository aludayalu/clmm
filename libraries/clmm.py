"""
Soroban CLMM Contract Python Wrapper

This script provides a complete Python wrapper for interacting with the 
Constant Liquidity Market Maker (CLMM) smart contract on Soroban.
"""

import contract


def init(kp, contract_id, user: str, starting_price: int):
    """
    Initialize the CLMM contract with a starting price.
    
    Args:
        kp: Key pair for transaction signing
        contract_id: The contract ID
        user: Address of the initializing user
        starting_price: Initial price (multiplied by 1e6 for decimal precision)
    """
    return contract.execute("init", kp, contract_id, args=[
        contract.scval.to_address(user), 
        contract.scval.to_uint128(starting_price)
    ])


def open_position(kp, contract_id, amount_a_in: int, tick_offset: int, user: str, random_position_id: int):
    """
    Open a new liquidity position.
    
    Args:
        kp: Key pair for transaction signing
        contract_id: The contract ID
        amount_a_in: Amount of token A to deposit
        tick_offset: Range of ticks for the position (symmetric around current tick)
        user: Address of the position owner
        random_position_id: Unique identifier for the position
    """
    return contract.execute("open_position", kp, contract_id, args=[
        contract.scval.to_uint128(amount_a_in),
        contract.scval.to_uint32(tick_offset),
        contract.scval.to_address(user),
        contract.scval.to_uint128(random_position_id)
    ])


def close_position(kp, contract_id, position_id: int, user: str):
    """
    Close an existing liquidity position.
    
    Args:
        kp: Key pair for transaction signing
        contract_id: The contract ID
        position_id: ID of the position to close
        user: Address of the position owner
    """
    return contract.execute("close_position", kp, contract_id, args=[
        contract.scval.to_uint128(position_id),
        contract.scval.to_address(user)
    ])


def get_position(contract_id, position_id: int):
    """
    Get details about a specific position.
    
    Args:
        contract_id: The contract ID
        position_id: ID of the position to query
    """
    return contract.execute("get_position", None, contract_id, args=[
        contract.scval.to_uint128(position_id)
    ], simulate=True)


def get_tick(contract_id, tick_index: int):
    """
    Get details about a specific tick.
    
    Args:
        contract_id: The contract ID
        tick_index: Index of the tick to query
    """
    return contract.execute("get_tick", None, contract_id, args=[
        contract.scval.to_int32(tick_index)
    ], simulate=True)


def get_current_price(contract_id):
    """
    Get the current price in the pool.
    
    Args:
        contract_id: The contract ID
    """
    return contract.execute("get_current_price", None, contract_id, args=[], simulate=True)


def get_base_price_at_tick(contract_id):
    """
    Get the base price at the current tick.
    
    Args:
        contract_id: The contract ID
    """
    return contract.execute("get_base_price_at_tick", None, contract_id, args=[], simulate=True)


def get_current_tick(contract_id):
    """
    Get the current tick index.
    
    Args:
        contract_id: The contract ID
    """
    return contract.execute("get_current_tick", None, contract_id, args=[], simulate=True)


def get_current_liquidity_a(contract_id):
    """
    Get the current liquidity of token A.
    
    Args:
        contract_id: The contract ID
    """
    return contract.execute("get_current_liquidity_a", None, contract_id, args=[], simulate=True)


def get_current_liquidity_b(contract_id):
    """
    Get the current liquidity of token B.
    
    Args:
        contract_id: The contract ID
    """
    return contract.execute("get_current_liquidity_b", None, contract_id, args=[], simulate=True)


def get_balance(contract_id, user: str):
    """
    Get the balance of a user.
    
    Args:
        contract_id: The contract ID
        user: Address of the user
    """
    return contract.execute("get_balance", None, contract_id, args=[
        contract.scval.to_address(user)
    ], simulate=True)


def swap(kp, contract_id, amount_in: int, is_xlm: bool, user: str):
    """
    Perform a swap operation.
    
    Args:
        kp: Key pair for transaction signing
        contract_id: The contract ID
        amount_in: Amount of tokens to swap
        is_xlm: True if swapping XLM for the other token, False otherwise
        user: Address of the user performing the swap
    """
    return contract.execute("swap", kp, contract_id, args=[
        contract.scval.to_uint128(amount_in),
        contract.scval.to_bool(is_xlm),
        contract.scval.to_address(user)
    ])