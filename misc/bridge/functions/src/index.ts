import * as functions from 'firebase-functions';
import * as admin from 'firebase-admin';
import * as blake2 from 'blake2';

admin.initializeApp();

const byteToHexString = (uint8arr: Uint8Array) => {
    var hexStr = '';
    for (var i = 0; i < uint8arr.length; i++) {
        var hex = (uint8arr[i] & 0xff).toString(16);
        hex = (hex.length === 1) ? '0' + hex : hex;
        hexStr += hex;
    }

    return hexStr.toUpperCase();
}

export const registerMemo = functions.https.onRequest(async (request, response) => {
    const nodlePublicKey = request.body.nodlePublicKey;

    // In order to generate a deterministic ID we hash the user's address.
    var hasher = blake2.createHash('blake2b', { digestLength: 16 });
    hasher.update(Buffer.from(nodlePublicKey));
    const memoHash = byteToHexString(hasher.digest());

    // We save the mapping to firebase
    await admin.firestore().collection('bridge-accounts').doc(memoHash).set({
        address: nodlePublicKey,
    }, { merge: true });

    response.send({ memoHash: memoHash });
});