"""
Soroban Lightning Contract Python Wrapper

This script provides a complete Python wrapper for interacting with the 
Lightning payment channel smart contract on Soroban.
"""

import contract


def constructor(kp, contract_id):
    """
    Initialize the Lightning contract.
    
    Args:
        kp: Key pair for transaction signing
        contract_id: The contract ID
    """
    return contract.execute("__constructor", kp, contract_id, args=[])


def start_channel(kp, contract_id, user_a: str, user_b: str, amount: int, channel_id: int):
    """
    Start a new payment channel between two users.
    
    Args:
        kp: Key pair for transaction signing
        contract_id: The contract ID
        user_a: Address of the first user
        user_b: Address of the second user
        amount: Initial amount to fund the channel
        channel_id: Unique identifier for the channel
    """
    return contract.execute("start_channel", kp, contract_id, args=[
        contract.scval.to_address(user_a),
        contract.scval.to_address(user_b),
        contract.scval.to_uint128(amount),
        contract.scval.to_uint128(channel_id)
    ])


def add_money(kp, contract_id, channel_id: int, amount: int):
    """
    Add more funds to an existing payment channel.
    
    Args:
        kp: Key pair for transaction signing
        contract_id: The contract ID
        channel_id: ID of the channel to add funds to
        amount: Amount to add to the channel
    """
    return contract.execute("add_money", kp, contract_id, args=[
        contract.scval.to_uint128(channel_id),
        contract.scval.to_uint128(amount)
    ])


def provide_signatures(kp, contract_id, channel_id: int, payload: bytes, pubkey: bytes, input_a: bytes, input_b: bytes):
    """
    Provide signatures for a payment channel update.
    
    Args:
        kp: Key pair for transaction signing
        contract_id: The contract ID
        channel_id: ID of the channel
        payload: The payload data
        pubkey: 32-byte public key for signature verification
        input_a: Signatures from user A
        input_b: Signatures from user B
    """
    return contract.execute("provide_signatures", kp, contract_id, args=[
        contract.scval.to_uint128(channel_id),
        contract.scval.to_bytes(payload),
        contract.scval.to_bytes_n(pubkey),
        contract.scval.to_bytes(input_a),
        contract.scval.to_bytes(input_b)
    ])


def close_channel(kp, contract_id, channel_id: int, user_a: str, user_b: str):
    """
    Close a payment channel and distribute funds according to the final state.
    
    Args:
        kp: Key pair for transaction signing
        contract_id: The contract ID
        channel_id: ID of the channel to close
        user_a: Address of the first user
        user_b: Address of the second user
    """
    return contract.execute("close_channel", kp, contract_id, args=[
        contract.scval.to_uint128(channel_id),
        contract.scval.to_address(user_a),
        contract.scval.to_address(user_b)
    ])


def get_channels(contract_id, user: str):
    """
    Get all channel IDs associated with a user.
    
    Args:
        contract_id: The contract ID
        user: Address of the user
    """
    return contract.execute("get_channels", None, contract_id, args=[
        contract.scval.to_address(user)
    ], simulate=True)


def get_channel(contract_id, channel_id: int):
    """
    Get details about a specific channel.
    
    Args:
        contract_id: The contract ID
        channel_id: ID of the channel to query
    """
    return contract.execute("get_channel", None, contract_id, args=[
        contract.scval.to_uint128(channel_id)
    ], simulate=True)


# Helper utility functions that might be useful when working with this contract

def create_payment_payload(amount: int) -> bytes:
    """
    Create a properly formatted payment payload with the required prefix.
    
    Args:
        amount: Amount to include in the payload
        
    Returns:
        bytes: Formatted payload ready for signing
    """
    prefix = 69696969  # The required prefix from the contract
    prefix_bytes = prefix.to_bytes(16, byteorder='big')
    amount_bytes = amount.to_bytes(16, byteorder='big')
    return prefix_bytes + amount_bytes


def parse_signatures(signatures_bytes: bytes) -> list:
    """
    Parse a byte string of concatenated signatures into a list of individual signatures.
    
    Args:
        signatures_bytes: Concatenated signatures, each 64 bytes
        
    Returns:
        list: List of individual 64-byte signatures
    """
    if len(signatures_bytes) % 64 != 0:
        raise ValueError("Signatures length must be a multiple of 64 bytes")
    
    signatures = []
    for i in range(0, len(signatures_bytes), 64):
        signatures.append(signatures_bytes[i:i+64])
    
    return signatures


def create_signature_input(signatures: list) -> bytes:
    """
    Concatenate multiple signatures into a single byte string for contract input.
    
    Args:
        signatures: List of 64-byte signatures
        
    Returns:
        bytes: Concatenated signatures as a single byte string
    """
    result = b''
    for sig in signatures:
        if len(sig) != 64:
            raise ValueError("Each signature must be exactly 64 bytes")
        result += sig
    
    return result