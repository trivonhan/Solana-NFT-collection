import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftCollection } from "../target/types/nft_collection";
import { SolanaConfigService } from "@coin98/solana-support-library/config";
import { sendTransaction } from "@coin98/solana-support-library";
import { Account, createMint, getAccount, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { collectionAuthorityRecordBeet, Metadata } from "@metaplex-foundation/mpl-token-metadata";

describe("nft-collection", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NftCollection as Program<NftCollection>;
  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  // Root public key and keypair
  let root: anchor.web3.Keypair;
  let mint: anchor.web3.PublicKey;
  let metadataAccount: anchor.web3.PublicKey;
  let rootATA:  Account;
  let collectionAuthorityRecord: anchor.web3.PublicKey;
  let masterEditionAccount: anchor.web3.PublicKey;

  // Second user public key and keypair
  let user2: anchor.web3.Keypair;
  let user2ATA: Account;
  let user2Mint: anchor.web3.PublicKey;
  let user2MetadataAccount: anchor.web3.PublicKey;
  let nft2MasterEditionAccount : anchor.web3.PublicKey;
  before(async () => {
    
    // Get root address
    root = await SolanaConfigService.getDefaultAccount();

    // Create mint
    mint = await createMint(
      connection,
      root,
      root.publicKey,
      root.publicKey,
      0,
    );
    console.log('Mint created: ', mint.toBase58());

    // Create root associated token account
    rootATA = await getOrCreateAssociatedTokenAccount(
      connection,
      root,
      mint,
      root.publicKey,
    );
    console.log('Root ATA created: ', rootATA.address.toBase58());
  });

  it("Mint token to root ATA", async () => {
    console.log('Minting first NFT...');
    const mintToTx = await mintTo(
      connection,
      root,
      mint,
      rootATA.address,
      root.publicKey,
      1
    );
    console.log('Minting first NFT done: ', mintToTx);

    const rootATABalance = await getAccount(connection, rootATA.address);
    console.log('Root ATA balance: ', Number(rootATABalance.amount));
  });

  it('Create metadata account', async () => {
    metadataAccount = findProgramAddressSync(
      [
        Buffer.from('metadata'),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];
    console.log('Metadata account: ', metadataAccount.toBase58());

    const metadataAccountTx = await program.methods.createMetadataAccount([
      {
        address: root.publicKey,
        verified: true,
        share: 100,
      }
    ],
      "Non-fungible token 1",
      "NFT",
      "https://raw.githubusercontent.com/Coding-and-Crypto/Solana-NFT-Marketplace/master/assets/example.json",
      null,
      new anchor.BN(0),
      ).accounts({
        metadataAccount,
        mint,
        mintAuthority: root.publicKey,
        payer: root.publicKey,
        updateAuthority: root.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      }).signers([root]).rpc().catch((err) => {
        console.log(err);
      });

      console.log('Metadata account created: ', metadataAccountTx);
      await new Promise(f => setTimeout(f, 100));

      const metadataInfo = await Metadata.fromAccountAddress(connection, metadataAccount);
      console.log(`Metadata by owner:`, metadataInfo);
  });

  it('Create master edition account', async ()=> {
    masterEditionAccount = findProgramAddressSync(
      [
        Buffer.from('metadata'),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
        Buffer.from('edition'),
      ],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];
    console.log('Master edition account: ', masterEditionAccount.toBase58());

    const masterEditionAccountTx = await program.methods.createMasterEditionAccount(new anchor.BN(0)).accounts({
      masterEditionAccount,
      metadataAccount, 
      mint,
      mintAuthority: root.publicKey,
      payer: root.publicKey,
      updateAuthority: root.publicKey,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    }).signers([root]).rpc();
    console.log('Master edition account created: ', masterEditionAccountTx);
    await new Promise(f => setTimeout(f, 100));

  });

  // it('Update metadata account', async () => {
  //   const updateMetadataAccountTx = await program.methods.updateMetadataAccount(
  //     root.publicKey,
  //     {
  //       symbol: 'UFT1',
  //       uri: "https://raw.githubusercontent.com/Coding-and-Crypto/Solana-NFT-Marketplace/master/assets/example.json",
  //       creators: [{
  //         address: root.publicKey,
  //         verified: true,
  //         share: 100,
  //       }],
  //       collection: {
  //         verified: false,
  //         key: root.publicKey,
  //       },
  //       name: "Updated NFT 1",
  //       sellerFeeBasisPoints: 0,
  //       uses: null,
  //     },
  //     false,
  //     true,
  //   ).accounts({
  //       metadataAccount,
  //       updateAuthority: root.publicKey,
  //       tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
  //     }).signers([root]).rpc().catch((err) => {
  //       console.log(err);
  //     });

  //     console.log('Metadata account updated: ', updateMetadataAccountTx);
  //     await new Promise(f => setTimeout(f, 100));

  //     const metadataInfo = await Metadata.fromAccountAddress(connection, metadataAccount);
  //     console.log(`Metadata by owner:`, metadataInfo.data);
  // });

  // it('Set Collection Size', async () => {
  //   collectionAuthorityRecord = findProgramAddressSync(
  //     [
  //       Buffer.from('metadata'),
  //       TOKEN_METADATA_PROGRAM_ID.toBuffer(),
  //       mint.toBuffer(),
  //       Buffer.from('collection_authority'),
  //       root.publicKey.toBuffer()
  //     ],
  //     TOKEN_METADATA_PROGRAM_ID,
  //   )[0];
  //   console.log('collectionAuthorityRecord account: ', collectionAuthorityRecord.toBase58());

  //   const setCollectionSizeTx = await program.methods.setCollectionSize(new anchor.BN(3)).accounts({
  //     collectionMetadataAccount: metadataAccount,
  //     collectionAuthority: root.publicKey,
  //     collectionMint: mint,
  //     collectionAuthorityRecord: collectionAuthorityRecord,
  //     tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
  //   }).signers([root])
  //   .rpc(
  // ).catch((err) => {
  //     console.log(err);
  //   });
  //   console.log('Set collection size done: ', setCollectionSizeTx);

  //   const metadataInfo = await Metadata.fromAccountAddress(connection, metadataAccount);
  //   console.log(`Metadata by owner:`, metadataInfo.data);
  // });

  it('Create second user', async () => {
    // Create user 2 account keypair
    user2 = anchor.web3.Keypair.generate();
    console.log('User 2 created: ', user2.publicKey.toBase58());

    // Airdrop to user 2
    const airdropSignature = await connection.requestAirdrop(
      user2.publicKey,
      LAMPORTS_PER_SOL,
    );
    
    await connection.confirmTransaction(airdropSignature);

    // Create user 2 mint
    user2Mint = await createMint(
      connection,
      user2,
      user2.publicKey, 
      user2.publicKey,
      0,
    );
    console.log('User 2 mint created: ', user2Mint.toBase58());

    // Create user 2 ATA
    user2ATA = await getOrCreateAssociatedTokenAccount(
      connection,
      user2,
      user2Mint,
      user2.publicKey,
    );
    console.log('User 2 ATA created: ', user2ATA.address.toBase58());

    // Mint to user 2 ATA
    const mintToTx = await mintTo(
      connection,
      user2,
      user2Mint,
      user2ATA.address,
      user2.publicKey,
      1
    ).catch((err) => {
      console.log('Error: ', err);
    });

    console.log('Minting second NFT done: ', mintToTx);
    await new Promise(f => setTimeout(f, 100));

    // Get user 2 ATA balance
    const user2ATABalance = await getAccount(connection, user2ATA.address);
    console.log('User 2 ATA balance: ', Number(user2ATABalance.amount));

    // Create user 2 metadata account
    user2MetadataAccount = findProgramAddressSync(
      [
        Buffer.from('metadata'),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        user2Mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];
    console.log('User 2 metadata account: ', user2MetadataAccount.toBase58());

    const user2MetadataAccountTx = await program.methods.createMetadataAccount([
      {
        address: user2.publicKey,
        verified: true,
        share: 100,
      }
    ],
      "Non-fungible token 2",
      "NFT",
      "https://raw.githubusercontent.com/Coding-and-Crypto/Solana-NFT-Marketplace/master/assets/example.json",
      {
        verified: false,
        key: mint,
      },
      null
    ).accounts({
        metadataAccount: user2MetadataAccount,
        mint: user2Mint,
        mintAuthority: user2.publicKey,
        payer: user2.publicKey,
        updateAuthority: user2.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      }).signers([user2]).rpc().catch((err) => {
        console.log('Error: ', err);
      });

      console.log('User 2 metadata account created: ', user2MetadataAccountTx);
      await new Promise(f => setTimeout(f, 100));

      const user2MetadataInfo = await Metadata.fromAccountAddress(connection, user2MetadataAccount);
      console.log(`User 2 metadata by owner:`, user2MetadataInfo);

      const collectionMetadataInfo = await Metadata.fromAccountAddress(connection, metadataAccount);
      console.log(`Collection metadata metadata by owner:`, collectionMetadataInfo);

      // Set master edition
      nft2MasterEditionAccount = findProgramAddressSync(
        [
          Buffer.from('metadata'),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          user2Mint.toBuffer(),
          Buffer.from('edition'),
        ],
        TOKEN_METADATA_PROGRAM_ID,
      )[0];
      console.log('Master edition account: ', nft2MasterEditionAccount.toBase58());
  
      const masterEditionAccountTx = await program.methods.createMasterEditionAccount(new anchor.BN(3)).accounts({
        masterEditionAccount: nft2MasterEditionAccount,
        metadataAccount: user2MetadataAccount, 
        mint: user2Mint,
        mintAuthority: user2.publicKey,
        payer: user2.publicKey,
        updateAuthority: user2.publicKey,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      }).signers([user2]).rpc();
      console.log('Master edition account created: ', masterEditionAccountTx);
      await new Promise(f => setTimeout(f, 100));
  });

  it("Verify sized collection of user 2 NFT", async () => {
      collectionAuthorityRecord = findProgramAddressSync(
      [
        Buffer.from('metadata'),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
        Buffer.from('collection_authority'),
        root.publicKey.toBuffer()
      ],
      TOKEN_METADATA_PROGRAM_ID,
    )[0];
    console.log('collectionAuthorityRecord account: ', collectionAuthorityRecord.toBase58());
    const verifyTransactionTx = await program.methods.verifySizedCollection().accounts({    
      metadataAccount: user2MetadataAccount,
      collectionAuthority: root.publicKey,
      payer: root.publicKey,
      collectionMint: mint,
      collectionMetadataAccount: metadataAccount,
      collectionMasterEditionAccount: masterEditionAccount,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      // collectionAuthorityRecord: null,
    }).signers([root]).rpc().catch((err) => {
      console.log('Error: ', err);
    });

    console.log('Verify collection done: ', verifyTransactionTx);

    const collectionMetadataInfo = await Metadata.fromAccountAddress(connection, metadataAccount);
    console.log(`Collection metadata metadata by owner:`, collectionMetadataInfo);

    const user2MetadataInfo = await Metadata.fromAccountAddress(connection, user2MetadataAccount);
    console.log(`User 2 metadata by owner:`, user2MetadataInfo);

  });

});
