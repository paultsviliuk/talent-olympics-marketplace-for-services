import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TalentOlympicsMarketplaceForServices } from "../target/types/talent_olympics_marketplace_for_services";
import { expect } from "chai";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("talent_olympics_marketplace_for_services", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TalentOlympicsMarketplaceForServices as Program<TalentOlympicsMarketplaceForServices>;

  const user = provider.wallet;
  const vendor = anchor.web3.Keypair.generate();
  const service = anchor.web3.Keypair.generate();
  const serviceNFT = anchor.web3.Keypair.generate();
  const mintAuthority = anchor.web3.Keypair.generate();
  const metadataAccount = anchor.web3.Keypair.generate();

  it("Registers a new vendor", async () => {
    await program.methods
      .registerVendor("Test Vendor")
      .accounts({
        vendor: vendor.publicKey,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([vendor])
      .rpc();

    const vendorAccount = await program.account.vendor.fetch(vendor.publicKey);
    expect(vendorAccount.name).to.equal("Test Vendor");
    expect(vendorAccount.owner.toString()).to.equal(user.publicKey.toString());
  });

  it("Create a service", async () => {

    await program.methods.createService(
      "Service1",
      "This is a test service",
      new anchor.BN(1000),
      true,
      "https://metadata.uri/service1"
    )
      .accounts({
        vendor: vendor.publicKey,
        service: service.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([vendor])
      .rpc();

    const serviceAccount = await program.account.serviceListing.fetch(service.publicKey);
    expect(serviceAccount.name).to.equal("Service1");
    expect(serviceAccount.description).to.equal("This is a test service");
    expect(serviceAccount.price.toNumber()).to.equal(1000);
    expect(serviceAccount.isSoulbound).to.equal(true);
    expect(serviceAccount.metadataUri).to.equal("https://metadata.uri/service1");
  });

  it("Purchases a service", async () => {
    // Define the buyer
    const buyer = anchor.web3.Keypair.generate();

    // Airdrop SOL to the buyer for testing purposes
    const airdropSignature = await provider.connection.requestAirdrop(buyer.publicKey, anchor.web3.LAMPORTS_PER_SOL);

    // confirm transaction
    await provider.connection.confirmTransaction({
      signature: airdropSignature,
      lastValidBlockHeight: await provider.connection.getLatestBlockhash().then(res => res.lastValidBlockHeight),
      blockhash: await provider.connection.getLatestBlockhash().then(res => res.blockhash),
    });

    // Fetch the mint public key
    const mint = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("mint"), serviceNFT.publicKey.toBuffer()],
      program.programId
    );

    // Fetch the token account public key
    const serviceNFTAccount = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("service_nft"), serviceNFT.publicKey.toBuffer(), buyer.publicKey.toBuffer()],
      program.programId
    );

    await program.methods.purchaseService()
      .accounts({
        serviceListing: service.publicKey,
        serviceNft: serviceNFT.publicKey,
        metadataAccount: metadataAccount.publicKey,
        metadataPda: metadataAccount.publicKey,
        buyer: buyer.publicKey,
        mint: mint[0],
        serviceNftAccount: serviceNFTAccount[0],
        mintAuthority: mintAuthority.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([buyer, serviceNFT, metadataAccount, mintAuthority])
      .rpc();

    const serviceNftAccount = await program.account.serviceNft.fetch(serviceNFT.publicKey);
    expect(serviceNftAccount.currentOwner.toString()).to.equal(buyer.publicKey.toString());
  });
});
