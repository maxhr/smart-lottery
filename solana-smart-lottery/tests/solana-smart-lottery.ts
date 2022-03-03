import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { SolanaSmartLottery } from "../target/types/solana_smart_lottery";

describe("solana-smart-lottery", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolanaSmartLottery as Program<SolanaSmartLottery>;
  const smartLottery = Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({
      accounts: {
        smartLottery: smartLottery.publicKey,
        authtority: program.provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [smartLottery],
    });
    console.log("Your transaction signature", tx);
  });
});
