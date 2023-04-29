import { Keypair } from "@solana/web3.js";

//Generate a new keypair
let kp = Keypair.generate()
console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}

To save your wallet, copy and paste the following into a JSON file:

[${kp.secretKey}]`)

// airdrop: https://explorer.solana.com/tx/2zF9NKERinmnCbYjfdGQebqbd32tAVu2xFumo1svnuNwPi49DvdSVcjEYnPEMpJzuoyJr2K8USL3A7UuCjuaetxN?cluster=devnet
// transfer: https://explorer.solana.com/tx/4aoH8BZ7YtbTDnMB9bEynF2pq6vGCd9H7bg6mXGbsyXMLRozwTWxgNyKfRKmFbAZVTSjt9qVXnNia7bZKF5uTstW?cluster=devnet
