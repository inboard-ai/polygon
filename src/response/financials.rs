//! Financial data types

/// Balance sheet data for a company
#[derive(Debug, Clone)]
pub struct BalanceSheet {
    /// Accounts payable
    pub accounts_payable: Option<f64>,
    /// Accrued and other current liabilities
    pub accrued_and_other_current_liabilities: Option<f64>,
    /// Accumulated other comprehensive income
    pub accumulated_other_comprehensive_income: Option<f64>,
    /// Additional paid-in capital
    pub additional_paid_in_capital: Option<f64>,
    /// Cash and cash equivalents
    pub cash_and_equivalents: Option<f64>,
    /// Central Index Key (CIK)
    pub cik: Option<String>,
    /// Commitments and contingencies
    pub commitments_and_contingencies: Option<f64>,
    /// Common stock value
    pub common_stock: Option<f64>,
    /// Current portion of debt
    pub debt_current: Option<f64>,
    /// Current deferred revenue
    pub deferred_revenue_current: Option<f64>,
    /// Filing date (YYYY-MM-DD)
    pub filing_date: Option<String>,
    /// Fiscal quarter (1-4)
    pub fiscal_quarter: Option<f64>,
    /// Fiscal year
    pub fiscal_year: Option<f64>,
    /// Goodwill
    pub goodwill: Option<f64>,
    /// Intangible assets, net
    pub intangible_assets_net: Option<f64>,
    /// Inventories
    pub inventories: Option<f64>,
    /// Long-term debt and capital lease obligations
    pub long_term_debt_and_capital_lease_obligations: Option<f64>,
    /// Noncontrolling interest
    pub noncontrolling_interest: Option<f64>,
    /// Other assets
    pub other_assets: Option<f64>,
    /// Other current assets
    pub other_current_assets: Option<f64>,
    /// Other equity
    pub other_equity: Option<f64>,
    /// Other noncurrent liabilities
    pub other_noncurrent_liabilities: Option<f64>,
    /// Period end date (YYYY-MM-DD)
    pub period_end: Option<String>,
    /// Preferred stock value
    pub preferred_stock: Option<f64>,
    /// Property, plant, and equipment, net
    pub property_plant_equipment_net: Option<f64>,
    /// Receivables
    pub receivables: Option<f64>,
    /// Retained earnings (accumulated deficit)
    pub retained_earnings_deficit: Option<f64>,
    /// Short-term investments
    pub short_term_investments: Option<f64>,
    /// List of ticker symbols
    pub tickers: Option<Vec<String>>,
    /// Timeframe (quarterly or annual)
    pub timeframe: Option<String>,
    /// Total assets
    pub total_assets: Option<f64>,
    /// Total current assets
    pub total_current_assets: Option<f64>,
    /// Total current liabilities
    pub total_current_liabilities: Option<f64>,
    /// Total equity
    pub total_equity: Option<f64>,
    /// Total equity attributable to parent
    pub total_equity_attributable_to_parent: Option<f64>,
    /// Total liabilities
    pub total_liabilities: Option<f64>,
    /// Total liabilities and equity
    pub total_liabilities_and_equity: Option<f64>,
    /// Treasury stock
    pub treasury_stock: Option<f64>,
}

/// Cash flow statement data for a company
#[derive(Debug, Clone)]
pub struct CashFlowStatement {
    /// Cash from operating activities from continuing operations
    pub cash_from_operating_activities_continuing_operations: Option<f64>,
    /// Net change in cash and cash equivalents
    pub change_in_cash_and_equivalents: Option<f64>,
    /// Net change in other operating assets and liabilities
    pub change_in_other_operating_assets_and_liabilities_net: Option<f64>,
    /// Central Index Key (CIK)
    pub cik: Option<String>,
    /// Depreciation, depletion and amortization expense
    pub depreciation_depletion_and_amortization: Option<f64>,
    /// Dividends paid
    pub dividends: Option<f64>,
    /// Effect of exchange rate changes on cash
    pub effect_of_currency_exchange_rate: Option<f64>,
    /// Filing date (YYYY-MM-DD)
    pub filing_date: Option<String>,
    /// Fiscal quarter (1-4)
    pub fiscal_quarter: Option<f64>,
    /// Fiscal year
    pub fiscal_year: Option<f64>,
    /// Income or loss from discontinued operations
    pub income_loss_from_discontinued_operations: Option<f64>,
    /// Net issuances or repayments of long-term debt
    pub long_term_debt_issuances_repayments: Option<f64>,
    /// Net cash from financing activities
    pub net_cash_from_financing_activities: Option<f64>,
    /// Net cash from financing activities (continuing operations)
    pub net_cash_from_financing_activities_continuing_operations: Option<f64>,
    /// Net cash from financing activities (discontinued operations)
    pub net_cash_from_financing_activities_discontinued_operations: Option<f64>,
    /// Net cash from investing activities
    pub net_cash_from_investing_activities: Option<f64>,
    /// Net cash from investing activities (continuing operations)
    pub net_cash_from_investing_activities_continuing_operations: Option<f64>,
    /// Net cash from investing activities (discontinued operations)
    pub net_cash_from_investing_activities_discontinued_operations: Option<f64>,
    /// Net cash from operating activities
    pub net_cash_from_operating_activities: Option<f64>,
    /// Net cash from operating activities (discontinued operations)
    pub net_cash_from_operating_activities_discontinued_operations: Option<f64>,
    /// Net income
    pub net_income: Option<f64>,
    /// Noncontrolling interests
    pub noncontrolling_interests: Option<f64>,
    /// Other cash adjustments
    pub other_cash_adjustments: Option<f64>,
    /// Other financing activities
    pub other_financing_activities: Option<f64>,
    /// Other investing activities
    pub other_investing_activities: Option<f64>,
    /// Other operating activities
    pub other_operating_activities: Option<f64>,
    /// Period end date (YYYY-MM-DD)
    pub period_end: Option<String>,
    /// Purchases of property, plant, and equipment
    pub purchase_of_property_plant_and_equipment: Option<f64>,
    /// Proceeds from sale of property, plant, and equipment
    pub sale_of_property_plant_and_equipment: Option<f64>,
    /// Net issuances or repayments of short-term debt
    pub short_term_debt_issuances_repayments: Option<f64>,
    /// List of ticker symbols
    pub tickers: Option<Vec<String>>,
    /// Timeframe (quarterly or annual)
    pub timeframe: Option<String>,
}

/// Income statement data for a company
#[derive(Debug, Clone)]
pub struct IncomeStatement {
    /// Basic earnings per share
    pub basic_earnings_per_share: Option<f64>,
    /// Basic shares outstanding
    pub basic_shares_outstanding: Option<f64>,
    /// Central Index Key (CIK)
    pub cik: Option<String>,
    /// Consolidated net income or loss
    pub consolidated_net_income_loss: Option<f64>,
    /// Cost of revenue
    pub cost_of_revenue: Option<f64>,
    /// Depreciation, depletion and amortization
    pub depreciation_depletion_amortization: Option<f64>,
    /// Diluted earnings per share
    pub diluted_earnings_per_share: Option<f64>,
    /// Diluted shares outstanding
    pub diluted_shares_outstanding: Option<f64>,
    /// Income or loss from discontinued operations
    pub discontinued_operations: Option<f64>,
    /// Earnings before interest, taxes, depreciation and amortization
    pub ebitda: Option<f64>,
    /// Equity in earnings or losses of affiliates
    pub equity_in_affiliates: Option<f64>,
    /// Extraordinary items
    pub extraordinary_items: Option<f64>,
    /// Filing date (YYYY-MM-DD)
    pub filing_date: Option<String>,
    /// Fiscal quarter (1-4)
    pub fiscal_quarter: Option<f64>,
    /// Fiscal year
    pub fiscal_year: Option<f64>,
    /// Gross profit
    pub gross_profit: Option<f64>,
    /// Income before income taxes
    pub income_before_income_taxes: Option<f64>,
    /// Income tax expense
    pub income_taxes: Option<f64>,
    /// Interest expense
    pub interest_expense: Option<f64>,
    /// Interest income
    pub interest_income: Option<f64>,
    /// Net income or loss attributable to common shareholders
    pub net_income_loss_attributable_common_shareholders: Option<f64>,
    /// Noncontrolling interest
    pub noncontrolling_interest: Option<f64>,
    /// Operating income or loss
    pub operating_income: Option<f64>,
    /// Other income or expense
    pub other_income_expense: Option<f64>,
    /// Other operating expenses
    pub other_operating_expenses: Option<f64>,
    /// Period end date (YYYY-MM-DD)
    pub period_end: Option<String>,
    /// Preferred stock dividends declared
    pub preferred_stock_dividends_declared: Option<f64>,
    /// Research and development expenses
    pub research_development: Option<f64>,
    /// Total revenue
    pub revenue: Option<f64>,
    /// Selling, general and administrative expenses
    pub selling_general_administrative: Option<f64>,
    /// List of ticker symbols
    pub tickers: Option<Vec<String>>,
    /// Timeframe (quarterly or annual)
    pub timeframe: Option<String>,
    /// Total operating expenses
    pub total_operating_expenses: Option<f64>,
    /// Total other income or expense
    pub total_other_income_expense: Option<f64>,
}

/// Financial ratios data for a company
#[derive(Debug, Clone)]
pub struct FinancialRatio {
    /// Average trading volume (30-day)
    pub average_volume: Option<f64>,
    /// Current ratio (cash / current liabilities)
    pub cash: Option<f64>,
    /// Central Index Key (CIK)
    pub cik: Option<String>,
    /// Current ratio (current assets / current liabilities)
    pub current: Option<f64>,
    /// Date of the ratio calculation (YYYY-MM-DD)
    pub date: Option<String>,
    /// Debt-to-equity ratio
    pub debt_to_equity: Option<f64>,
    /// Dividend yield
    pub dividend_yield: Option<f64>,
    /// Earnings per share (EPS)
    pub earnings_per_share: Option<f64>,
    /// Enterprise value
    pub enterprise_value: Option<f64>,
    /// Enterprise value to EBITDA ratio
    pub ev_to_ebitda: Option<f64>,
    /// Enterprise value to sales ratio
    pub ev_to_sales: Option<f64>,
    /// Free cash flow
    pub free_cash_flow: Option<f64>,
    /// Market capitalization
    pub market_cap: Option<f64>,
    /// Stock price
    pub price: Option<f64>,
    /// Price-to-book ratio
    pub price_to_book: Option<f64>,
    /// Price-to-cash-flow ratio
    pub price_to_cash_flow: Option<f64>,
    /// Price-to-earnings (P/E) ratio
    pub price_to_earnings: Option<f64>,
    /// Price-to-free-cash-flow ratio
    pub price_to_free_cash_flow: Option<f64>,
    /// Price-to-sales ratio
    pub price_to_sales: Option<f64>,
    /// Quick ratio (liquid assets / current liabilities)
    pub quick: Option<f64>,
    /// Return on assets (ROA)
    pub return_on_assets: Option<f64>,
    /// Return on equity (ROE)
    pub return_on_equity: Option<f64>,
    /// Ticker symbol
    pub ticker: Option<String>,
}

/// Trait for types that can decode financial data from decoder::Value
pub trait DecodeFinancials: Sized {
    /// Provide the decoder function for this type
    fn decoder_fn() -> impl Fn(decoder::Value) -> decoder::Result<Self> + Send + Sync + 'static;
}

impl DecodeFinancials for Vec<BalanceSheet> {
    fn decoder_fn() -> impl Fn(decoder::Value) -> decoder::Result<Self> + Send + Sync + 'static {
        use decoder::decode::{f64, map, sequence, string};

        |value: decoder::Value| {
            let mut response = map(value)?;
            response.required(
                "results",
                sequence(|v| {
                    let mut obj = map(v)?;
                    Ok(BalanceSheet {
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
                }),
            )
        }
    }
}

impl DecodeFinancials for Vec<CashFlowStatement> {
    fn decoder_fn() -> impl Fn(decoder::Value) -> decoder::Result<Self> + Send + Sync + 'static {
        use decoder::decode::{f64, map, sequence, string};

        |value: decoder::Value| {
            let mut response = map(value)?;
            response.required(
                "results",
                sequence(|v| {
                    let mut obj = map(v)?;
                    Ok(CashFlowStatement {
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
                        net_cash_from_financing_activities: obj.optional("net_cash_from_financing_activities", f64)?,
                        net_cash_from_financing_activities_continuing_operations: obj
                            .optional("net_cash_from_financing_activities_continuing_operations", f64)?,
                        net_cash_from_financing_activities_discontinued_operations: obj
                            .optional("net_cash_from_financing_activities_discontinued_operations", f64)?,
                        net_cash_from_investing_activities: obj.optional("net_cash_from_investing_activities", f64)?,
                        net_cash_from_investing_activities_continuing_operations: obj
                            .optional("net_cash_from_investing_activities_continuing_operations", f64)?,
                        net_cash_from_investing_activities_discontinued_operations: obj
                            .optional("net_cash_from_investing_activities_discontinued_operations", f64)?,
                        net_cash_from_operating_activities: obj.optional("net_cash_from_operating_activities", f64)?,
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
                }),
            )
        }
    }
}

impl DecodeFinancials for Vec<IncomeStatement> {
    fn decoder_fn() -> impl Fn(decoder::Value) -> decoder::Result<Self> + Send + Sync + 'static {
        use decoder::decode::{f64, map, sequence, string};

        |value: decoder::Value| {
            let mut response = map(value)?;
            response.required(
                "results",
                sequence(|v| {
                    let mut obj = map(v)?;
                    Ok(IncomeStatement {
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
                        preferred_stock_dividends_declared: obj.optional("preferred_stock_dividends_declared", f64)?,
                        research_development: obj.optional("research_development", f64)?,
                        revenue: obj.optional("revenue", f64)?,
                        selling_general_administrative: obj.optional("selling_general_administrative", f64)?,
                        tickers: obj.optional("tickers", sequence(string))?,
                        timeframe: obj.optional("timeframe", string)?,
                        total_operating_expenses: obj.optional("total_operating_expenses", f64)?,
                        total_other_income_expense: obj.optional("total_other_income_expense", f64)?,
                    })
                }),
            )
        }
    }
}

impl DecodeFinancials for Vec<FinancialRatio> {
    fn decoder_fn() -> impl Fn(decoder::Value) -> decoder::Result<Self> + Send + Sync + 'static {
        use decoder::decode::{f64, map, sequence, string};

        |value: decoder::Value| {
            let mut response = map(value)?;
            response.required(
                "results",
                sequence(|v| {
                    let mut obj = map(v)?;
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
                }),
            )
        }
    }
}
