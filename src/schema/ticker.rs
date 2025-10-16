//! Ticker data types and decoders

/// Company address data
#[derive(Debug, Clone)]
pub struct CompanyAddress {
    /// Address line 1
    pub address1: Option<String>,
    /// Address line 2
    pub address2: Option<String>,
    /// City
    pub city: Option<String>,
    /// State
    pub state: Option<String>,
    /// Country
    pub country: Option<String>,
    /// Postal code
    pub postal_code: Option<String>,
}

/// Branding data for a ticker
#[derive(Debug, Clone)]
pub struct Branding {
    /// Icon URL
    pub icon_url: Option<String>,
    /// Logo URL  
    pub logo_url: Option<String>,
    /// Accent color
    pub accent_color: Option<String>,
    /// Light color
    pub light_color: Option<String>,
    /// Dark color
    pub dark_color: Option<String>,
}

/// Basic ticker information
#[derive(Debug, Clone)]
pub struct Ticker {
    /// Whether the ticker is actively traded
    pub active: Option<bool>,
    /// Central Index Key (CIK)
    pub cik: Option<String>,
    /// Composite FIGI
    pub composite_figi: Option<String>,
    /// Currency name
    pub currency_name: Option<String>,
    /// Currency symbol
    pub currency_symbol: Option<String>,
    /// Base currency symbol
    pub base_currency_symbol: Option<String>,
    /// Base currency name
    pub base_currency_name: Option<String>,
    /// Delisted UTC timestamp
    pub delisted_utc: Option<String>,
    /// Last updated UTC timestamp
    pub last_updated_utc: Option<String>,
    /// Locale
    pub locale: Option<String>,
    /// Market
    pub market: Option<String>,
    /// Ticker name
    pub name: Option<String>,
    /// Primary exchange
    pub primary_exchange: Option<String>,
    /// Share class FIGI
    pub share_class_figi: Option<String>,
    /// Ticker symbol
    pub ticker: Option<String>,
    /// Ticker type
    pub type_: Option<String>,
    /// Source feed (optional)
    pub source_feed: Option<String>,
}

#[cfg(feature = "decoder")]
impl crate::query::Decodable for Ticker {}

#[cfg(feature = "decoder")]
impl crate::query::Decodable for Vec<Ticker> {}

#[cfg(feature = "decoder")]
impl crate::query::Decodable for Vec<String> {}
