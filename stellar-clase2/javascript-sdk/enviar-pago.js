import { Horizon, Keypair, TransactionBuilder, Networks, Operation, Asset, BASE_FEE, Memo } from '@stellar/stellar-sdk';
import 'dotenv/config';


const server = new Horizon.Server('https://horizon-testnet.stellar.org');
const networkPassphrase = Networks.TESTNET;
const SECRET_KEY = process.env.SECRET_KEY;

const destinatarios = [
  { publicKey: 'GDHMT4MV7PUIS6Q5JLACC6XRXZCZJ2IMFPZY6MHFENU5IOXIBTRDKGRK', memo: 'Pago-001' },
  { publicKey: 'GC72MQLGNLCBI7JL2PK7UWFTKPDD35JUZNM2MXN2QJNLWVDUBNTJTS3X', memo: 'Pago-002' },
  { publicKey: 'GDMWPF3RNOZECFQJPD5CV4HSZZ7PS4J2BUG2E6GYRVIPII7ZD5BATW4W', memo: 'Pago-003' }
];

async function enviarPago(destino, amount, memo = '') {
  try {
    const sourceKeys = Keypair.fromSecret(SECRET_KEY);
    const sourceAccount = await server.loadAccount(sourceKeys.publicKey());

    if (destino === sourceKeys.publicKey()) {
  throw new Error('No puedes enviar pagos a tu propia cuenta, por favor ahorra fee');
}
    const xlmBalance = sourceAccount.balances.find(b => b.asset_type === 'native');
    const balanceDisponible = parseFloat(xlmBalance.balance);
    if (balanceDisponible < amount + 1) {
      throw new Error('Fondos insuficientes para enviar la transacción');
    }

    const transaction = new TransactionBuilder(sourceAccount, {
      fee: BASE_FEE,
      networkPassphrase: networkPassphrase
    })
      .addOperation(Operation.payment({
        destination: destino,
        asset: Asset.native(),
        amount: amount.toString()
      }))
      .addMemo(memo ? Memo.text(memo) : Memo.none())
      .setTimeout(30)
      .build();

    transaction.sign(sourceKeys);
    const result = await server.submitTransaction(transaction);

    console.log(`Pago de ${amount} XLM enviado a ${destino}`);
    console.log(`Hash: ${result.hash}\n`);

  } catch (error) {
    console.error('ERROR en enviarPago:', error.message);
    throw error;
  }
}

async function sistemaDePagos() {
  const amount = 2;  
  try {
    for (const destinatario of destinatarios) {
      await enviarPago(destinatario.publicKey, amount, destinatario.memo);
    }
        
  } catch (error) {
    console.error(`Sistema detenido: ${error.message}`);
  }
}

sistemaDePagos();
