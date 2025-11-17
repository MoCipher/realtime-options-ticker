// This file defines the data structures used to represent options data. 
// It exports structs like OptionData and OptionQuote, along with their associated methods.

pub struct OptionData {
    pub symbol: String,
    pub expiration_date: String,
    pub strike_price: f64,
    pub option_type: String,
    pub last_price: f64,
    pub bid: f64,
    pub ask: f64,
    pub volume: u64,
    pub open_interest: u64,
}

impl OptionData {
    pub fn new(
        symbol: String,
        expiration_date: String,
        strike_price: f64,
        option_type: String,
        last_price: f64,
        bid: f64,
        ask: f64,
        volume: u64,
        open_interest: u64,
    ) -> Self {
        OptionData {
            symbol,
            expiration_date,
            strike_price,
            option_type,
            last_price,
            bid,
            ask,
            volume,
            open_interest,
        }
    }
}

pub struct OptionQuote {
    pub option_data: OptionData,
    pub timestamp: String,
}

impl OptionQuote {
    pub fn new(option_data: OptionData, timestamp: String) -> Self {
        OptionQuote { option_data, timestamp }
    }
}