/**
 * ESTV Token Information Script
 * Fetches and displays token data from Solana
 */

const { Connection, PublicKey } = require('@solana/web3.js');

const ESTV_MINT = '7GovpZ67R8t3NssZWkFE6pKL6HUVTXwkv9C1RTDADRY';
const RPC_URL = 'https://api.mainnet-beta.solana.com';

// Foundation wallet addresses (Squads Multisig)
const FOUNDATION_WALLETS = {
    'Ecosystem Rewards':    '5KCK5gpu1rbcZALP8pDEZnFGRgB5GAp8GgZRCbSU6GgV',
    'Long-term Ecosystem':  'DyujGTjzC1b6DzogQf7vjCgLsTuK3MPLonrU4yBFZvKx',
    'Liquidity & Ops':      '7g8DLJsPXoC2mHhioEczgvkc4x72ucEa5smqHoVaXukR',
    'Community':            '2c2zfjZ1f8e3RuGisKEYEv1Yrwmstgy3q4HmA4o1EJPN',
    'Development':          'E8xPzrwGTbj5R2LMiwPeqXPD7mTG6rM9Svjv7jgUdAZy',
    'Partnership & Growth': 'DgtpvncHA9MT4QhoccDSVacLgEt2Jj4PMovNcxdFhbDN',
    'Early Participants':   '6c8QZ138DUXarhmXym6hDmW84V5oQvzpHDvDxpAYxEi3',
    'Team / Founders':      'XNmi836Bz1ctTDv9Jh2puYYdVvVBUCNW6ChAUWjfSGj',
};

async function getTokenInfo() {
    console.log('==========================================');
    console.log('ESTV Token Information');
    console.log('==========================================\n');

    const connection = new Connection(RPC_URL, 'confirmed');
    const mintPubkey = new PublicKey(ESTV_MINT);

    try {
        // Get mint account info
        const mintInfo = await connection.getParsedAccountInfo(mintPubkey);

        if (mintInfo.value) {
            const data = mintInfo.value.data.parsed.info;

            console.log('Token Details:');
            console.log('--------------');
            console.log(`Mint Address: ${ESTV_MINT}`);
            console.log(`Decimals: ${data.decimals}`);
            console.log(`Supply: ${data.supply / Math.pow(10, data.decimals)}`);
            console.log(`Mint Authority: ${data.mintAuthority || 'Disabled'}`);
            console.log(`Freeze Authority: ${data.freezeAuthority || 'Disabled'}`);
            console.log('');
            console.log('Explorer Links:');
            console.log(`- Solscan: https://solscan.io/token/${ESTV_MINT}`);
            console.log(`- Explorer: https://explorer.solana.com/address/${ESTV_MINT}`);
        } else {
            console.log('Error: Could not fetch token info');
        }
    } catch (error) {
        console.error('Error:', error.message);
    }

    console.log('\n==========================================');
    console.log('\nFoundation Wallets (Squads Multisig):');
    console.log('-------------------------------------');
    for (const [name, address] of Object.entries(FOUNDATION_WALLETS)) {
        console.log(`${name.padEnd(22)} ${address}`);
    }
    console.log('\n==========================================');
}

getTokenInfo();
