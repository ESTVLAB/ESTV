#!/bin/bash

# ESTV Contract Verification Script
# Verifies the deployed contract matches source code

set -e

PROGRAM_ID="7GovpZ67R8t3NssZWkFE6pKL6HUVTXwkv9C1RTDADRY"
NETWORK="mainnet-beta"

echo "=========================================="
echo "ESTV Contract Verification"
echo "=========================================="
echo ""
echo "Program ID: $PROGRAM_ID"
echo "Network: $NETWORK"
echo ""

# Check if anchor is installed
if ! command -v anchor &> /dev/null; then
    echo "Error: Anchor CLI not found"
    echo "Install: npm install -g @coral-xyz/anchor-cli"
    exit 1
fi

# Check if solana is installed
if ! command -v solana &> /dev/null; then
    echo "Error: Solana CLI not found"
    echo "Install: sh -c \"\$(curl -sSfL https://release.solana.com/stable/install)\""
    exit 1
fi

echo "Building program..."
anchor build

echo ""
echo "Verifying deployed program..."
anchor verify $PROGRAM_ID --provider.cluster $NETWORK

echo ""
echo "=========================================="
echo "Verification Complete"
echo "=========================================="
