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

impl From<u64> for ChainId {
    fn from(value: u64) -> Self {
        match value {
            1 => ChainId::Ethereum,
            10 => ChainId::Optimism,
            56 => ChainId::Bsc,
            100 => ChainId::Gnosis,
            137 => ChainId::Polygon,
            250 => ChainId::Fantom,
            42161 => ChainId::Arbitrum,
            43114 => ChainId::Avalanche,
            81457 => ChainId::Blast,
            1287 => ChainId::Base,
            11155111 => ChainId::Sepolia,
            421611 => ChainId::ArbitrumNova,
            43113 => ChainId::AvalancheFuji,
            80085 => ChainId::BerachainArtio,
            _ => panic!("Invalid chain id"),
        }
    }
}


pub fn AORI_V2_SINGLE_CHAIN_ZONE_ADDRESSES() -> HashMap<ChainId, HashSet<String>> {
    HashMap::from([
        (ChainId::Optimism, HashSet::from(["0x0AD86842EadEe5b484E31db60716EB6867B46e21".to_string()])),
        (ChainId::Polygon, HashSet::from(["0x0AD86842EadEe5b484E31db60716EB6867B46e21".to_string()])),
        (ChainId::Blast, HashSet::from(["0x0AD86842EadEe5b484E31db60716EB6867B46e21".to_string()])),
        (ChainId::Base, HashSet::from(["0x0AD86842EadEe5b484E31db60716EB6867B46e21".to_string()])),
        (ChainId::Arbitrum, HashSet::from(["0x0AD86842EadEe5b484E31db60716EB6867B46e21".to_string()])),
        (ChainId::Sepolia, HashSet::from(["0x0AD86842EadEe5b484E31db60716EB6867B46e21".to_string()])),
    ])
}

pub fn SUPPORTED_AORI_CHAINS() -> HashSet<U256> {
    HashSet::from_iter(
        AORI_V2_SINGLE_CHAIN_ZONE_ADDRESSES().keys().into_iter().map(|x| U256::from(*x as u64)),
    )
}

pub const AORI_HTTP_RFQ_URL: &str = "https://rfq.aori.io";
pub const AORI_WS_RFQ_URL: &str = "wss://rfq.aori.io";

pub const DEFAULT_ZONE: &str = "0x0AD86842EadEe5b484E31db60716EB6867B46e21";
pub const DEFAULT_ZONEHASH: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";
pub const SEATS_NFT_ADDRESS: &str = "0xD539e71371414F027Af025fd1EfFb6e11b5C902A";
pub const SEATS_DAO_ADDRESS: &str = "0x6E0Fd80bA37EC02855E4A8D7574f685984d83a5E";
pub const DEFAULT_SEAT_ID: i32 = 0;
pub const DEFAULT_SEAT_SCORE: i32 = 1;
pub const DEFAULT_SEAT_HOLDER: &str = "0x2EDEB6E06E81020F48d930FA7444a592ebE9FaB6";
