use std::collections::{HashMap, HashSet};

use alloy::primitives::U256;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum ChainId {
    ////////////////////////////////////////////////////////////////
    //                          MAINNETS
    ////////////////////////////////////////////////////////////////
    Ethereum = 1,
    Optimism = 10,
    Bsc = 56,
    Gnosis = 100,
    Polygon = 137,
    Fantom = 250,
    Arbitrum = 42161,
    Avalanche = 43114,
    Blast = 81457,
    Base = 1287,
    Sepolia = 11155111,
    ArbitrumNova = 421611,
    AvalancheFuji = 43113,

    ////////////////////////////////////////////////////////////////
    //                          TESTNETS
    ////////////////////////////////////////////////////////////////
    BerachainArtio = 80085,
}

pub fn AORI_V2_SINGLE_CHAIN_ZONE_ADDRESSES() -> HashMap<ChainId, HashSet<String>> {
    HashMap::from([
        (
            ChainId::Optimism,
            HashSet::from(["0xcc1A0DA89593441571f35Dd99a0aC1856d3F1FB5".to_string()]),
        ),
        (
            ChainId::Polygon,
            HashSet::from(["0xcc1A0DA89593441571f35Dd99a0aC1856d3F1FB5".to_string()]),
        ),
        (ChainId::Blast, HashSet::from(["0xcc1A0DA89593441571f35Dd99a0aC1856d3F1FB5".to_string()])),
        (ChainId::Base, HashSet::from(["0xcc1A0DA89593441571f35Dd99a0aC1856d3F1FB5".to_string()])),
        (
            ChainId::Arbitrum,
            HashSet::from([
                "0xcc1A0DA89593441571f35Dd99a0aC1856d3F1FB5".to_string(),
                "0x6A979916234013AbA003d906e4e7136496B90AA6".to_string(),
            ]),
        ),
        (
            ChainId::Sepolia,
            HashSet::from([
                "0xcc1A0DA89593441571f35Dd99a0aC1856d3F1FB5".to_string(),
                "0x6A979916234013AbA003d906e4e7136496B90AA6".to_string(),
            ]),
        ),
        (
            ChainId::BerachainArtio,
            HashSet::from(["0xcc1A0DA89593441571f35Dd99a0aC1856d3F1FB5".to_string()]),
        ),
    ])
}

pub fn SUPPORTED_AORI_CHAINS() -> HashSet<U256> {
    HashSet::from_iter(
        AORI_V2_SINGLE_CHAIN_ZONE_ADDRESSES().keys().into_iter().map(|x| U256::from(*x as u64)),
    )
}

pub const DEFAULT_ZONE: &str = "0xeA2b4e7F02b859305093f9F4778a19D66CA176d5";
pub const DEFAULT_ZONEHASH: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";
pub const SEATS_NFT_ADDRESS: &str = "0xD539e71371414F027Af025fd1EfFb6e11b5C902A";
pub const SEATS_DAO_ADDRESS: &str = "0x6E0Fd80bA37EC02855E4A8D7574f685984d83a5E";
pub const DEFAULT_SEAT_ID: i32 = 0;
pub const DEFAULT_SEAT_SCORE: i32 = 1;
pub const DEFAULT_SEAT_HOLDER: &str = "0x2EDEB6E06E81020F48d930FA7444a592ebE9FaB6";
