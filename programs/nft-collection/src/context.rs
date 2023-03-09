use anchor_lang::prelude::*;

use crate::{
    constant::{
        METADATA_SEED,
    },
    state::*,
};

#[derive(Accounts)]
pub struct CreateMetadataAccountsContext<'info> {

    /// CHECK: Metadata account
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,

    /// CHECK: Mint account according to user
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    #[account(mut)]
    pub mint_authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub update_authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,

    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,

}

#[derive(Accounts)]
pub struct CreateMasterEditionAccountContext<'info> {
    
    /// CHECK: Master edition account
    #[account(mut)]
    pub master_edition_account: AccountInfo<'info>,

    /// CHECK: Metadata account
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,

    /// CHECK: Mint account according to user
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    #[account(mut)]
    pub mint_authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub update_authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,

    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,

    /// CHECK: Token program ID (default = TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA)
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetCollectionSizeContext<'info> {

    /// CHECK: Metadata account of the NFT (which is the first chosen for the collection )
    #[account(mut)]
    pub collection_metadata_account: AccountInfo<'info>,

    /// CHECK: Collection Update authority
    pub collection_authority: Signer<'info>,

    /// CHECK: Mint account of the NFT (which is the first chosen for the collection )
    #[account(mut)]
    pub collection_mint: AccountInfo<'info>,

    /// CHECK: PDA of the collection authority record
    #[account(mut)]
    pub collection_authority_record: AccountInfo<'info>,

    /// CHECK: Metaplex will check this (TOKEN METADATA PROGRAM ID)
    pub token_metadata_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateMetadataAccountContext<'info> {
    
    /// CHECK: Metadata account of the NFT (which is the first chosen for the collection )
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,

    /// CHECK: Collection Update authority
    pub update_authority: Signer<'info>,

    /// CHECK: Metaplex will check this (TOKEN METADATA PROGRAM ID)
    pub token_metadata_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct VerifyCollection<'info> {
    
    /// CHECK: Metadata of NFT in the collection need be verified collection
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,

    /// CHECK: Collection Update authority
    pub collection_authority: Signer<'info>,

    pub payer: Signer<'info>,

    /// CHECK: Mint of the Collection (mint of first NFT in collection)
    #[account(mut)]
    pub collection_mint: AccountInfo<'info>,

    /// CHECK: Metadata account of the Collection (metadata of first NFT in collection)
    #[account(mut)]
    pub collection_metadata_account: AccountInfo<'info>,

    /// CHECK: Master edition account of the Collection (master edition of first NFT in collection)
    #[account(mut)]
    pub collection_master_edition_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: Metaplex will check this (TOKEN METADATA PROGRAM ID)
    pub token_metadata_program: AccountInfo<'info>,
}

// #[derive(Accounts)]
// pub struct InitCollectionRecordAuthorityContext<'info> {

//     #[account(mut)]
//     pub collection_authority_record: AccountInfo<'info>,

//     #[account(mut)]
//     pub new_collection_authority: AccountInfo<'info>,

//     pub payer: Signer<'info>,

//     /// CHECK: Metadata account of the NFT (which is the first chosen for the collection )
//     #[account(mut)]
//     pub metadata_account: AccountInfo<'info>,

//     pub system_program: Program<'info, System>,

//     /// CHECK: Metaplex will check this (TOKEN METADATA PROGRAM ID)
//     pub token_metadata_program: AccountInfo<'info>,

//     pub rent: Sysvar<'info, Rent>,
// }