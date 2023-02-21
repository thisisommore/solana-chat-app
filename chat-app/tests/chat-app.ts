import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {ChatApp} from "../target/types/chat_app";

describe("chat-app", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.ChatApp as Program<ChatApp>;

  const receiver = new anchor.web3.Keypair()
  it("Is initialized!", async () => {

    const CHAT_SEED = "user-chat"

    const [CHAT_PDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(CHAT_SEED), provider.wallet.publicKey.toBuffer(), receiver.publicKey.toBuffer()],
      program.programId
    );
    // Add your test here.
    const tx = await program.methods.initialize("hemlo").accounts({
      chat: CHAT_PDA,
      user: provider.wallet.publicKey,
      receiver:receiver.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
  });

  it("Should create user", async ()=>{
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

  it("Should put message", async ()=>{
    const CHAT_SEED = "user-chat"

    const [CHAT_PDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(CHAT_SEED), provider.wallet.publicKey.toBuffer(), receiver.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods.putMessage("abcd").accounts({
      chat: CHAT_PDA,
      sender: provider.wallet.publicKey,
    }).rpc();
  })
});
