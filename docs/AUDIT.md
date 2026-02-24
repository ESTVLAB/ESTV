# Security Audits

## Audit Status

| Audit Type | Auditor | Status | Report |
|------------|---------|--------|--------|
| Smart Contract | CertiK / Hacken | Scheduled Q1 2026 | Pending |
| Penetration Test | TBD | Planned | - |
| Code Review | Internal | Completed | Internal |

---

## Planned Audits

### Smart Contract Audit

**Scope**:
- Token contract on Solana
- Merkle root commitment system
- Reward claiming mechanism
- Access control and permissions
- Emergency functions

**Auditors Under Consideration**:
- CertiK
- Hacken
- SlowMist
- OtterSec (Solana specialist)

**Timeline**: Q1 2026

### Penetration Testing

**Scope**:
- Platform infrastructure
- API security
- Wallet integration
- User authentication

---

## Security Measures

### Smart Contract Security

| Feature | Implementation |
|---------|----------------|
| Language | Rust (memory-safe) |
| Framework | Anchor (Solana standard) |
| Admin Control | Multi-signature |
| Upgradability | Timelock protected |
| Emergency | Pause functionality |

### Access Control

```
┌─────────────────────────────────────────┐
│           Multi-Sig Admin               │
│         (3 of 5 required)               │
└─────────────────┬───────────────────────┘
                  │
    ┌─────────────┼─────────────┐
    │             │             │
    ▼             ▼             ▼
┌───────┐   ┌─────────┐   ┌─────────┐
│ Pause │   │ Upgrade │   │ Config  │
│ 1/3   │   │ 3/5+TL  │   │ 2/5     │
└───────┘   └─────────┘   └─────────┘

TL = Timelock (48-72 hours)
```

### Security Principles

1. **Role Separation**
   - Merkle commit authority separate from admin
   - Limited scope per role

2. **Duplicate Claim Prevention**
   - On-chain claim state tracking
   - Per-epoch claim limits

3. **Timelock Protection**
   - Critical changes require waiting period
   - Community notification before execution

4. **Emergency Pause**
   - Circuit breaker for anomalies
   - Rapid response capability

---

## Bug Bounty Program

### Status
**Coming Q2 2026**

### Planned Structure

| Severity | Reward Range |
|----------|--------------|
| Critical | $10,000 - $50,000 |
| High | $5,000 - $10,000 |
| Medium | $1,000 - $5,000 |
| Low | $100 - $1,000 |

### Scope (Planned)
- Smart contracts
- Platform APIs
- Wallet integrations
- Critical infrastructure

---

## Audit Report Publication

Upon completion, audit reports will be:

1. Published in `/audits` directory
2. Linked in README
3. Announced on official channels
4. Submitted to exchanges

---

## Verification

### Contract Verification

**Mainnet Contract**: `7GovpZ67R8t3NssZWkFE6pKL6HUVTXwkv9C1RTDADRY`

Verify on:
- [Solscan](https://solscan.io/token/7GovpZ67R8t3NssZWkFE6pKL6HUVTXwkv9C1RTDADRY)
- [Solana Explorer](https://explorer.solana.com/address/7GovpZ67R8t3NssZWkFE6pKL6HUVTXwkv9C1RTDADRY)

### Source Code Verification

Source code in this repository matches deployed contract.

```bash
# Verify locally
anchor build
anchor verify 7GovpZ67R8t3NssZWkFE6pKL6HUVTXwkv9C1RTDADRY
```

---

## Contact

**Security Issues**: security@estvlabs.com

**Responsible Disclosure**:
1. Email security team
2. Provide detailed description
3. Allow 48 hours for acknowledgment
4. Coordinate disclosure timeline

---

*Audit information updated as audits are completed.*
