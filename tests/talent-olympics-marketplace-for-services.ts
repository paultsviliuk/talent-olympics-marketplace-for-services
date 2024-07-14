import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TalentOlympicsMarketplaceForServices } from "../target/types/talent_olympics_marketplace_for_services";
import { expect } from "chai";

describe("test_marketplace", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TalentOlympicsMarketplaceForServices as Program<TalentOlympicsMarketplaceForServices>;

  const user = provider.wallet;
  const vendor = anchor.web3.Keypair.generate();

  it("Registers a new vendor", async () => {
    await program.methods
      .registerVendor("Test Vendor")
      .accounts({
        vendor: vendor.publicKey,
        user: user.publicKey,
      })
      .signers([vendor])
      .rpc();

    const vendorAccount = await program.account.vendor.fetch(vendor.publicKey);
    expect(vendorAccount.name).to.equal("Test Vendor");
    expect(vendorAccount.owner.toString()).to.equal(user.publicKey.toString());
  });
});
