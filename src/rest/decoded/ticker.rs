//! Ticker data types and decoders
use decoder::decode::{bool, map, string};
use decoder::{Result, Value};

use crate::schema::ticker::*;

impl CompanyAddress {
    /// Decode a CompanyAddress from a Value
    pub fn decode(value: Value) -> Result<Self> {
        let mut addr = map(value)?;

        Ok(CompanyAddress {
            address1: addr.optional("address1", string)?,
            address2: addr.optional("address2", string)?,
            city: addr.optional("city", string)?,
            state: addr.optional("state", string)?,
            country: addr.optional("country", string)?,
            postal_code: addr.optional("postal_code", string)?,
        })
    }
}

impl Branding {
    /// Decode Branding from a Value
    pub fn decode(value: Value) -> Result<Self> {
        let mut brand = map(value)?;

        Ok(Branding {
            icon_url: brand.optional("icon_url", string)?,
            logo_url: brand.optional("logo_url", string)?,
            accent_color: brand.optional("accent_color", string)?,
            light_color: brand.optional("light_color", string)?,
            dark_color: brand.optional("dark_color", string)?,
        })
    }
}

impl Ticker {
    /// Decode a Ticker from a Value
    pub fn decode(value: Value) -> Result<Self> {
        let mut ticker = map(value)?;

        Ok(Ticker {
            active: ticker.optional("active", bool)?,
            cik: ticker.optional("cik", string)?,
            composite_figi: ticker.optional("composite_figi", string)?,
            currency_name: ticker.optional("currency_name", string)?,
            currency_symbol: ticker.optional("currency_symbol", string)?,
            base_currency_symbol: ticker.optional("base_currency_symbol", string)?,
            base_currency_name: ticker.optional("base_currency_name", string)?,
            delisted_utc: ticker.optional("delisted_utc", string)?,
            last_updated_utc: ticker.optional("last_updated_utc", string)?,
            locale: ticker.optional("locale", string)?,
            market: ticker.optional("market", string)?,
            name: ticker.optional("name", string)?,
            primary_exchange: ticker.optional("primary_exchange", string)?,
            share_class_figi: ticker.optional("share_class_figi", string)?,
            ticker: ticker.optional("ticker", string)?,
            type_: ticker.optional("type", string)?,
            source_feed: ticker.optional("source_feed", string)?,
        })
    }
}
