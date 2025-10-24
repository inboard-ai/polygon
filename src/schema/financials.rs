//! Financial data types

/// Balance sheet data for a company
#[derive(Debug, Clone)]
pub struct FinancialBalanceSheet {
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
pub struct FinancialCashFlowStatement {
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
pub struct FinancialIncomeStatement {
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

#[cfg(feature = "decoder")]
impl crate::query::Decodable for Vec<FinancialBalanceSheet> {}

#[cfg(feature = "decoder")]
impl crate::query::Decodable for Vec<FinancialCashFlowStatement> {}

#[cfg(feature = "decoder")]
impl crate::query::Decodable for Vec<FinancialIncomeStatement> {}

#[cfg(feature = "decoder")]
impl crate::query::Decodable for Vec<FinancialRatio> {}
