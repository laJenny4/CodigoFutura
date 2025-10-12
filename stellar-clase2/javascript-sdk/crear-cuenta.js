import { Keypair, Horizon } from '@stellar/stellar-sdk';

async function crearCuenta() {
  const pair = Keypair.random();
  const publicKey = pair.publicKey();
  const secretKey = pair.secret();
  let balance = '0';
  console.log(`PUBLIC KEY: ${publicKey}`);
  console.log(`SECRET KEY: ${secretKey}`);
  const server = new Horizon.Server('https://horizon-testnet.stellar.org');
  try {
    const response = await fetch(
      `https://friendbot.stellar.org/?addr=${publicKey}`
    );
    
    const result = await response.json();
    
    if (result.successful || response.ok) {
        const account = await server.loadAccount(publicKey);
        const xlmBalance = account.balances.find(b => b.asset_type === 'native');
        balance = xlmBalance.balance;
        console.log(`BALANCE INICIAL: ${balance} XLM\n`);
    }
  } catch (error) {
    console.error('Error al fondear:', error.message);
  }

return { publicKey, secretKey, balance };

}

async function main() {
  const cuentas = [];

  for (let i = 0; i < 5; i++) {
    console.log(`Creando cuenta ${i + 1}:\n`);
    const info = await crearCuenta();
    cuentas.push(info);
  }

  console.log('--- RESUMEN DE CUENTAS ---');
  console.table(cuentas);
}

main();


