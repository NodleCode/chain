import * as functions from 'firebase-functions';
import * as admin from 'firebase-admin';
import Blake2s from 'blake2s-js';

admin.initializeApp();

export const registerMemo = functions.https.onRequest(async (request, response) => {
    const nodlePublicKey = request.body.ss58Address;

    // In order to generate a deterministic ID we hash the user's address.
    const hasher = new Blake2s(32);
    hasher.update(new TextEncoder().encode(nodlePublicKey));
    const memoHash = hasher.hexDigest();

    // We save the mapping to firebase
    await admin.firestore().collection('bridge-accounts').doc(memoHash).set({
        address: nodlePublicKey,
    }, { merge: true });

    response.send({ memoHash: memoHash });
});