import {Keypair} from "@solana/web3.js";
import fs from "fs";

export function loadWalletKey(keypair: string): Keypair {
    if (!keypair || keypair == "") {
        throw new Error("Keypair is required!");
    }
    const loaded = Keypair.fromSecretKey(
        new Uint8Array(JSON.parse(fs.readFileSync(keypair).toString())),
    );
    console.log(`wallet public key: ${loaded.publicKey}`);
    return loaded;
}
