import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LiarsBarDapp } from "../target/types/liars_bar_dapp";
import {
  ComputeBudgetProgram,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { decrypt } from "@inco/solana-sdk";
import nacl from "tweetnacl";

describe("liars-bars", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const tableId = new anchor.BN(Date.now());

  const program = anchor.workspace.LiarsBarDapp as Program<LiarsBarDapp>;

  // You need to know the IncoLightning program ID
  const INCO_LIGHTNING_PROGRAM_ID = new PublicKey(
    "5sjEbPiqgZrYwR31ahR6Uk9wf5awoX61YGg7jExQSwaj",
  );

  const dummyPlayers: Keypair[] = [
    Keypair.generate(),
    Keypair.generate(),
    Keypair.generate(),
    Keypair.generate(),
  ];

  // async function airdropSol(publicKey: PublicKey, amount: number = 1) {
  //   const signature = await provider.connection.requestAirdrop(
  //     publicKey,
  //     amount * LAMPORTS_PER_SOL,
  //   );
  //   await provider.connection.confirmTransaction(signature, "confirmed");
  //   console.log(`Airdropped ${amount} SOL to ${publicKey.toString()}`);
  // }

  // before("initializing-event-listner", async () => {
  //   program.addEventListener("liarsTableCreated", (event, slot, signature) => {
  //     console.log("liarsTableCreated event:", event.tableId.toNumber());
  //     // console.log("tx:", signature);
  //     // console.log("slots :", slot);
  //   });

  //   program.addEventListener("playerJoined", (event, slot, signature) => {
  //     console.log("playerJoined", event);
  //     console.log(event.player.toString());
  //     console.log(event.tableId.toNumber());
  //   });

  //   program.addEventListener("cardPlaced", (event, slot, signature) => {
  //     console.log("cardPlaced", event);
  //     console.log(event.player.toString());
  //     console.log(event.tableId.toNumber());
  //   });

  //   program.addEventListener("roundStarted", (event, slot, signature) => {
  //     console.log("roundStarted", event);
  //     // console.log(event..toString());
  //     console.log(event.tableId.toNumber());
  //   });

  //   program.addEventListener(
  //     "suffleCardsForPlayer",
  //     (event, slot, signature) => {
  //       console.log("suffleCardsForPlayer", event);
  //       console.log(event.player.toString());
  //       console.log(event.tableId.toNumber());
  //       console.log(event.next.toString());
  //     },
  //   );
  // });

  it("create-table", async () => {
    const [tableAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("table"), tableId.toArrayLike(Buffer, "le", 16)],
      program.programId,
    );

    const tx = await program.methods
      .createTable(tableId)
      .accounts({
        signer: provider.wallet.publicKey,
        table: tableAddress,
        systemProgram: SystemProgram.programId,
        incoLightningProgram: INCO_LIGHTNING_PROGRAM_ID,
      } as any)
      .rpc();

    console.log("Tx:", tx);
  });

  it("join-table", async () => {
    const [tableAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("table"), tableId.toArrayLike(Buffer, "le", 16)],
      program.programId,
    );

    const [playerAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("player"),
        tableId.toArrayLike(Buffer, "le", 16),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId,
    );

    const tx = await program.methods
      .joinTable(tableId, "dog")
      .accounts({
        signer: provider.wallet.publicKey,
        table: tableAddress,
        player: playerAddress,
        systemProgram: SystemProgram.programId,
        incoLightningProgram: INCO_LIGHTNING_PROGRAM_ID,
      } as any)
      .rpc();

    console.log("Tx:", tx);

    const table = await program.account.liarsTable.fetch(tableAddress);

    console.log(table);

    const player = await program.account.player.fetch(playerAddress);

    console.log(player);
  });

  it("check table", async () => {
    const [tableAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("table"), tableId.toArrayLike(Buffer, "le", 16)],
      program.programId,
    );
    const table = await program.account.liarsTable.fetch(tableAddress);

    console.log("table data");
    console.log(table);
  });

  it("start round", async () => {
    const [tableAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("table"), tableId.toArrayLike(Buffer, "le", 16)],
      program.programId,
    );
    const tx = await program.methods
      .startRound(tableId)
      .accounts({
        signer: provider.wallet.publicKey,
        table: tableAddress,
        // player: playerAddress,
        systemProgram: SystemProgram.programId,
        incoLightningProgram: INCO_LIGHTNING_PROGRAM_ID,
      } as any)
      .rpc();

    console.log("Tx:", tx);
  });

  it("suffle cards", async () => {
    const [tableAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("table"), tableId.toArrayLike(Buffer, "le", 16)],
      program.programId,
    );

    const [playerAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("player"),
        tableId.toArrayLike(Buffer, "le", 16), // Should be 16 bytes, not 8
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId,
    );

    const tx = await program.methods
      .suffleCards(tableId)
      .accounts({
        signer: provider.wallet.publicKey,
        table: tableAddress,
        players: playerAddress,
        incoLightningProgram: INCO_LIGHTNING_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      } as any)
      .rpc();

    console.log("Tx:", tx);
  });

  it("decrypt cards", async () => {
    const [playerAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("player"),
        tableId.toArrayLike(Buffer, "le", 16),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId,
    );

    // 1. Fetch player account to get encrypted card handles
    const player = await program.account.player.fetch(playerAddress);
    console.log("Player has", player.cards.length, "encrypted cards");

    // Helper: extract u128 handle from Anchor-deserialized Euint128 tuple struct
    function extractHandle(euint128: any): bigint {
      // Anchor deserializes Euint128(u128) as an array-like object
      // Could be: [BN], { "0": BN }, or BN directly
      if (euint128 && euint128._bn) return BigInt(euint128.toString());
      if (euint128 && euint128["0"]) return BigInt(euint128["0"].toString());
      if (Array.isArray(euint128) && euint128.length > 0)
        return BigInt(euint128[0].toString());
      return BigInt(0);
    }

    // Helper: derive allowance PDA from handle + allowed address
    function deriveAllowancePda(
      handle: bigint,
      allowedAddress: PublicKey,
    ): [PublicKey, number] {
      const handleBuffer = Buffer.alloc(16);
      let h = handle;
      for (let i = 0; i < 16; i++) {
        handleBuffer[i] = Number(h & BigInt(0xff));
        h = h >> BigInt(8);
      }
      return PublicKey.findProgramAddressSync(
        [handleBuffer, allowedAddress.toBuffer()],
        INCO_LIGHTNING_PROGRAM_ID,
      );
    }

    // 2. Derive allowance PDAs for each card's shape and value handles
    const remainingAccounts: {
      pubkey: PublicKey;
      isSigner: boolean;
      isWritable: boolean;
    }[] = [];
    const handles: { shape: string; value: string }[] = [];

    for (const card of player.cards) {
      const shapeHandle = extractHandle(card.shape);
      const valueHandle = extractHandle(card.value);
      handles.push({
        shape: shapeHandle.toString(),
        value: valueHandle.toString(),
      });

      const [shapeAllowancePda] = deriveAllowancePda(
        shapeHandle,
        provider.wallet.publicKey,
      );
      const [valueAllowancePda] = deriveAllowancePda(
        valueHandle,
        provider.wallet.publicKey,
      );

      remainingAccounts.push(
        { pubkey: shapeAllowancePda, isSigner: false, isWritable: true },
        { pubkey: valueAllowancePda, isSigner: false, isWritable: true },
      );
    }

    console.log("Card handles:", handles);

    // 3. Call grant_card_access to allow our wallet to decrypt
    const tx = await program.methods
      .grantCardAccess(tableId)
      .accounts({
        signer: provider.wallet.publicKey,
        player: playerAddress,
        incoLightningProgram: INCO_LIGHTNING_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      } as any)
      .remainingAccounts(remainingAccounts)
      .rpc();

    console.log("Grant card access Tx:", tx);

    // 4. Wait for TEE to process the allowances
    await new Promise((r) => setTimeout(r, 3000));

    // 5. Decrypt each card using Inco SDK
    const wallet = (provider.wallet as anchor.Wallet).payer;
    const shapes = ["Spades", "Hearts", "Diamonds", "Clubs"];
    const values = [
      "A",
      "2",
      "3",
      "4",
      "5",
      "6",
      "7",
      "8",
      "9",
      "10",
      "J",
      "Q",
      "K",
    ];

    for (let i = 0; i < handles.length; i++) {
      const result = await decrypt(
        [handles[i].shape, handles[i].value],
        {
          address: wallet.publicKey,
          signMessage: async (msg: Uint8Array) =>
            nacl.sign.detached(msg, wallet.secretKey),
        },
      );

      const shapeIdx = parseInt(result.plaintexts[0]);
      const valueIdx = parseInt(result.plaintexts[1]);

      console.log(
        `Card ${i + 1}: ${values[valueIdx] ?? valueIdx} of ${shapes[shapeIdx] ?? shapeIdx}`,
      );
    }
  });

  it("place cards", async () => {
    const [tableAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("table"), tableId.toArrayLike(Buffer, "le", 16)],
      program.programId,
    );

    const [playerAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("player"),
        tableId.toArrayLike(Buffer, "le", 16),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId,
    );

    const tx = await program.methods
      .placeCards(tableId, Buffer.from([4, 2]))
      .accounts({
        signer: provider.wallet.publicKey,
        table: tableAddress,
        player: playerAddress,
        systemProgram: SystemProgram.programId,
        incoLightningProgram: INCO_LIGHTNING_PROGRAM_ID,
      } as any)
      .rpc();

    console.log("Tx:", tx);

    const table = await program.account.liarsTable.fetch(tableAddress);

    console.log(table);

    const player = await program.account.player.fetch(playerAddress);

    console.log(player);
  });
});
