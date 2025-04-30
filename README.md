```
# CLMM (Concentrated Liquidity Market Maker)

A Stellar-based implementation of a Concentrated Liquidity Market Maker (CLMM) using Soroban smart contracts. This project provides a decentralized exchange mechanism with concentrated liquidity pools, enabling efficient token swaps and liquidity provision on the Stellar network.

## Overview

The Concentrated Liquidity Market Maker (CLMM) is a decentralized exchange protocol that allows liquidity providers to concentrate their liquidity within specific price ranges. Unlike traditional constant product AMMs (like Uniswap V2), CLMM enables more capital efficiency by allowing liquidity providers to:

- Define custom price ranges for their liquidity positions
- Provide liquidity only where they expect trading activity
- Earn higher fees by concentrating capital in active price ranges
- Reduce impermanent loss by limiting exposure to price movements

### How Price Ranges Work

In a CLMM, each liquidity position is defined by:
- A lower price bound
- An upper price bound
- The amount of liquidity provided

The price range mechanism works as follows:

1. **Price Range Selection**: Liquidity providers can choose any price range where they want to provide liquidity. For example, a provider might choose to provide liquidity between $90 and $110 for a token pair.

2. **Capital Efficiency**: By concentrating liquidity in specific ranges, providers can achieve higher capital efficiency. The same amount of capital can provide deeper liquidity in a narrow range compared to a wide range.

3. **Price Impact**: The price impact of trades is determined by the amount of liquidity available at the current price. More concentrated liquidity means lower price impact for trades within the range.

4. **Fee Generation**: Liquidity providers earn fees proportional to the amount of trading activity in their price range. Concentrated positions in active ranges can generate higher fees.

5. **Impermanent Loss Mitigation**: By limiting the price range, providers can reduce their exposure to impermanent loss. If the price moves outside their range, their position becomes inactive but protected from further loss.

### Benefits of Concentrated Liquidity

- **Higher Capital Efficiency**: Same amount of capital can provide deeper liquidity in specific ranges
- **Better Price Discovery**: More accurate pricing due to concentrated liquidity
- **Reduced Slippage**: Lower price impact for trades within active ranges
- **Flexible Risk Management**: Providers can choose ranges based on their risk tolerance
- **Dynamic Fee Generation**: Higher fees in active trading ranges

This implementation leverages Stellar's Soroban smart contracts to provide:

- Automated market making with concentrated liquidity
- Liquidity pool management with customizable price ranges
- Token swapping with minimal price impact
- Position tracking and management
- Real-time balance monitoring

## Features

### Core Functionality
- Smart contract-based concentrated liquidity pools with configurable price ranges
- Position management system for liquidity providers
- Token swapping functionality with optimized pricing
- Balance tracking and historical data
- Stellar blockchain integration with Soroban

### Advanced Features
- Dynamic fee adjustment based on pool utilization
- Position locking mechanisms
- Automated rebalancing
- Price impact protection
- Multi-token support

## Prerequisites

### System Requirements
- Python 3.x (3.8 or higher recommended)
- Node.js (v16 or higher) and npm
- Git
- 4GB RAM minimum
- 10GB free disk space

### Development Tools
- Stellar SDK
- Soroban CLI
- Local Stellar network (for development)
- Code editor (VS Code recommended)
- Postman or similar API testing tool

## Installation

### 1. Clone the Repository
```bash
git clone [repository-url]
cd clmm
```

### 2. Set Up Python Environment
```bash
# Create and activate virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install Python dependencies
pip install -r requirements.txt
pip install stellar-sdk
```

### 3. Install Node.js Dependencies
```bash
npm install
```

### 4. Configure Development Environment
```bash
# Copy example configuration
cp .env.example .env

# Edit .env with your settings
nano .env
```

## Project Structure

```
clmm/
├── contracts/           # Soroban smart contracts
│   ├── clmm/           # Main CLMM contract
│   └── token/          # Token contract implementations
├── libraries/          # Shared library code
│   ├── utils/          # Utility functions
│   └── constants/      # Constant definitions
├── scripts/            # Utility scripts
│   ├── deploy.sh       # Contract deployment
│   └── test.sh         # Test runner
├── tests/              # Test suites
├── main.py            # Main application entry point
├── contract.py        # Contract interaction utilities
└── *.sh               # Utility scripts
```

## Usage

### Starting the Local Network

#### 1. Start the Stellar Network
```bash
./rpc.sh
```

#### 2. Verify Network Status
```bash
curl http://localhost:8000/health
```

### Interacting with the Contract

#### Basic Operations

The main application (`main.py`) demonstrates various contract interactions:

##### Initialize Contract
```python
from contract import execute, Keypair, scval

# Create keypair
kp = Keypair.from_mnemonic_phrase("your-mnemonic-phrase")

# Initialize contract with parameters
execute("init", kp, contract_id, [
    scval.to_address(kp.public_key),
    scval.to_uint128(1000000)  # Initial liquidity
])
```

##### Open Position
```python
# Generate unique position ID
position_id = random.randint(0, 1024 ** 4)

# Open position with parameters
execute("open_position", kp, contract_id, [
    scval.to_uint128(5000),    # Amount
    scval.to_uint32(1),        # Duration
    scval.to_address(kp.public_key),
    scval.to_uint128(position_id)
])
```

##### Swap Tokens
```python
# Perform token swap
execute("swap", kp, contract_id, [
    scval.to_uint128(100),     # Amount
    scval.to_bool(False),      # Direction
    scval.to_address(kp.public_key)
])
```

##### Check Balance
```python
# Get current balance
balance = execute("get_balance", kp, contract_id, [
    scval.to_address(kp.public_key)
], simulate=True)
print(f"Current balance: {balance}")
```

### Advanced Usage

#### Position Management
```python
# Close position
execute("close_position", kp, contract_id, [
    scval.to_uint128(position_id)
])

# Update position
execute("update_position", kp, contract_id, [
    scval.to_uint128(position_id),
    scval.to_uint128(new_amount)
])
```

#### Pool Management
```python
# Add liquidity
execute("add_liquidity", kp, contract_id, [
    scval.to_uint128(amount),
    scval.to_address(kp.public_key)
])

# Remove liquidity
execute("remove_liquidity", kp, contract_id, [
    scval.to_uint128(amount),
    scval.to_address(kp.public_key)
])
```

## Development

### Smart Contracts

The project uses Soroban smart contracts for core functionality. Key contracts include:

- `CLMM.soroban`: Main contract implementing the CLMM logic
- `Token.soroban`: Token contract for managing token operations
- `Position.soroban`: Position management contract

### Testing

#### Unit Tests
```bash
# Run all tests
./scripts/test.sh

# Run specific test suite
python -m pytest tests/test_clmm.py
```

#### Integration Tests
```bash
# Run integration tests
./scripts/integration_test.sh
```

### Deployment

#### Local Development
```bash
# Deploy to local network
./scripts/deploy.sh local
```

#### Testnet Deployment
```bash
# Deploy to testnet
./scripts/deploy.sh testnet
```


## Acknowledgments

- Stellar Development Foundation for the blockchain infrastructure
- Soroban Team for the smart contract platform
- Contributors and maintainers who have helped shape this project
- The open-source community for their valuable tools and libraries

## Support

For support, please:
- Open an issue in the GitHub repository
- Join our Discord community
- Check the documentation
- Contact the maintainers

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.```
