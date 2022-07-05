use serde::Serialize;

#[derive(Serialize, Default, PartialEq, Clone)]
pub struct BaseCurrency {
    name: String,
    symbol: String, // 2-6 characters long
    decimals: u32,
}

impl BaseCurrency {
    pub fn eth() -> Self {
        Self {
            name: String::from("Ether"),
            symbol: String::from("ETH"),
            decimals: 18,
        }
    }

    pub fn avax() -> Self {
        Self {
            name: String::from("AVAX"),
            symbol: String::from("AVAX"),
            decimals: 18,
        }
    }
}
