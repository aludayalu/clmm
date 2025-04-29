"""
Soroban Lightning Contract Python Wrapper

This script provides a complete Python wrapper for interacting with the 
Lightning payment channel smart contract on Soroban.
"""

import contract


def constructor():
    """
    Initialize the Lightning contract.
    """
    return contract.execute("__constructor", [])


def start_channel(user_a: str, user_b: str, amount: int, channel_id: int) -> None:
    """
    Start a new payment channel between two users.
    
    Args:
        user_a: Address of the first user
        user_b: Address of the second user
        amount: Initial amount to fund the channel
        channel_id: Unique identifier for the channel
    """
    return contract.execute("start_channel", [
        contract.scval.to_address(user_a),
        contract.scval.to_address(user_b),
        contract.scval.to_uint128(amount),
        contract.scval.to_uint128(channel_id)
    ])


def add_money(channel_id: int, amount: int) -> None:
    """
    Add more funds to an existing payment channel.
    
    Args:
        channel_id: ID of the channel to add funds to
        amount: Amount to add to the channel
    """
    return contract.execute("add_money", [
        contract.scval.to_uint128(channel_id),
        contract.scval.to_uint128(amount)
    ])


def provide_signatures(channel_id: int, payload: bytes, pubkey: bytes, input_a: bytes, input_b: bytes) -> None:
    """
    Provide signatures for a payment channel update.
    
    Args:
        channel_id: ID of the channel
        payload: The payload data
        pubkey: 32-byte public key for signature verification
        input_a: Signatures from user A
        input_b: Signatures from user B
    """
    # Ensure pubkey is exactly 32 bytes
    if len(pubkey) != 32:
        raise ValueError("Public key must be exactly 32 bytes")
    
    return contract.execute("provide_signatures", [
        contract.scval.to_uint128(channel_id),
        contract.scval.to_bytes(payload),
        contract.scval.to_bytes_n(pubkey),
        contract.scval.to_bytes(input_a),
        contract.scval.to_bytes(input_b)
    ])


def close_channel(channel_id: int, user_a: str, user_b: str) -> None:
    """
    Close a payment channel and distribute funds according to the final state.
    
    Args:
        channel_id: ID of the channel to close
        user_a: Address of the first user
        user_b: Address of the second user
    """
    return contract.execute("close_channel", [
        contract.scval.to_uint128(channel_id),
        contract.scval.to_address(user_a),
        contract.scval.to_address(user_b)
    ])


def get_channels(user: str) -> list:
    """
    Get all channel IDs associated with a user.
    
    Args:
        user: Address of the user
        
    Returns:
        list: List of channel IDs
    """
    result = contract.execute("get_channels", [contract.scval.to_address(user)])
    # Convert the result to a Python list
    return [int(channel_id) for channel_id in result]


def get_channel(channel_id: int) -> dict:
    """
    Get details about a specific channel.
    
    Args:
        channel_id: ID of the channel to query
        
    Returns:
        dict: Channel details containing userA, userB, payloadsA, signaturesA, 
              payloadsB, signaturesB, amount, finalA, finalB, awaitingClosure, unix
    """
    result = contract.execute("get_channel", [contract.scval.to_uint128(channel_id)])
    # Parse the Channel struct to a Python dict
    return {
        "userA": result["userA"],
        "userB": result["userB"],
        "payloadsA": result["payloadsA"],
        "signaturesA": result["signaturesA"],
        "payloadsB": result["payloadsB"],
        "signaturesB": result["signaturesB"],
        "amount": result["amount"],
        "finalA": result["finalA"],
        "finalB": result["finalB"],
        "awaitingClosure": result["awaitingClosure"],
        "unix": result["unix"]
    }


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