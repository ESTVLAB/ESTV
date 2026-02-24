/**
 * ESTV Token Information Script
 * Fetches and displays token data from Solana
 */

const { Connection, PublicKey } = require('@solana/web3.js');

const ESTV_MINT = '7GovpZ67R8t3NssZWkFE6pKL6HUVTXwkv9C1RTDADRY';
const RPC_URL = 'https://api.mainnet-beta.solana.com';

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
}

getTokenInfo();
