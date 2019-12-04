#!/bin/sh

set -uexo pipefail

cd "$(dirname $0)"

cargo build
alias client='./target/debug/substrate-client-cli ${SCC_WS_URL:-ws://127.0.0.1:9944}'

# System -----------------------------------------------------------------------

client read system-account-nonce '"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"'
client read system-account-nonce '"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"'

client read system-block-hash 0 # hash of block 0

# hash of block 1, may be null as historical block hashes are not always stored
# if a recent block hash was queried, a non-null value would be returned
client read system-block-hash 1

# null, there is no entry for such and advanced block
client read system-block-hash 100000000

# TimeStamp --------------------------------------------------------------------

client read timestamp-now

# Babe -------------------------------------------------------------------------

client read babe-epoch-index
client read babe-authorities
client read babe-genesis-slot
client read babe-current-slot
client read babe-randomness

# Balances ---------------------------------------------------------------------

client read balances-total-issuance

client read balances-free-balance '"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"' # alice
client read balances-free-balance '"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"' # bob

client read balances-reserved-balance '"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"' # alice
client read balances-reserved-balance '"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"' # bob
