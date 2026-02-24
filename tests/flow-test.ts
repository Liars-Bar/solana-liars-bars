// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { LiarsBarDapp } from "../target/types/liars_bar_dapp";
// import {
//   Keypair,
//   LAMPORTS_PER_SOL,
//   PublicKey,
//   SystemProgram,
// } from "@solana/web3.js";
// import { assert } from "chai";

// describe("liars-bars full game flow", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);

//   const program = anchor.workspace.LiarsBarDapp as Program<LiarsBarDapp>;
//   const tableId = new anchor.BN(Date.now());
//   const tableIdBuf = tableId.toArrayLike(Buffer, "le", 16);

//   const INCO_LIGHTNING_PROGRAM_ID = new PublicKey(
//     "5sjEbPiqgZrYwR31ahR6Uk9wf5awoX61YGg7jExQSwaj",
//   );

//   // Player 1 = provider wallet, Player 2 = new keypair
//   const player1 = (provider.wallet as anchor.Wallet).payer;
//   const player2 = Keypair.generate();

//   // ── Helpers ──────────────────────────────────────────────────

//   function tablePDA(): PublicKey {
//     const [pda] = PublicKey.findProgramAddressSync(
//       [Buffer.from("table"), tableIdBuf],
//       program.programId,
//     );
//     return pda;
//   }

//   function playerPDA(pubkey: PublicKey): PublicKey {
//     const [pda] = PublicKey.findProgramAddressSync(
//       [Buffer.from("player"), tableIdBuf, pubkey.toBuffer()],
//       program.programId,
//     );
//     return pda;
//   }

//   function getKeypair(pubkey: PublicKey): Keypair {
//     if (pubkey.equals(player1.publicKey)) return player1;
//     if (pubkey.equals(player2.publicKey)) return player2;
//     throw new Error("Unknown player");
//   }

//   function extra(player: Keypair): Keypair[] {
//     return player.publicKey.equals(player1.publicKey) ? [] : [player];
//   }

//   function label(pubkey: PublicKey): string {
//     if (pubkey.equals(player1.publicKey)) return "Player1(alice)";
//     if (pubkey.equals(player2.publicKey)) return "Player2(bob)";
//     return pubkey.toString().slice(0, 8) + "...";
//   }

//   // ── Instruction wrappers ─────────────────────────────────────

//   async function createTable() {
//     return program.methods
//       .createTable(tableId)
//       .accounts({
//         signer: player1.publicKey,
//         table: tablePDA(),
//         systemProgram: SystemProgram.programId,
//         incoLightningProgram: INCO_LIGHTNING_PROGRAM_ID,
//       } as any)
//       .rpc();
//   }

//   async function joinTable(player: Keypair, characterId: string) {
//     return program.methods
//       .joinTable(tableId, characterId)
//       .accounts({
//         signer: player.publicKey,
//         table: tablePDA(),
//         players: playerPDA(player.publicKey),
//         systemProgram: SystemProgram.programId,
//         incoLightningProgram: INCO_LIGHTNING_PROGRAM_ID,
//       } as any)
//       .signers(extra(player))
//       .rpc();
//   }

//   async function startRound(player: Keypair) {
//     return program.methods
//       .startRound(tableId)
//       .accounts({
//         signer: player.publicKey,
//         table: tablePDA(),
//         players: playerPDA(player.publicKey),
//         systemProgram: SystemProgram.programId,
//         incoLightningProgram: INCO_LIGHTNING_PROGRAM_ID,
//       } as any)
//       .signers(extra(player))
//       .rpc();
//   }

//   async function shuffleCards(player: Keypair) {
//     return program.methods
//       .suffleCards(tableId)
//       .accounts({
//         signer: player.publicKey,
//         table: tablePDA(),
//         players: playerPDA(player.publicKey),
//         incoLightningProgram: INCO_LIGHTNING_PROGRAM_ID,
//         systemProgram: SystemProgram.programId,
//       } as any)
//       .signers(extra(player))
//       .rpc();
//   }

//   async function placeCards(player: Keypair, indices: number[]) {
//     return program.methods
//       .placeCards(tableId, Buffer.from(indices))
//       .accounts({
//         user: player.publicKey,
//         table: tablePDA(),
//         player: playerPDA(player.publicKey),
//         systemProgram: SystemProgram.programId,
//         incoLightningProgram: INCO_LIGHTNING_PROGRAM_ID,
//       } as any)
//       .signers(extra(player))
//       .rpc();
//   }

//   async function liarsCall(player: Keypair) {
//     return program.methods
//       .liarsCall(tableId)
//       .accounts({
//         signer: player.publicKey,
//         table: tablePDA(),
//         players: playerPDA(player.publicKey),
//         systemProgram: SystemProgram.programId,
//         incoLightningProgram: INCO_LIGHTNING_PROGRAM_ID,
//       } as any)
//       .signers(extra(player))
//       .rpc();
//   }

//   async function fetchTable() {
//     return program.account.liarsTable.fetch(tablePDA());
//   }

//   async function fetchPlayer(pubkey: PublicKey) {
//     return program.account.player.fetch(playerPDA(pubkey));
//   }

//   // ── Event tracking ───────────────────────────────────────────

//   let gameOverFired = false;
//   let winnerPubkey: PublicKey | null = null;
//   const eventLog: string[] = [];

//   before("airdrop SOL to player2", async () => {
//     const sig = await provider.connection.requestAirdrop(
//       player2.publicKey,
//       2 * LAMPORTS_PER_SOL,
//     );
//     await provider.connection.confirmTransaction(sig, "confirmed");
//     console.log(`  Airdropped 2 SOL to ${label(player2.publicKey)}`);
//   });

//   before("setup event listeners", () => {
//     program.addEventListener("liarsTableCreated", () => {
//       eventLog.push("TableCreated");
//       console.log(`    [event] Table created`);
//     });
//     program.addEventListener("playerJoined", (e) => {
//       eventLog.push(`Joined(${label(e.player)})`);
//       console.log(`    [event] ${label(e.player)} joined`);
//     });
//     program.addEventListener("roundStarted", () => {
//       eventLog.push("RoundStarted");
//       console.log(`    [event] Round started`);
//     });
//     program.addEventListener("suffleCardsForPlayer", (e) => {
//       eventLog.push(`Shuffled(${label(e.player)})`);
//       console.log(`    [event] Cards dealt to ${label(e.player)}`);
//     });
//     program.addEventListener("cardPlaced", (e) => {
//       eventLog.push(`CardPlaced(${label(e.player)})`);
//       console.log(`    [event] ${label(e.player)} placed card(s)`);
//     });
//     program.addEventListener("tableTrun", (e) => {
//       eventLog.push(`Turn(${label(e.player)})`);
//       console.log(`    [event] Turn -> ${label(e.player)}`);
//     });
//     program.addEventListener("emptyBulletFired", (e) => {
//       eventLog.push(`EmptyBullet(${label(e.player)})`);
//       console.log(`    [event] Empty bullet! ${label(e.player)} survives`);
//     });
//     program.addEventListener("playerEleminated", (e) => {
//       eventLog.push(`Eliminated(${label(e.player)})`);
//       console.log(`    [event] ${label(e.player)} ELIMINATED`);
//     });
//     program.addEventListener("gameWinner", (e) => {
//       winnerPubkey = e.player;
//       eventLog.push(`Winner(${label(e.player)})`);
//       console.log(`    [event] WINNER: ${label(e.player)}`);
//     });
//     program.addEventListener("gameOver", () => {
//       gameOverFired = true;
//       eventLog.push("GameOver");
//       console.log(`    [event] GAME OVER`);
//     });
//   });

//   // ── Game flow ────────────────────────────────────────────────

//   it("Step 1: Create table", async () => {
//     await createTable();
//     const table = await fetchTable();
//     assert.isTrue(table.isOpen);
//     assert.equal(table.players.length, 0);
//     console.log(`  Table PDA: ${tablePDA().toString()}`);
//   });

//   it("Step 2: Player1 (alice) joins", async () => {
//     await joinTable(player1, "alice");
//     const table = await fetchTable();
//     assert.equal(table.players.length, 1);
//   });

//   it("Step 3: Player2 (bob) joins", async () => {
//     await joinTable(player2, "bob");
//     const table = await fetchTable();
//     assert.equal(table.players.length, 2);
//   });

//   it("Step 4: Start round", async () => {
//     await startRound(player1);
//     const table = await fetchTable();
//     assert.isFalse(table.isOpen);
//     console.log(`  Table card suit: ${table.tableCard}`);
//     console.log(`  Turn to play: ${table.trunToPlay}`);
//     console.log(`  Bullets: [${table.remainingBullet}]`);
//   });

//   it("Step 5: Deal cards to both players", async () => {
//     const table = await fetchTable();
//     for (let i = 0; i < table.players.length; i++) {
//       const kp = getKeypair(table.players[i]);
//       await shuffleCards(kp);
//       const pData = await fetchPlayer(kp.publicKey);
//       console.log(`  ${label(kp.publicKey)} got ${pData.cards.length} cards`);
//     }
//   });

//   it("Step 6: Play until game over", async () => {
//     const MAX_ITERATIONS = 20;

//     for (let i = 1; i <= MAX_ITERATIONS; i++) {
//       let table = await fetchTable();
//       if (table.players.length <= 1 || gameOverFired) {
//         console.log(`  Game ended.`);
//         break;
//       }

//       console.log(`\n  ── Iteration ${i} ──`);

//       // Who places cards?
//       const turnIdx = table.trunToPlay % table.players.length;
//       const placer = getKeypair(table.players[turnIdx]);

//       // Who calls liar? (next player)
//       const callerIdx = (turnIdx + 1) % table.players.length;
//       const caller = getKeypair(table.players[callerIdx]);

//       // Check placer has cards
//       const placerData = await fetchPlayer(placer.publicKey);
//       if (placerData.cards.length === 0) {
//         console.log(`  ${label(placer.publicKey)} has no cards — skipping place`);
//       } else {
//         console.log(`  ${label(placer.publicKey)} places 1 card...`);
//         try {
//           await placeCards(placer, [0]);
//         } catch (err: any) {
//           console.log(`  placeCards failed: ${err.message?.slice(0, 150)}`);
//           break;
//         }
//       }

//       // Next player calls liar
//       console.log(`  ${label(caller.publicKey)} calls LIAR!`);
//       try {
//         await liarsCall(caller);
//       } catch (err: any) {
//         console.log(`  liarsCall failed: ${err.message?.slice(0, 150)}`);
//         break;
//       }

//       // Small delay for events
//       await new Promise((r) => setTimeout(r, 1000));

//       // Check game state
//       table = await fetchTable();
//       console.log(`  Players remaining: ${table.players.length}`);

//       if (table.players.length <= 1 || gameOverFired) {
//         console.log(`\n  Game Over after iteration ${i}!`);
//         break;
//       }

//       // Game continues → round was auto-reset → re-shuffle
//       console.log(`  Round reset — re-dealing cards...`);
//       for (let s = 0; s < table.players.length; s++) {
//         const shuffler = getKeypair(table.players[s]);
//         try {
//           await shuffleCards(shuffler);
//           console.log(`    ${label(shuffler.publicKey)} re-dealt`);
//         } catch (err: any) {
//           console.log(
//             `    Shuffle failed for ${label(shuffler.publicKey)}: ${err.message?.slice(0, 120)}`,
//           );
//         }
//       }
//     }
//   });

//   it("Step 7: Verify final state", async () => {
//     const table = await fetchTable();
//     console.log(`\n  ── Final State ──`);
//     console.log(`  Players remaining: ${table.players.length}`);
//     console.log(`  GameOver event: ${gameOverFired}`);
//     console.log(`  Winner: ${winnerPubkey ? label(winnerPubkey) : "N/A"}`);
//     console.log(`  Events: ${eventLog.join(" -> ")}`);

//     if (gameOverFired) {
//       assert.equal(table.players.length, 1);
//       assert.isNotNull(winnerPubkey);
//       console.log(`\n  WINNER: ${label(table.players[0])}`);
//     }
//   });
// });
