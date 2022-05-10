const {
  Connection,
  sendAndConfirmTransaction,
  Keypair,
  Transaction,
  SystemProgram,
  PublicKey,
  TransactionInstruction,
} = require("@solana/web3.js");

const main = async () => {
  var args = process.argv.slice(2);
  // args[0]: Program ID
  // args[1] (Optional): Echo buffer account
  const programId = new PublicKey(args[0]);

  console.log(programId.toBase58());
  const connection = new Connection("https://api.devnet.solana.com/");
  const feePayer = new Keypair();

  console.log("Requesting Airdrop of 1 SOL...");
  await connection.requestAirdrop(feePayer.publicKey, 2e9);
  console.log("Airdrop received");

  const echo = new Keypair();
  let echoKey = echo.publicKey;
  let tx = new Transaction();
  let signers = [feePayer];

  const msg_offset = 1 + 4;
  const msg = "Hello, Solana!";
  const idx = Buffer.allocUnsafe(msg_offset + msg.length);
  let offset = idx.writeUint8(0);
  offset = idx.writeUint32LE(msg.length, offset);
  offset = idx.write(msg, offset);

  if (args.length > 1) {
    console.log("Found echo address");
    echoKey = new PublicKey(args[1]);
  } else {
    console.log("Generating new echo address");
    let createIx = SystemProgram.createAccount({
      fromPubkey: feePayer.publicKey,
      newAccountPubkey: echoKey,
      /** Amount of lamports to transfer to the created account */
      lamports: await connection.getMinimumBalanceForRentExemption(idx.length),
      /** Amount of space in bytes to allocate to the created account */
      space: idx.length,
      /** Public key of the program to assign as the owner of the created account */
      programId: programId,
    });
    signers.push(echo);
    tx.add(createIx);
  }

  let echoIx = new TransactionInstruction({
    keys: [
      {
        pubkey: echoKey,
        isSigner: false,
        isWritable: true,
      }
    ],
    programId: programId,
    data: idx,
  });
  /*
    TransactionInstruction({
      keys: Array<AccountMeta>,
      programId: PublicKey,
      data: Buffer,
    });
  */
  tx.add(echoIx);

  let txid = await sendAndConfirmTransaction(connection, tx, signers, {
    skipPreflight: true,
    preflightCommitment: "confirmed",
    confirmation: "confirmed",
  });
  console.log(`https://explorer.solana.com/tx/${txid}?cluster=devnet`);

  data = (await connection.getAccountInfo(echoKey)).data;
  console.log("Echo key:", echoKey.toBase58());
  console.log("Echo data: ", data);
};

main()
  .then(() => {
    console.log("Success");
  })
  .catch((e) => {
    console.error(e);
  });
