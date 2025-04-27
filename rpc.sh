mkdir -p ~/stellar-testnet
docker rm -f stellar-dev
docker run --rm -it \
  --name stellar-dev \
  -p 8000:8000 \
  -v ~/stellar-testnet:/opt/stellar \
  stellar/quickstart:testing \
    --testnet \
    --enable rpc,horizon,friendbot
