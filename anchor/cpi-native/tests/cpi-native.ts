import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CpiNative } from "../target/types/cpi_native";
import { Connection } from "@solana/web3.js";
import { expect } from "chai";

describe("cpi-native", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const connection = anchor.getProvider().connection;

  const program = anchor.workspace.cpiNative as Program<CpiNative>;
  const newAccount = anchor.web3.Keypair.generate();
  const programId = new anchor.web3.PublicKey(
    "HDXB678We1HjkpXQ8mBctBfpoZTcTkJK6rzWYFymfb3N"
  );

  it("Is initialized!", async () => {
    // Add your test here.

    const tx = await program.methods
      .initialize()
      .accounts({
        dataAccount: newAccount.publicKey,
        payer: anchor.getProvider().publicKey,
        cpiProgram: programId,
      })
      .signers([newAccount])
      .rpc();

    console.log("Your transaction signature ", tx);
    console.log("Your new account ", newAccount.publicKey.toBase58());

    const account_info = await connection.getAccountInfo(newAccount.publicKey);

    const count = account_info?.data.readUInt32LE(0);

    console.log("Raw buffer:", account_info?.data);
    console.log("Counter value:", count);

    expect(count).to.equal(1);
  });

  it("Double ", async () => {
    const tx = await program.methods
      .double()
      .accounts({
        payer: anchor.getProvider().publicKey,
        dataAccount: newAccount.publicKey,
        cpiProgram: programId,
      })
      .signers([newAccount])
      .rpc();
    console.log("Your transaction signature ", tx);

    const account_info = await connection.getAccountInfo(newAccount.publicKey);

    const count = account_info?.data.readUInt32LE(0);

    console.log("Raw buffer:", account_info?.data);
    console.log("Counter value:", count);

    expect(count).to.equal(2);
  });
});
