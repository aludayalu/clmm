import contract


def init(user: str, starting_price: int) -> bool:
    """
    Initialize the CLMM contract with a starting price.
    
    Args:
        user: Address of the initializing user
        starting_price: Initial price (multiplied by 1e6 for decimal precision)
        
    Returns:
        bool: True if initialization was successful, False otherwise
    """
    return contract.execute("init", [
        contract.scval.to_address(user), 
        contract.scval.to_uint128(starting_price)
    ])


def open_position(amount_a_in: int, tick_offset: int, user: str, random_position_id: int) -> bool:
    """
    Open a new liquidity position.
    
    Args:
        amount_a_in: Amount of token A to deposit
        tick_offset: Range of ticks for the position (symmetric around current tick)
        user: Address of the position owner
        random_position_id: Unique identifier for the position
        
    Returns:
        bool: True if position was successfully opened, False otherwise
    """
    return contract.execute("open_position", [
        contract.scval.to_uint128(amount_a_in),
        contract.scval.to_uint32(tick_offset),
        contract.scval.to_address(user),
        contract.scval.to_uint128(random_position_id)
    ])


def close_position(position_id: int, user: str) -> None:
    """
    Close an existing liquidity position.
    
    Args:
        position_id: ID of the position to close
        user: Address of the position owner
    """
    contract.execute("close_position", [
        contract.scval.to_uint128(position_id),
        contract.scval.to_address(user)
    ])


def get_position(position_id: int) -> dict:
    """
    Get details about a specific position.
    
    Args:
        position_id: ID of the position to query
        
    Returns:
        dict: Position details containing user, starting_tick, tick_offset, amount_a, amount_b
    """
    result = contract.execute("get_position", [contract.scval.to_uint128(position_id)])
    # Parse the Position struct to a Python dict
    return {
        "user": result["user"],
        "starting_tick": result["starting_tick"],
        "tick_offset": result["tick_offset"],
        "amount_a": result["amount_a"],
        "amount_b": result["amount_b"]
    }


def get_tick(tick_index: int) -> dict:
    """
    Get details about a specific tick.
    
    Args:
        tick_index: Index of the tick to query
        
    Returns:
        dict: Tick details containing index, delta_a, delta_b
    """
    result = contract.execute("get_tick", [contract.scval.to_int32(tick_index)])
    # Parse the Tick struct to a Python dict
    return {
        "index": result["index"],
        "delta_a": result["delta_a"],
        "delta_b": result["delta_b"]
    }


def get_current_price() -> int:
    """
    Get the current price in the pool.
    
    Returns:
        int: Current price (multiplied by 1e6)
    """
    return contract.execute("get_current_price", [])


def get_base_price_at_tick() -> int:
    """
    Get the base price at the current tick.
    
    Returns:
        int: Base price at tick (multiplied by 1e6)
    """
    return contract.execute("get_base_price_at_tick", [])


def get_current_tick() -> int:
    """
    Get the current tick index.
    
    Returns:
        int: Current tick index
    """
    return contract.execute("get_current_tick", [])


def get_current_liquidity_a() -> int:
    """
    Get the current liquidity of token A.
    
    Returns:
        int: Current liquidity of token A
    """
    return contract.execute("get_current_liquidity_a", [])


def get_current_liquidity_b() -> int:
    """
    Get the current liquidity of token B.
    
    Returns:
        int: Current liquidity of token B
    """
    return contract.execute("get_current_liquidity_b", [])


def get_balance(user: str) -> int:
    """
    Get the balance of a user.
    
    Args:
        user: Address of the user
        
    Returns:
        int: User's balance
    """
    return contract.execute("get_balance", [contract.scval.to_address(user)])


def swap(amount_in: int, is_xlm: bool, user: str) -> int:
    """
    Perform a swap operation.
    
    Args:
        amount_in: Amount of tokens to swap
        is_xlm: True if swapping XLM for the other token, False otherwise
        user: Address of the user performing the swap
        
    Returns:
        int: Amount of tokens received from the swap
    """
    return contract.execute("swap", [
        contract.scval.to_uint128(amount_in),
        contract.scval.to_bool(is_xlm),
        contract.scval.to_address(user)
    ])