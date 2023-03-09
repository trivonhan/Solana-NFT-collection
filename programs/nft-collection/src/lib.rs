use anchor_lang::prelude::*;
use std::convert::From;

pub mod context;
pub mod state;
pub mod constant;

use crate::{
    context::*,
    state::*,
};

use mpl_token_metadata::{
    instruction as mpl_instruction,
    ID as TOKEN_METADATA_ID,
    state as mpl_token_state,
};

use anchor_lang::solana_program::{
    program::invoke,
};

declare_id!("7NnjQxQqB4Et5qQQVGDyHYKzCy2iTH24yMb7aB2qV3c6");

#[program]
pub mod nft_collection {
    use super::*;

    pub fn create_metadata_account(
        ctx: Context<CreateMetadataAccountsContext>, 
        creators: Vec<Creator>, 
        name: String, 
        symbol: String, 
        uri: String,
        collection: Option<Collection>,
        size: Option<u64>,
    ) -> Result<()> {
        let metadata_account = &ctx.accounts.metadata_account;
        let mint_authority = &ctx.accounts.mint_authority;
        let payer = &ctx.accounts.payer;
        let update_authority = &ctx.accounts.update_authority;
        let mint = &ctx.accounts.mint;
        let system_program = &ctx.accounts.system_program;
        let rent = &ctx.accounts.rent;
        let token_metadata_program = &ctx.accounts.token_metadata_program;

        let mut token_creators: Vec<mpl_token_metadata::state::Creator> = Vec::new();
        let token_collection : Option<mpl_token_metadata::state::Collection>;
        let token_collection_details : Option<mpl_token_metadata::state::CollectionDetails>;

        for creator in creators.iter() {
            token_creators.push(
                mpl_token_metadata::state::Creator {
                    address: creator.address,
                    verified: creator.verified,
                    share: creator.share,
                }
            )
        }

        if let Some(collection) = collection {
            token_collection = Some(
                mpl_token_metadata::state::Collection {
                    key: collection.key,
                    verified: collection.verified,
                }
            )
        } else {
            token_collection = None;
        }

        if let Some(size) = size {
            token_collection_details = Some(
                mpl_token_metadata::state::CollectionDetails::V1 {
                    size,
                }
            )
        }
        else {
            token_collection_details = None;
        }

        msg!("Collection: {:?}", token_collection);

        let instruction = mpl_instruction::create_metadata_accounts_v3(
            TOKEN_METADATA_ID, 
            metadata_account.key(), 
            mint.key(), 
            mint_authority.key(), 
            payer.key(), 
            update_authority.key(), 
            name, 
            symbol, 
            uri, 
            Some(token_creators), 
            0, 
            true, 
            true, 
            token_collection, 
            None,
            token_collection_details,
        );
        msg!("DEBUG: create metadata {:?}", instruction);

        invoke(
            &instruction, 
            &[
                metadata_account.to_account_info(),
                mint.to_account_info(),
                mint_authority.to_account_info(),
                payer.to_account_info(),
                update_authority.to_account_info(),
                system_program.to_account_info(),
                rent.to_account_info(),
                token_metadata_program.to_account_info(),
            ])
        .expect("CPI failed");
        Ok(())
    }

    pub fn create_master_edition_account(ctx: Context<CreateMasterEditionAccountContext>, max_supply: u64) -> Result<()> {
        let master_edition_account = &ctx.accounts.master_edition_account;
        let metadata_account = &ctx.accounts.metadata_account;
        let mint = &ctx.accounts.mint;
        let mint_authority = &ctx.accounts.mint_authority;
        let payer = &ctx.accounts.payer;
        let update_authority = &ctx.accounts.update_authority;
        let system_program = &ctx.accounts.system_program;
        let rent = &ctx.accounts.rent;
        let token_metadata_program = &ctx.accounts.token_metadata_program;
        let token_program = &ctx.accounts.token_program;

        let instruction = mpl_instruction::create_master_edition_v3(
            TOKEN_METADATA_ID, 
            master_edition_account.key(), 
            mint.key(), 
            update_authority.key(), 
            mint_authority.key(), 
            metadata_account.key(), 
            payer.key(), 
            Some(max_supply),
        );

        msg!("DEBUG: Create master edition instruction {:?}", instruction);

        invoke(&instruction, &[
            metadata_account.to_account_info(),
            mint.to_account_info(),
            mint_authority.to_account_info(), 
            payer.to_account_info(),
            update_authority.to_account_info(),
            system_program.to_account_info(),
            rent.to_account_info(),
            token_metadata_program.to_account_info(),
            master_edition_account.to_account_info(),
            token_program.to_account_info(),
        ])
        .expect("CPI failed");

        Ok(())
    }

    pub fn update_metadata_account(
        ctx: Context<UpdateMetadataAccountContext>, 
        new_update_authority: Option<Pubkey>,
        data: Option<DataV2>, 
        primary_sale_happened: Option<bool>, 
        is_mutable: Option<bool>
    ) -> Result<()> {
        let metadata_account = &ctx.accounts.metadata_account;
        let update_authority = &ctx.accounts.update_authority;
        let token_metadata_program = &ctx.accounts.token_metadata_program;
        let update_data: Option<mpl_token_metadata::state::DataV2>;

        if let Some(data_record) = data {
            msg!("Update data: {:?}", data_record.clone());
            update_data = Some(data_record.into());
        } else {
            update_data = None;
        }

        let instruction = mpl_instruction::update_metadata_accounts_v2(
            TOKEN_METADATA_ID, 
            metadata_account.key(), 
            update_authority.key(), 
            new_update_authority, 
            update_data, 
            primary_sale_happened, 
            is_mutable
        );

        invoke(&instruction, &[
            metadata_account.to_account_info(),
            update_authority.to_account_info(),
            token_metadata_program.to_account_info(),
        ]).expect("CPI failed");

        Ok(())
    }

    pub fn set_collection_size(ctx: Context<SetCollectionSizeContext>, size: u64) -> Result<()> {
        let collection_metadata_account = &ctx.accounts.collection_metadata_account;
        let collection_authority = &ctx.accounts.collection_authority; 
        let collection_mint = &ctx.accounts.collection_mint;
        let collection_authority_record = &ctx.accounts.collection_authority_record;
        let token_metadata_program = &ctx.accounts.token_metadata_program;

        msg!("DEBUG: collection_authority {:?}", collection_authority);

        let instruction = mpl_instruction::set_collection_size(
            TOKEN_METADATA_ID, 
            collection_metadata_account.key(), 
            collection_authority.key(), 
            collection_mint.key(), 
            Some(collection_authority_record.key()), 
            size
        );
        
        invoke(&instruction, &[
            collection_metadata_account.to_account_info(),
            collection_authority.to_account_info(),
            collection_mint.to_account_info(),
            collection_authority.to_account_info(),
            token_metadata_program.to_account_info(),
            collection_authority_record.to_account_info(),
        ]).expect("CPI failed");

        Ok(())
    }

    // pub fn approve_collection_authority(ctx: Context<InitCollectionRecordAuthorityContext>) -> Result<()> {
    //     let collection_authority_record = &ctx.accounts.collection_authority_record;
    //     let new_collection_authority = &ctx.accounts.new_collection_authority;
    //     let payer = &ctx.accounts.payer;
    //     let metadata_account = &ctx.accounts.metadata_account;
    //     let system_program = &ctx.accounts.system_program;
    //     let rent = &ctx.accounts.rent;
    //     let token_meta_data_program = &ctx.accounts.token_metadata_program;

    //     let instruction = mpl_instruction::approve_collection_authority(
    //         TOKEN_METADATA_ID, 
    //         collection_authority_record.key(), 
    //         new_collection_authority.key(), 
    //         payer.key(), 
    //         metadata_account.key()
    //     );

    //     Ok(())
    // }
}

impl From<Creator> for mpl_token_state::Creator {
    fn from(value: Creator) -> Self {
        mpl_token_state::Creator {
            address: value.address,
            verified: value.verified,
            share: value.share,
        }
    }
}

impl From<Collection> for mpl_token_state::Collection {
    fn from(value: Collection) -> Self {
        mpl_token_state::Collection {
            verified: value.verified,
            key: value.key,
        }
    }
}

impl From<UseMethod> for mpl_token_state::UseMethod {
    fn from(value: UseMethod) -> Self {
        match value {
            UseMethod::Burn => mpl_token_state::UseMethod::Burn,
            UseMethod::Multiple => mpl_token_state::UseMethod::Multiple,
            UseMethod::Single => mpl_token_state::UseMethod::Single,
        }
    }
}

impl From<Uses> for mpl_token_state::Uses {
    fn from(value: Uses) -> Self {
        mpl_token_state::Uses {
            use_method: mpl_token_state::UseMethod::from(value.use_method),
            remaining: value.remaining,
            total: value.total,
        }
    }
}

impl From<CollectionDetails> for mpl_token_state::CollectionDetails {
    fn from(value: CollectionDetails) -> Self {
        match value {
            CollectionDetails::V1 { size } => mpl_token_state::CollectionDetails::V1 { size },
        }
    }
}

impl From<DataV2> for mpl_token_state::DataV2 {
    fn from(value: DataV2) -> Self {
        let creators: Option<Vec<mpl_token_state::Creator>>;
        let collection: Option<mpl_token_state::Collection>;
        let uses: Option<mpl_token_state::Uses>;

        if let Some(creators_vec) = value.creators {
            creators = Some(creators_vec.into_iter().map(|creator| creator.into()).collect());
        } else {
            creators = None;
        }

        if let Some(collection_value) = value.collection {
            collection = Some(collection_value.into());
        } else {
            collection = None;
        }

        if let Some(uses_value) = value.uses {
            uses = Some(uses_value.into());
        } else {
            uses = None;
        }

        mpl_token_state::DataV2  {
            name: value.name,
            symbol: value.symbol,
            uri: value.uri,
            seller_fee_basis_points: value.seller_fee_basis_points,
            creators,
            collection,
            uses,
        }
    }
}