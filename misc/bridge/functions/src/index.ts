import * as functions from 'firebase-functions';
import * as admin from 'firebase-admin';
import * as blake2 from 'blake2';
import StellarSdk from 'stellar-sdk';

admin.initializeApp();
const stellarServer = new StellarSdk.Server(functions.config().stellar.horizonurl);

const firestoreAccountsCollection = 'bridge-accounts';
const firestoreTransactionsCollection = 'bridge-transactions';

const byteToHexString = (uint8arr: Uint8Array) => {
    let hexStr = '';
    for (const elem of uint8arr) {
        let hex = (elem & 0xff).toString(16);
        hex = (hex.length === 1) ? '0' + hex : hex;
        hexStr += hex;
    }

    return hexStr.toUpperCase();
}

export const registerMemo = functions.https.onRequest(async (request, response) => {
    const nodlePublicKey = request.body.nodlePublicKey;

    // In order to generate a deterministic ID we hash the user's address.
    const hasher = blake2.createHash('blake2b', { digestLength: 32 });
    hasher.update(Buffer.from(nodlePublicKey));
    const memoHash = byteToHexString(hasher.digest());

    // We save the mapping to firebase
    await admin.firestore().collection(firestoreAccountsCollection).doc(memoHash).set({
        address: nodlePublicKey,
    }, { merge: true });

    response.send({ memoHash: memoHash, destination: functions.config().nodle.coinsdest });
});

export const proveTransaction = functions.https.onRequest(async (request, response) => {
    const tx = await stellarServer.transactions()
        .transaction(request.body.txHash)
        .call();

    const duplicate = await admin.firestore()
        .collection(firestoreTransactionsCollection)
        .doc(request.body.txHash)
        .get();
    if (duplicate.exists) {
        throw new functions.https.HttpsError('invalid-argument', 'transaction already submitted');
    }

    if (!tx.successful) {
        throw new functions.https.HttpsError('invalid-argument', 'transaction not successful');
    }

    const operations = await tx.operations();

    // Verify destination, save amount
    let goodPaymentFound = false;
    let paymentAmount = 0;
    for (const op of operations.records) {
        if (op.type === 'payment' &&
            op.asset_code === functions.config().stellar.code &&
            op.asset_issuer === functions.config().stellar.issuer &&
            op.to === functions.config().stellar.dest) {
            goodPaymentFound = true;
            paymentAmount = op.amount;
        }
    }

    if (!goodPaymentFound) {
        throw new functions.https.HttpsError('invalid-argument', 'did not find a good payment');
    }

    const decodedMemo = byteToHexString(Buffer.from(tx.memo, 'base64'));
    const nodlePublicKey = await admin.firestore()
        .collection(firestoreAccountsCollection)
        .doc(decodedMemo)
        .get();

    if (!nodlePublicKey.exists) {
        throw new functions.https.HttpsError('not-found', 'memo not associated to a public key');
    }

    const pkDataShouldNotBeUndefined = nodlePublicKey.data() || {};

    // Register the transaction to be sent later
    await admin.firestore().collection(firestoreTransactionsCollection).doc(request.body.txHash).set({
        address: pkDataShouldNotBeUndefined.address,
        amount: paymentAmount,
        paid: false,
    });

    response.send({ validTxHash: request.body.txHash })
});