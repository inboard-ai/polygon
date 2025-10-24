//! Decoded financials endpoints - returns typed data instead of JSON strings

use crate::client::Polygon;
use crate::query::Query;
use crate::request::Request;
use crate::rest::raw;
use crate::schema::financials::*;
use decoder::decode::{f64, map, sequence, string};

/// Get balance sheet data for public companies (decoded)
pub fn balance_sheets<'a, Client: Request>(
    client: &'a Polygon<Client>,
) -> Query<'a, Client, Vec<FinancialBalanceSheet>> {
    raw::financials::balance_sheets(client).with_decoder(_balance_sheets)
}

/// Get cash flow statement data for public companies (decoded)
pub fn cash_flow_statements<'a, Client: Request>(
    client: &'a Polygon<Client>,
) -> Query<'a, Client, Vec<FinancialCashFlowStatement>> {
    raw::financials::cash_flow_statements(client).with_decoder(_cash_flow_statements)
}

/// Get income statement data for public companies (decoded)
pub fn income_statements<'a, Client: Request>(
    client: &'a Polygon<Client>,
) -> Query<'a, Client, Vec<FinancialIncomeStatement>> {
    raw::financials::income_statements(client).with_decoder(_income_statements)
}

/// Get financial ratios data for public companies (decoded)
pub fn ratios<'a, Client: Request>(
    client: &'a Polygon<Client>,
) -> Query<'a, Client, Vec<FinancialRatio>> {
    raw::financials::ratios(client).with_decoder(_ratios)
}

// Internal decoder functions
fn _balance_sheets(value: decoder::Value) -> decoder::Result<Vec<FinancialBalanceSheet>> {
    let mut response = map(value)?;
    response.required("results", sequence(_decode_balance_sheet))
}

fn _cash_flow_statements(
    value: decoder::Value,
) -> decoder::Result<Vec<FinancialCashFlowStatement>> {
    let mut response = map(value)?;
    response.required("results", sequence(_decode_cash_flow_statement))
}

fn _income_statements(value: decoder::Value) -> decoder::Result<Vec<FinancialIncomeStatement>> {
    let mut response = map(value)?;
    response.required("results", sequence(_decode_income_statement))
}

fn _ratios(value: decoder::Value) -> decoder::Result<Vec<FinancialRatio>> {
    let mut response = map(value)?;
    response.required("results", sequence(_decode_ratio))
}

fn _decode_balance_sheet(value: decoder::Value) -> decoder::Result<FinancialBalanceSheet> {
    let mut obj = map(value)?;

    Ok(FinancialBalanceSheet {
        accounts_payable: obj.optional("accounts_payable", f64)?,
        accrued_and_other_current_liabilities: obj
            .optional("accrued_and_other_current_liabilities", f64)?,
        accumulated_other_comprehensive_income: obj
            .optional("accumulated_other_comprehensive_income", f64)?,
        additional_paid_in_capital: obj.optional("additional_paid_in_capital", f64)?,
        cash_and_equivalents: obj.optional("cash_and_equivalents", f64)?,
        cik: obj.optional("cik", string)?,
        commitments_and_contingencies: obj.optional("commitments_and_contingencies", f64)?,
        common_stock: obj.optional("common_stock", f64)?,
        debt_current: obj.optional("debt_current", f64)?,
        deferred_revenue_current: obj.optional("deferred_revenue_current", f64)?,
        filing_date: obj.optional("filing_date", string)?,
        fiscal_quarter: obj.optional("fiscal_quarter", f64)?,
        fiscal_year: obj.optional("fiscal_year", f64)?,
        goodwill: obj.optional("goodwill", f64)?,
        intangible_assets_net: obj.optional("intangible_assets_net", f64)?,
        inventories: obj.optional("inventories", f64)?,
        long_term_debt_and_capital_lease_obligations: obj
            .optional("long_term_debt_and_capital_lease_obligations", f64)?,
        noncontrolling_interest: obj.optional("noncontrolling_interest", f64)?,
        other_assets: obj.optional("other_assets", f64)?,
        other_current_assets: obj.optional("other_current_assets", f64)?,
        other_equity: obj.optional("other_equity", f64)?,
        other_noncurrent_liabilities: obj.optional("other_noncurrent_liabilities", f64)?,
        period_end: obj.optional("period_end", string)?,
        preferred_stock: obj.optional("preferred_stock", f64)?,
        property_plant_equipment_net: obj.optional("property_plant_equipment_net", f64)?,
        receivables: obj.optional("receivables", f64)?,
        retained_earnings_deficit: obj.optional("retained_earnings_deficit", f64)?,
        short_term_investments: obj.optional("short_term_investments", f64)?,
        tickers: obj.optional("tickers", sequence(string))?,
        timeframe: obj.optional("timeframe", string)?,
        total_assets: obj.optional("total_assets", f64)?,
        total_current_assets: obj.optional("total_current_assets", f64)?,
        total_current_liabilities: obj.optional("total_current_liabilities", f64)?,
        total_equity: obj.optional("total_equity", f64)?,
        total_equity_attributable_to_parent: obj
            .optional("total_equity_attributable_to_parent", f64)?,
        total_liabilities: obj.optional("total_liabilities", f64)?,
        total_liabilities_and_equity: obj.optional("total_liabilities_and_equity", f64)?,
        treasury_stock: obj.optional("treasury_stock", f64)?,
    })
}

fn _decode_cash_flow_statement(
    value: decoder::Value,
) -> decoder::Result<FinancialCashFlowStatement> {
    let mut obj = map(value)?;

    Ok(FinancialCashFlowStatement {
        cash_from_operating_activities_continuing_operations: obj
            .optional("cash_from_operating_activities_continuing_operations", f64)?,
        change_in_cash_and_equivalents: obj.optional("change_in_cash_and_equivalents", f64)?,
        change_in_other_operating_assets_and_liabilities_net: obj
            .optional("change_in_other_operating_assets_and_liabilities_net", f64)?,
        cik: obj.optional("cik", string)?,
        depreciation_depletion_and_amortization: obj
            .optional("depreciation_depletion_and_amortization", f64)?,
        dividends: obj.optional("dividends", f64)?,
        effect_of_currency_exchange_rate: obj.optional("effect_of_currency_exchange_rate", f64)?,
        filing_date: obj.optional("filing_date", string)?,
        fiscal_quarter: obj.optional("fiscal_quarter", f64)?,
        fiscal_year: obj.optional("fiscal_year", f64)?,
        income_loss_from_discontinued_operations: obj
            .optional("income_loss_from_discontinued_operations", f64)?,
        long_term_debt_issuances_repayments: obj
            .optional("long_term_debt_issuances_repayments", f64)?,
        net_cash_from_financing_activities: obj
            .optional("net_cash_from_financing_activities", f64)?,
        net_cash_from_financing_activities_continuing_operations: obj
            .optional("net_cash_from_financing_activities_continuing_operations", f64)?,
        net_cash_from_financing_activities_discontinued_operations: obj
            .optional("net_cash_from_financing_activities_discontinued_operations", f64)?,
        net_cash_from_investing_activities: obj
            .optional("net_cash_from_investing_activities", f64)?,
        net_cash_from_investing_activities_continuing_operations: obj
            .optional("net_cash_from_investing_activities_continuing_operations", f64)?,
        net_cash_from_investing_activities_discontinued_operations: obj
            .optional("net_cash_from_investing_activities_discontinued_operations", f64)?,
        net_cash_from_operating_activities: obj
            .optional("net_cash_from_operating_activities", f64)?,
        net_cash_from_operating_activities_discontinued_operations: obj
            .optional("net_cash_from_operating_activities_discontinued_operations", f64)?,
        net_income: obj.optional("net_income", f64)?,
        noncontrolling_interests: obj.optional("noncontrolling_interests", f64)?,
        other_cash_adjustments: obj.optional("other_cash_adjustments", f64)?,
        other_financing_activities: obj.optional("other_financing_activities", f64)?,
        other_investing_activities: obj.optional("other_investing_activities", f64)?,
        other_operating_activities: obj.optional("other_operating_activities", f64)?,
        period_end: obj.optional("period_end", string)?,
        purchase_of_property_plant_and_equipment: obj
            .optional("purchase_of_property_plant_and_equipment", f64)?,
        sale_of_property_plant_and_equipment: obj
            .optional("sale_of_property_plant_and_equipment", f64)?,
        short_term_debt_issuances_repayments: obj
            .optional("short_term_debt_issuances_repayments", f64)?,
        tickers: obj.optional("tickers", sequence(string))?,
        timeframe: obj.optional("timeframe", string)?,
    })
}

fn _decode_income_statement(value: decoder::Value) -> decoder::Result<FinancialIncomeStatement> {
    let mut obj = map(value)?;

    Ok(FinancialIncomeStatement {
        basic_earnings_per_share: obj.optional("basic_earnings_per_share", f64)?,
        basic_shares_outstanding: obj.optional("basic_shares_outstanding", f64)?,
        cik: obj.optional("cik", string)?,
        consolidated_net_income_loss: obj.optional("consolidated_net_income_loss", f64)?,
        cost_of_revenue: obj.optional("cost_of_revenue", f64)?,
        depreciation_depletion_amortization: obj
            .optional("depreciation_depletion_amortization", f64)?,
        diluted_earnings_per_share: obj.optional("diluted_earnings_per_share", f64)?,
        diluted_shares_outstanding: obj.optional("diluted_shares_outstanding", f64)?,
        discontinued_operations: obj.optional("discontinued_operations", f64)?,
        ebitda: obj.optional("ebitda", f64)?,
        equity_in_affiliates: obj.optional("equity_in_affiliates", f64)?,
        extraordinary_items: obj.optional("extraordinary_items", f64)?,
        filing_date: obj.optional("filing_date", string)?,
        fiscal_quarter: obj.optional("fiscal_quarter", f64)?,
        fiscal_year: obj.optional("fiscal_year", f64)?,
        gross_profit: obj.optional("gross_profit", f64)?,
        income_before_income_taxes: obj.optional("income_before_income_taxes", f64)?,
        income_taxes: obj.optional("income_taxes", f64)?,
        interest_expense: obj.optional("interest_expense", f64)?,
        interest_income: obj.optional("interest_income", f64)?,
        net_income_loss_attributable_common_shareholders: obj
            .optional("net_income_loss_attributable_common_shareholders", f64)?,
        noncontrolling_interest: obj.optional("noncontrolling_interest", f64)?,
        operating_income: obj.optional("operating_income", f64)?,
        other_income_expense: obj.optional("other_income_expense", f64)?,
        other_operating_expenses: obj.optional("other_operating_expenses", f64)?,
        period_end: obj.optional("period_end", string)?,
        preferred_stock_dividends_declared: obj
            .optional("preferred_stock_dividends_declared", f64)?,
        research_development: obj.optional("research_development", f64)?,
        revenue: obj.optional("revenue", f64)?,
        selling_general_administrative: obj.optional("selling_general_administrative", f64)?,
        tickers: obj.optional("tickers", sequence(string))?,
        timeframe: obj.optional("timeframe", string)?,
        total_operating_expenses: obj.optional("total_operating_expenses", f64)?,
        total_other_income_expense: obj.optional("total_other_income_expense", f64)?,
    })
}

fn _decode_ratio(value: decoder::Value) -> decoder::Result<FinancialRatio> {
    let mut obj = map(value)?;

    Ok(FinancialRatio {
        average_volume: obj.optional("average_volume", f64)?,
        cash: obj.optional("cash", f64)?,
        cik: obj.optional("cik", string)?,
        current: obj.optional("current", f64)?,
        date: obj.optional("date", string)?,
        debt_to_equity: obj.optional("debt_to_equity", f64)?,
        dividend_yield: obj.optional("dividend_yield", f64)?,
        earnings_per_share: obj.optional("earnings_per_share", f64)?,
        enterprise_value: obj.optional("enterprise_value", f64)?,
        ev_to_ebitda: obj.optional("ev_to_ebitda", f64)?,
        ev_to_sales: obj.optional("ev_to_sales", f64)?,
        free_cash_flow: obj.optional("free_cash_flow", f64)?,
        market_cap: obj.optional("market_cap", f64)?,
        price: obj.optional("price", f64)?,
        price_to_book: obj.optional("price_to_book", f64)?,
        price_to_cash_flow: obj.optional("price_to_cash_flow", f64)?,
        price_to_earnings: obj.optional("price_to_earnings", f64)?,
        price_to_free_cash_flow: obj.optional("price_to_free_cash_flow", f64)?,
        price_to_sales: obj.optional("price_to_sales", f64)?,
        quick: obj.optional("quick", f64)?,
        return_on_assets: obj.optional("return_on_assets", f64)?,
        return_on_equity: obj.optional("return_on_equity", f64)?,
        ticker: obj.optional("ticker", string)?,
    })
}
