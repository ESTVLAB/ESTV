# ESTV Token Integration Guide

## Overview

This guide covers how to integrate ESTV token into wallets, exchanges, and applications.

---

## Token Specifications

```json
{
  "name": "ESTV",
  "symbol": "ESTV",
  "blockchain": "Solana",
  "standard": "SPL Token",
  "decimals": 9,
  "contract": "7GovpZ67R8t3NssZWkFE6pKL6HUVTXwkv9C1RTDADRY",
  "totalSupply": "1000000000",
  "website": "https://estv-token.io",
  "explorer": "https://solscan.io/token/7GovpZ67R8t3NssZWkFE6pKL6HUVTXwkv9C1RTDADRY"
}
```

---

## For Exchanges

### Listing Requirements

| Requirement | Details |
|-------------|---------|
| Network | Solana Mainnet |
| Token Standard | SPL |
| Contract | `7GovpZ67R8t3NssZWkFE6pKL6HUVTXwkv9C1RTDADRY` |
| Decimals | 9 |
| Minimum Confirmations | 32 (recommended) |

### Deposit/Withdrawal

```typescript
// Using @solana/web3.js
import { Connection, PublicKey } from '@solana/web3.js';
import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from '@solana/spl-token';

const ESTV_MINT = new PublicKey('7GovpZ67R8t3NssZWkFE6pKL6HUVTXwkv9C1RTDADRY');

// Get user's ESTV token account
async function getESTVAccount(userWallet: PublicKey): Promise<PublicKey> {
  return await getAssociatedTokenAddress(ESTV_MINT, userWallet);
}
```

### Balance Query

```typescript
import { getAccount } from '@solana/spl-token';

async function getESTVBalance(connection: Connection, tokenAccount: PublicKey): Promise<bigint> {
  const account = await getAccount(connection, tokenAccount);
  return account.amount;
}
```

---

## For Wallets

### Token Metadata

```json
{
  "name": "ESTV",
  "symbol": "ESTV",
  "logo": "https://raw.githubusercontent.com/ESTVLAB/ESTV/main/assets/token-icon.png",
  "decimals": 9,
  "address": "7GovpZ67R8t3NssZWkFE6pKL6HUVTXwkv9C1RTDADRY",
  "chainId": 101,
  "tags": ["utility-token", "esports", "media"]
}
```

### Logo Assets

| Size | URL |
|------|-----|
| Icon (64x64) | `https://raw.githubusercontent.com/ESTVLAB/ESTV/main/assets/logo-icon.png` |
| Standard (256x256) | `https://raw.githubusercontent.com/ESTVLAB/ESTV/main/assets/token-icon.png` |
| Banner | `https://raw.githubusercontent.com/ESTVLAB/ESTV/main/assets/logo-banner.png` |

---

## For DApps

### Web3 Integration

```typescript
import { Connection, clusterApiUrl } from '@solana/web3.js';

// Connect to Solana mainnet
const connection = new Connection(clusterApiUrl('mainnet-beta'));

// ESTV Token Mint Address
const ESTV_MINT = '7GovpZ67R8t3NssZWkFE6pKL6HUVTXwkv9C1RTDADRY';
```

### Transfer Example

```typescript
import {
  createTransferInstruction,
  getAssociatedTokenAddress
} from '@solana/spl-token';
import { Transaction, sendAndConfirmTransaction } from '@solana/web3.js';

async function transferESTV(
  connection: Connection,
  payer: Keypair,
  recipient: PublicKey,
  amount: number
): Promise<string> {
  const payerTokenAccount = await getAssociatedTokenAddress(ESTV_MINT, payer.publicKey);
  const recipientTokenAccount = await getAssociatedTokenAddress(ESTV_MINT, recipient);

  const instruction = createTransferInstruction(
    payerTokenAccount,
    recipientTokenAccount,
    payer.publicKey,
    amount * 10 ** 9  // Adjust for decimals
  );

  const transaction = new Transaction().add(instruction);
  return await sendAndConfirmTransaction(connection, transaction, [payer]);
}
```

---

## API Endpoints

### RPC Providers

| Provider | Endpoint |
|----------|----------|
| Solana (Free) | `https://api.mainnet-beta.solana.com` |
| Helius | `https://mainnet.helius-rpc.com` |
| QuickNode | Custom endpoint |
| Alchemy | Custom endpoint |

### Price Data

Available on:
- **BitMart** — ESTV/USDT (Live)
- CoinGecko API (registration pending)
- CoinMarketCap API (registration pending)
- DEX aggregators (Raydium, Jupiter)

---

## SDK (Coming Soon)

### ESTV SDK Features (Planned)

```typescript
// Future SDK usage example
import { ESTVClient } from '@estv/sdk';

const client = new ESTVClient({
  network: 'mainnet',
  apiKey: 'your-api-key'
});

// Get engagement rewards
const rewards = await client.getRewards(walletAddress);

// Claim rewards
const tx = await client.claimRewards(walletAddress, proof);
```

---

## Testing

### Devnet

For testing, use Solana Devnet:

```typescript
const connection = new Connection(clusterApiUrl('devnet'));
```

### Faucet

Request devnet SOL: https://faucet.solana.com

---

## Support

### Technical Support

- GitHub Issues: https://github.com/ESTVLAB/ESTV/issues
- Developer Email: dev@estvlabs.com

### Exchange Listing Support

- Email: listing@estvlabs.com
- Include: Exchange name, requirements, timeline

---

## Resources

| Resource | Link |
|----------|------|
| GitHub | https://github.com/ESTVLAB/ESTV |
| Documentation | https://github.com/ESTVLAB/ESTV/docs |
| Solscan | https://solscan.io/token/7GovpZ67R8t3NssZWkFE6pKL6HUVTXwkv9C1RTDADRY |
| Website | https://estv-token.io |
