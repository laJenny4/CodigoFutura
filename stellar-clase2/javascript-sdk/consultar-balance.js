import pkg from '@stellar/stellar-sdk';
const {Horizon} = pkg;

const server = new Horizon.Server('https://horizon-testnet.stellar.org');

const PUBLIC_KEYS = [
  'GB2AR7AB6GKYYIWBKCHJNSTJQXRSLF7NTV5YQDE3Z64IBSHOAPADW7HR',
  'GDHMT4MV7PUIS6Q5JLACC6XRXZCZJ2IMFPZY6MHFENU5IOXIBTRDKGRK',
  'GC72MQLGNLCBI7JL2PK7UWFTKPDD35JUZNM2MXN2QJNLWVDUBNTJTS3X'
];

async function consultarBalance(publicKey) {
  try {
    const account = await server.loadAccount(publicKey);
    const xlmBalance = account.balances.find(b => b.asset_type === 'native');
    const trustlines = account.balances.filter(b => b.asset_type !== 'native').length;

    console.log(`Cuenta: ${account.id}`);
    console.log(`  Balance: ${xlmBalance.balance} XLM`);
    console.log(`  Trustlines: ${trustlines}`);
    console.log(`  Sequence: ${account.sequenceNumber()}\n`);
    
  } catch (error) {
    if (error.response && error.response.status === 404) {
      console.error('Cuenta no encontrada');
    } else {
      console.error('Error:', error.message);
    }
    throw error;
  }
}

async function monitorDeCuentas() {
  console.log('=== MONITOR DE CUENTAS ===\n');
  
  for (const publicKey of PUBLIC_KEYS) { 
    await consultarBalance(publicKey);
  }
}

monitorDeCuentas();