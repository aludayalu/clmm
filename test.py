from stellar_sdk.soroban_server import SorobanServer

# Soroban JSON-RPC endpoint
soroban = SorobanServer("http://localhost:8000/soroban/rpc")

# health check
print(soroban.get_health())
