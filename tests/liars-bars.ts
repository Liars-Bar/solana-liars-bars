import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LiarsBarDapp } from "../target/types/liars_bar_dapp";
import { PublicKey } from "@solana/web3.js";

describe("liars-bars", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LiarsBarDapp as Program<LiarsBarDapp>;

  it("create-room", async () => {
    const roomId = new anchor.BN(1);

    const tx = await program.methods
      .createRoom(roomId)
      .rpc();

    console.log("Tx:", tx);

    const [roomAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("room"),
        roomId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    const room = await program.account.gameRoom.fetch(roomAddress);
    console.log(room);
  });
});
