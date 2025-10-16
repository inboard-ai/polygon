//! Ticker data types and decoders
use decoder::{Result, Value, decode};

use crate::schema::ticker::*;

impl CompanyAddress {
    /// Decode a CompanyAddress from a Value
    pub fn decode(value: Value) -> Result<Self> {
        let mut addr = decode::map(value)?;

        Ok(CompanyAddress {
            address1: addr.optional("address1", decode::string)?,
            address2: addr.optional("address2", decode::string)?,
            city: addr.optional("city", decode::string)?,
            state: addr.optional("state", decode::string)?,
            country: addr.optional("country", decode::string)?,
            postal_code: addr.optional("postal_code", decode::string)?,
        })
    }
}

impl Branding {
    /// Decode Branding from a Value
    pub fn decode(value: Value) -> Result<Self> {
        let mut brand = decode::map(value)?;

        Ok(Branding {
            icon_url: brand.optional("icon_url", decode::string)?,
            logo_url: brand.optional("logo_url", decode::string)?,
            accent_color: brand.optional("accent_color", decode::string)?,
            light_color: brand.optional("light_color", decode::string)?,
            dark_color: brand.optional("dark_color", decode::string)?,
        })
    }
}

impl Ticker {
    /// Decode a Ticker from a Value
    pub fn decode(value: Value) -> Result<Self> {
        let mut ticker = decode::map(value)?;

        Ok(Ticker {
            active: ticker.optional("active", decode::bool)?,
            cik: ticker.optional("cik", decode::string)?,
            composite_figi: ticker.optional("composite_figi", decode::string)?,
            currency_name: ticker.optional("currency_name", decode::string)?,
            currency_symbol: ticker.optional("currency_symbol", decode::string)?,
            base_currency_symbol: ticker.optional("base_currency_symbol", decode::string)?,
            base_currency_name: ticker.optional("base_currency_name", decode::string)?,
            delisted_utc: ticker.optional("delisted_utc", decode::string)?,
            last_updated_utc: ticker.optional("last_updated_utc", decode::string)?,
            locale: ticker.optional("locale", decode::string)?,
            market: ticker.optional("market", decode::string)?,
            name: ticker.optional("name", decode::string)?,
            primary_exchange: ticker.optional("primary_exchange", decode::string)?,
            share_class_figi: ticker.optional("share_class_figi", decode::string)?,
            ticker: ticker.optional("ticker", decode::string)?,
            type_: ticker.optional("type", decode::string)?,
            source_feed: ticker.optional("source_feed", decode::string)?,
        })
    }
}
