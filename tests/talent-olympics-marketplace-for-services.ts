import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TalentOlympicsMarketplaceForServices } from "../target/types/talent_olympics_marketplace_for_services";

describe("talent-olympics-marketplace-for-services", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TalentOlympicsMarketplaceForServices as Program<TalentOlympicsMarketplaceForServices>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
