#!/bin/bash

# Replace with the deployed canister ID of your backend
CANISTER_ID=$(dfx br5f7-7uaaa-aaaaa-qaaca-cai)

# Test receive_tokens function
dfx canister call backend receive_tokens '(principal "aaaaa-aa", 100)'

# Test get_balance function
dfx canister call backend get_balance '(principal "aaaaa-aa")'

# Test send_tokens function
dfx canister call backend send_tokens '(principal "aaaaa-aa", principal "bbbbb-bb", 50)'

# Verify balances after transfer
dfx canister call backend get_balance '(principal "aaaaa-aa")'
dfx canister call backend get_balance '(principal "bbbbb-bb")'
