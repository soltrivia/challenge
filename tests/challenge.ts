import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Challenge } from "../target/types/challenge";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";

async function airdropSol(publicKey, amount) {
  let airdropTx = await anchor
    .getProvider()
    .connection.requestAirdrop(publicKey, amount);
  await confirmTransaction(airdropTx);
}

async function confirmTransaction(tx) {
  const latestBlockHash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: tx,
  });
}
async function MycreateMint(provider, authority) {
  if (authority === undefined) {
    authority = provider.wallet.publicKey;
  }
  const mint = await createMint(
    provider.connection,
    provider.wallet.payer,
    provider.wallet.publicKey,
    provider.wallet.publicKey,
    6
  );
  return mint;
}

describe("challenge", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const alice = anchor.web3.Keypair.generate();
  const bob = anchor.web3.Keypair.generate();
  const alicenewKp = anchor.web3.Keypair.generate();
  const bobnewKp = anchor.web3.Keypair.generate();

  const program = anchor.workspace.Challenge as Program<Challenge>;
  const [programState] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("program_state")],
    program.programId
  );
  const [solpda] = anchor.web3.PublicKey.findProgramAddressSync(
    [programState.toBytes(), Buffer.from("liq_sol")],
    program.programId
  );
  it("Is initialized!", async () => {
    await airdropSol(alice.publicKey, 5 * anchor.web3.LAMPORTS_PER_SOL); //await anchor.getProvider().connection.requestAirdrop(alice.publicKey, 5 * anchor.web3.LAMPORTS_PER_SOL);
    await airdropSol(bob.publicKey, 5 * anchor.web3.LAMPORTS_PER_SOL);

    console.log(
      "alice",
      alice.publicKey,
      await provider.connection.getBalance(alice.publicKey)
    );
    console.log(
      "bob",
      bob.publicKey,
      await provider.connection.getBalance(bob.publicKey)
    );
    const tx = await program.methods
      .initialize()
      .accounts({
        programState: programState,
        signer: provider.wallet.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
  it("deposit_sol", async () => {
    const [depositaddr] = anchor.web3.PublicKey.findProgramAddressSync(
      [alice.publicKey.toBytes()],
      program.programId
    );
    const tx = await program.methods
      .depositSol(new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL))
      .accounts({
        programState,
        signer: alice.publicKey,
        deposit: depositaddr,
        bank: solpda,
      })
      .signers([alice])
      .rpc();
    console.log("Your transaction signature", tx);
    await program.provider.connection.confirmTransaction(tx);
    const currentProgramState = await program.account.programState.fetch(
      programState
    );
    console.log(
      "currentProgramState totalDeposits ",
      currentProgramState.totalDeposits.toNumber()
    );
  });
  it("deposit_sol alice 2", async () => {
    //const alicenewKp = anchor.web3.Keypair.generate();
    const [depositaddr] = anchor.web3.PublicKey.findProgramAddressSync(
      [alice.publicKey.toBytes()],
      program.programId
    );
    const tx = await program.methods
      .depositSol(new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL))
      .accounts({
        programState,
        signer: alice.publicKey,
        deposit: depositaddr,
        //deposit: alicenewKp.publicKey,
        bank: solpda,
      })
      .signers([alice])
      .rpc();
    console.log("Your transaction signature", tx);
    await program.provider.connection.confirmTransaction(tx);
    const currentProgramState = await program.account.programState.fetch(
      programState
    );
    console.log(
      "currentProgramState totalDeposits ",
      currentProgramState.totalDeposits.toNumber()
    );
    const deposit = await program.account.deposit.fetch(depositaddr);
    console.log(
      "deposit",
      deposit.amount.toNumber(),
      deposit.authority.toBase58()
    );
  });

  it("deposit_sol2", async () => {
    const [depositaddr] = anchor.web3.PublicKey.findProgramAddressSync(
      [bob.publicKey.toBytes()],
      program.programId
    );

    //const alicenewKp = anchor.web3.Keypair.generate();
    const tx = await program.methods
      .depositSol(new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL))
      .accounts({
        programState,
        signer: bob.publicKey,
        deposit: depositaddr,
        bank: solpda,
      })
      .signers([bob])
      .rpc();
    console.log("Your transaction signature", tx);
    await program.provider.connection.confirmTransaction(tx);
    const currentProgramState = await program.account.programState.fetch(
      programState
    );
    console.log(
      "currentProgramState totalDeposits ",
      currentProgramState.totalDeposits.toNumber()
    );
    const deposit = await program.account.deposit.fetch(depositaddr);
    console.log(
      "deposit",
      deposit.amount.toNumber(),
      deposit.authority.toBase58()
    );
  });
  it("create_challenge bob", async () => {
    const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
      [bobnewKp.publicKey.toBytes(), Buffer.from("vault")],
      program.programId
    );
    const [depositaddr] = anchor.web3.PublicKey.findProgramAddressSync(
      [bob.publicKey.toBytes()],
      program.programId
    );
    const tx = await program.methods
      .createChallenge(new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL))
      .accounts({
        programState,
        creater: provider.wallet.publicKey,
        challenge: bobnewKp.publicKey,
        ower: bob.publicKey,
        vault: vault,
        bank: solpda,
        deposit: depositaddr,
        //bank: solpda,
      })
      .signers([provider.wallet.payer, bobnewKp])
      .rpc();
    await program.provider.connection.confirmTransaction(tx);
    console.log("vault", await provider.connection.getBalance(vault));

    const challenge = await program.account.challengeState.fetch(
      bobnewKp.publicKey
    );
    console.log("challage", challenge.user.toBase58());
  });

  it("join_challenge alice", async () => {
    const [depositaddr] = anchor.web3.PublicKey.findProgramAddressSync(
      [alice.publicKey.toBytes()],
      program.programId
    );
    const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
      [bobnewKp.publicKey.toBytes(), Buffer.from("vault")],
      program.programId
    );
    const tx = await program.methods
      .joinChallenge()
      .accounts({
        programState,
        signer: provider.wallet.publicKey,
        challenge: bobnewKp.publicKey,
        ower: alice.publicKey,
        vault: vault,
        deposit: depositaddr,
        bank: solpda,
      })
      .rpc();
    await program.provider.connection.confirmTransaction(tx);
    console.log("vault", await provider.connection.getBalance(vault));
    const challenge = await program.account.challengeState.fetch(
      bobnewKp.publicKey
    );
    console.log("challage", challenge.accpetUser.toBase58());
  });
  it("end_challenge alice", async () => {
    const [depositaddr] = anchor.web3.PublicKey.findProgramAddressSync(
      [alice.publicKey.toBytes()],
      program.programId
    );
    const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
      [bobnewKp.publicKey.toBytes(), Buffer.from("vault")],
      program.programId
    );
    const tx = await program.methods
      .endChallenge()
      .accounts({
        programState,
        signer: provider.wallet.publicKey,
        challenge: bobnewKp.publicKey,
        //ower: alice.publicKey,
        vault: vault,
        //deposit: depositaddr,
        //bank: solpda,
        winner: alice.publicKey,
      })
      .rpc();
    await program.provider.connection.confirmTransaction(tx);
    console.log("vault", await provider.connection.getBalance(vault));
    const challenge = await program.account.challengeState.fetch(
      bobnewKp.publicKey
    );
    console.log("challage", challenge.accpetUser.toBase58());
    //console.log("vault", await provider.connection.getBalance(vault));
  });
});
