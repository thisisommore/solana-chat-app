import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { ChatApp } from "../target/types/chat_app";

describe("chat-app", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.ChatApp as Program<ChatApp>;

  const otherUser = new anchor.web3.Keypair()

  it("Should create user", async () => {
    const USER_SEED = "user"
    const [USER_PDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(USER_SEED), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods.createUser("abcd").accounts({
      userAccount: USER_PDA,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
  })

  it("Should put message", async () => {

    let MESSAGE_ACCOUNT = anchor.web3.Keypair.generate()

    await program.methods.putMessage("Hello").accounts({
      message: MESSAGE_ACCOUNT.publicKey,
      sender: provider.wallet.publicKey,
      receiver: otherUser.publicKey
    }).signers([MESSAGE_ACCOUNT]).rpc();
    console.log(provider.wallet.publicKey.toBase58());

    MESSAGE_ACCOUNT = anchor.web3.Keypair.generate()
    await program.methods.putMessage("How are you doing?").accounts({
      message: MESSAGE_ACCOUNT.publicKey,
      sender: provider.wallet.publicKey,
      receiver: otherUser.publicKey
    }).signers([MESSAGE_ACCOUNT]).rpc();
    const airdrop_signature = await program.provider.connection.requestAirdrop(otherUser.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(airdrop_signature);
    MESSAGE_ACCOUNT = anchor.web3.Keypair.generate()
    await program.methods.putMessage("I am doing well").accounts({
      message: MESSAGE_ACCOUNT.publicKey,
      sender: otherUser.publicKey,
      receiver: provider.wallet.publicKey
    }).signers([otherUser, MESSAGE_ACCOUNT]).rpc();
    const fecthed_accounts = await program.account.message.all([
      {
        memcmp: {
          offset: 8,
          bytes: provider.wallet.publicKey.toBase58()
        }
      }
    ])

    expect(fecthed_accounts.map(e => e.account.contentHash)).to.eql(["Hello", "How are you doing?"])
    // expect(chat_account.messages.map(e => e.contentHash)).to.eq([true])
  })
});
