use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DataV2 {
    /// The name of the asset
    pub name: String,
    /// The symbol for the asset
    pub symbol: String,
    /// URI pointing to JSON representing the asset
    pub uri: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: u16,
    /// Array of creators, optional
    pub creators: Option<Vec<Creator>>,
    /// Collection
    pub collection: Option<Collection>,
    /// Uses
    pub uses: Option<Uses>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    // In percentages, NOT basis points ;) Watch out!
    pub share: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Collection {
    pub verified: bool,
    pub key: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Uses {
    // 17 bytes + Option byte
    pub use_method: UseMethod, //1
    pub remaining: u64,        //8
    pub total: u64,            //8
}

// #[account]
// pub struct CollectionAuthorityRecord {
//     pub key: mpl_token_metadata::state::Key,
//     pub bump: u8,
//     pub update_authority: Option<Pubkey>,
// }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum CollectionDetails {
    V1 { size: u64 },
}