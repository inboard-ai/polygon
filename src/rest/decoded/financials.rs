//! Decoded financials endpoints - returns typed data instead of JSON strings

use crate::client::Polygon;
use crate::processor::Decoder;
use crate::request::Request;
use crate::request::financials::Financials;
use crate::response::financials::*;
use crate::rest::financials;

/// Get balance sheet data for public companies (decoded)
///
/// Returns a decoded request builder that will return typed `Vec<`[`BalanceSheet`]`>` data.
/// Use builder methods like `.ticker()`, `.filing_date()`, etc. to customize the request.
pub fn balance_sheets<'a, Client: Request>(
    client: &'a Polygon<Client>,
) -> Financials<'a, Client, Decoder<Vec<BalanceSheet>>> {
    financials::balance_sheets(client).decoded()
}

/// Get cash flow statement data for public companies (decoded)
///
/// Returns a decoded request builder that will return typed `Vec<`[`CashFlowStatement`]`>` data.
/// Use builder methods like `.ticker()`, `.filing_date()`, etc. to customize the request.
pub fn cash_flow_statements<'a, Client: Request>(
    client: &'a Polygon<Client>,
) -> Financials<'a, Client, Decoder<Vec<CashFlowStatement>>> {
    financials::cash_flow_statements(client).decoded()
}

/// Get income statement data for public companies (decoded)
///
/// Returns a decoded request builder that will return typed `Vec<`[`IncomeStatement`]`>` data.
/// Use builder methods like `.ticker()`, `.filing_date()`, etc. to customize the request.
pub fn income_statements<'a, Client: Request>(
    client: &'a Polygon<Client>,
) -> Financials<'a, Client, Decoder<Vec<IncomeStatement>>> {
    financials::income_statements(client).decoded()
}

/// Get financial ratios data for public companies (decoded)
///
/// Returns a decoded request builder that will return typed `Vec<`[`FinancialRatio`]`>` data.
/// Use builder methods like `.ticker()`, `.date()`, etc. to customize the request.
pub fn ratios<'a, Client: Request>(
    client: &'a Polygon<Client>,
) -> Financials<'a, Client, Decoder<Vec<FinancialRatio>>> {
    financials::ratios(client).decoded()
}
