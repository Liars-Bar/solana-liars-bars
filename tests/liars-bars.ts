import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LiarsBarDapp } from "../target/types/liars_bar_dapp";
import { PublicKey, SystemProgram } from "@solana/web3.js";

describe("liars-bars", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.LiarsBarDapp as Program<LiarsBarDapp>;

  // You need to know the IncoLightning program ID
  const INCO_LIGHTNING_PROGRAM_ID = new PublicKey(
    "5sjEbPiqgZrYwR31ahR6Uk9wf5awoX61YGg7jExQSwaj"
  );

  it("create-table", async () => {
    const roomId = new anchor.BN(3);
    const [tableAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("table"), roomId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    const tx = await program.methods
      .createTable(roomId)
      .accounts({
        signer: provider.wallet.publicKey,
        table: tableAddress,
        systemProgram: SystemProgram.programId,
        incoLightningProgram: INCO_LIGHTNING_PROGRAM_ID,
      } as any)
      .rpc();

    console.log("Tx:", tx);

    const table = await program.account.gameTable.fetch(tableAddress);

    console.log(table);
  });

  it("create-player", async () => {
    const tableId = new anchor.BN(3);
    const [tableAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("player"),
        tableId.toArrayLike(Buffer, "le", 8),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    const tx = await program.methods
      .createPlayer(tableId)
      .accounts({
        signer: provider.wallet.publicKey,
        table: tableAddress,
        systemProgram: SystemProgram.programId,
        incoLightningProgram: INCO_LIGHTNING_PROGRAM_ID,
      } as any)
      .rpc();

    console.log("Tx:", tx);

    const player = await program.account.player.fetch(tableAddress);

    console.log(player);
  });
});
