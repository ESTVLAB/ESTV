# Security Audits

## Audit Status

| Audit Type | Auditor | Status | Report |
|------------|---------|--------|--------|
| Smart Contract | CertiK (Primary) | RFQ in Progress | Pending |
| KYC Verification | CertiK KYC | Planned | Pending |
| Code Review | Internal | ✅ Completed | Internal |
| Penetration Test | TBD | Planned | - |

---

## Audit Strategy

### Phase 1: CertiK Smart Contract Audit (Primary)

ESTV has selected **CertiK** as the primary audit partner for its industry-leading reputation and comprehensive Solana audit capabilities.

**Scope**:
- Token contract on Solana
- Merkle root commitment system
- Reward claiming mechanism
- Access control and permissions
- Emergency functions

**Estimated Investment**:

| Component | Range |
|-----------|-------|
| Smart Contract Audit | $15,000 - $35,000 |
| KYC Team Verification | $3,000 - $5,000 |
| **Total** | **$18,000 - $40,000** |

> Estimate based on ESTV's medium-high technical complexity (Rust/Anchor, Merkle tree, multi-sig admin).

**Expected Timeline**:

```
RFQ Submission  →  Onboarding  →  Audit Period  →  Report
    Week 1          Week 2-3       Week 4-7        Week 8
                                                     │
                                              Final Report
                                              & Skynet Badge
```

### Phase 2: CertiK KYC Verification

Team identity verification through CertiK's KYC program to enhance project credibility on CoinMarketCap, CoinGecko, and exchange listings.

### Phase 3: Ongoing Monitoring

- CertiK Skynet security score tracking
- Continuous vulnerability monitoring
- Real-time alert system

---

## Alternative Auditors Evaluated

| Auditor | Specialization | Consideration |
|---------|---------------|---------------|
| **CertiK** | Full-stack blockchain security | ✅ Primary — industry standard, Skynet integration |
| **OtterSec** | Solana-native specialist | Shortlisted — deep Solana/Anchor expertise |
| **Sec3 (Anza)** | Solana automated + manual | Evaluated — strong automated tooling |
| **Halborn** | Multi-chain audits | Evaluated — flexible engagement models |
| **Neodyme** | Solana security research | Evaluated — strong Solana track record |

> CertiK was selected for its exchange recognition, CoinMarketCap/CoinGecko integration, and comprehensive Skynet monitoring platform.

---

## Penetration Testing

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
