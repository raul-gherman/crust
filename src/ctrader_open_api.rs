/// Base message that is used for all messages that are sent to/from Open API proxy of cTrader platform.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoMessage {
    /// Contains id of ProtoPayloadType or other custom PayloadTypes (e.g. ProtoOAPayloadType).
    #[prost(uint32, required, tag = "1")]
    pub payload_type: u32,
    /// Serialized protobuf message that corresponds to payloadType.
    #[prost(bytes = "vec", optional, tag = "2")]
    pub payload: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// Request message id, assigned by the client that will be returned in the response.
    #[prost(string, optional, tag = "3")]
    pub client_msg_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoErrorRes {
    #[prost(
        enumeration = "ProtoPayloadType",
        optional,
        tag = "1",
        default = "ErrorRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Contains name of ProtoErrorCode or other custom ErrorCodes (e.g. ProtoCHErrorCode).
    #[prost(string, required, tag = "2")]
    pub error_code: ::prost::alloc::string::String,
    /// Error description.
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// The Unix time in milliseconds of the end of the maintenance.
    #[prost(uint64, optional, tag = "4")]
    pub maintenance_end_timestamp: ::core::option::Option<u64>,
}
/// Event that is sent from Open API proxy and can be used as criteria that connection is healthy when no other messages are sent by cTrader platform
/// Open API client can send this message when it needs to keep the connection open for a period longer than 30 seconds without other messages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoHeartbeatEvent {
    #[prost(
        enumeration = "ProtoPayloadType",
        optional,
        tag = "1",
        default = "HeartbeatEvent"
    )]
    pub payload_type: ::core::option::Option<i32>,
}
/// Asset entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaAsset {
    /// The unique asset ID.
    #[prost(int64, required, tag = "1")]
    pub asset_id: i64,
    /// The asset name.
    #[prost(string, required, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// User friendly name.
    #[prost(string, optional, tag = "3")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Precision of the asset.
    #[prost(int32, optional, tag = "4")]
    pub digits: ::core::option::Option<i32>,
}
///   Symbol trading session entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaInterval {
    /// Interval start, specified in seconds starting from SUNDAY 00:00 in specified time zone (inclusive to the interval).
    #[prost(uint32, required, tag = "3")]
    pub start_second: u32,
    /// Interval end, specified in seconds starting from SUNDAY 00:00 in specified time zone (exclusive from the interval).
    #[prost(uint32, required, tag = "4")]
    pub end_second: u32,
}
/// Trading symbol entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSymbol {
    /// The unique identifier of the symbol in specific server environment within cTrader platform. Different servers have different IDs.
    #[prost(int64, required, tag = "1")]
    pub symbol_id: i64,
    /// Number of price digits to be displayed.
    #[prost(int32, required, tag = "2")]
    pub digits: i32,
    /// Pip position on digits.
    #[prost(int32, required, tag = "3")]
    pub pip_position: i32,
    /// If TRUE then the short selling with the symbol is enabled.
    #[prost(bool, optional, tag = "4")]
    pub enable_short_selling: ::core::option::Option<bool>,
    /// If TRUE then setting of guaranteedStopLoss is available for limited risk accounts.
    #[prost(bool, optional, tag = "5")]
    pub guaranteed_stop_loss: ::core::option::Option<bool>,
    /// Day of the week when 3x rollover is charged.
    #[prost(
        enumeration = "ProtoOaDayOfWeek",
        optional,
        tag = "6",
        default = "Monday"
    )]
    pub swap_rollover3_days: ::core::option::Option<i32>,
    /// SWAP charge for long positions.
    #[prost(double, optional, tag = "7")]
    pub swap_long: ::core::option::Option<f64>,
    /// SWAP charge for short positions.
    #[prost(double, optional, tag = "8")]
    pub swap_short: ::core::option::Option<f64>,
    /// Maximum allowed volume in cents for an order with a symbol.
    #[prost(int64, optional, tag = "9")]
    pub max_volume: ::core::option::Option<i64>,
    /// Minimum allowed volume in cents for an order with a symbol.
    #[prost(int64, optional, tag = "10")]
    pub min_volume: ::core::option::Option<i64>,
    /// Step of the volume in cents for an order.
    #[prost(int64, optional, tag = "11")]
    pub step_volume: ::core::option::Option<i64>,
    /// Value of max exposure per symbol, per account. Blocks execution if breached.
    #[prost(uint64, optional, tag = "12")]
    pub max_exposure: ::core::option::Option<u64>,
    /// Symbol trading interval, specified in seconds starting from SUNDAY 00:00 in specified time zone.
    #[prost(message, repeated, tag = "13")]
    pub schedule: ::prost::alloc::vec::Vec<ProtoOaInterval>,
    /// Commission base amount. Total commission depends on commissionType. Use preciseTradingCommissionRate.
    #[deprecated]
    #[prost(int64, optional, tag = "14")]
    pub commission: ::core::option::Option<i64>,
    /// Commission type. See ProtoOACommissionType for details.
    #[prost(
        enumeration = "ProtoOaCommissionType",
        optional,
        tag = "15",
        default = "UsdPerMillionUsd"
    )]
    pub commission_type: ::core::option::Option<i32>,
    /// Minimum allowed distance between stop loss and current market price.
    #[prost(uint32, optional, tag = "16")]
    pub sl_distance: ::core::option::Option<u32>,
    /// Minimum allowed distance between take profit and current market price.
    #[prost(uint32, optional, tag = "17")]
    pub tp_distance: ::core::option::Option<u32>,
    /// Minimum allowed distance between guaranteed stop loss and current market price.
    #[prost(uint32, optional, tag = "18")]
    pub gsl_distance: ::core::option::Option<u32>,
    /// Guaranteed stop loss fee.
    #[prost(int64, optional, tag = "19")]
    pub gsl_charge: ::core::option::Option<i64>,
    /// Unit of distance measure for slDistance, tpDistance, gslDistance.
    #[prost(
        enumeration = "ProtoOaSymbolDistanceType",
        optional,
        tag = "20",
        default = "SymbolDistanceInPoints"
    )]
    pub distance_set_in: ::core::option::Option<i32>,
    /// Minimum commission amount per trade. Use preciseMinCommission.
    #[deprecated]
    #[prost(int64, optional, tag = "21")]
    pub min_commission: ::core::option::Option<i64>,
    /// Minimum commission Type. See ProtoOAMinCommissionType for details.
    #[prost(
        enumeration = "ProtoOaMinCommissionType",
        optional,
        tag = "22",
        default = "Currency"
    )]
    pub min_commission_type: ::core::option::Option<i32>,
    /// Currency for minimum commission. (USD or quote currency).
    #[prost(string, optional, tag = "23", default = "USD")]
    pub min_commission_asset: ::core::option::Option<::prost::alloc::string::String>,
    /// Administrative Fee, charged instead of Swaps if the Account is marked as a "Shariah Compliant (Swap Free)". The Administrative Fee is charged daily as USD per current open volume of Position in lots. The Account charged in the Deposit currency.
    #[prost(int64, optional, tag = "24")]
    pub rollover_commission: ::core::option::Option<i64>,
    /// Initial period before the first rolloverCommission will be charged on the account.
    #[prost(int32, optional, tag = "25")]
    pub skip_rollover_days: ::core::option::Option<i32>,
    /// Time zone for the symbol trading intervals.
    #[prost(string, optional, tag = "26")]
    pub schedule_time_zone: ::core::option::Option<::prost::alloc::string::String>,
    /// Rules for trading with the symbol. See ProtoOATradingMode for details.
    #[prost(
        enumeration = "ProtoOaTradingMode",
        optional,
        tag = "27",
        default = "Enabled"
    )]
    pub trading_mode: ::core::option::Option<i32>,
    /// Day of the week (in UTC) when Administrative Fee charge amount will be tripled. Applied only if RolloverChargePeriod = 0 or 1
    #[prost(
        enumeration = "ProtoOaDayOfWeek",
        optional,
        tag = "28",
        default = "Monday"
    )]
    pub rollover_commission3_days: ::core::option::Option<i32>,
    /// Specifies type of SWAP computation as PIPS (0) or PERCENTAGE (1, annual, in percent)
    #[prost(
        enumeration = "ProtoOaSwapCalculationType",
        optional,
        tag = "29",
        default = "Pips"
    )]
    pub swap_calculation_type: ::core::option::Option<i32>,
    /// Lot size of the Symbol (in cents)
    #[prost(int64, optional, tag = "30")]
    pub lot_size: ::core::option::Option<i64>,
    /// Commission base amount. Total commission depends on commissionType: for non-percentage types it is multiplied by 10^8, for percentage of value commission type it is multiplied by 10^5
    #[prost(int64, optional, tag = "31")]
    pub precise_trading_commission_rate: ::core::option::Option<i64>,
    /// Minimum commission amount per trade multiplied by 10^8.
    #[prost(int64, optional, tag = "32")]
    pub precise_min_commission: ::core::option::Option<i64>,
    /// List of holidays for this symbol specified by broker.
    #[prost(message, repeated, tag = "33")]
    pub holiday: ::prost::alloc::vec::Vec<ProtoOaHoliday>,
    /// Percentage (1 = 0.01%) of the realized Gross Profit, which will be paid by the Trader for any trade if the Quote Asset of the traded Symbol is not matched with the Deposit Asset.
    #[prost(int32, optional, tag = "34")]
    pub pnl_conversion_fee_rate: ::core::option::Option<i32>,
    /// The unique identifier of dynamic leverage entity. <https://help.ctrader.com/ctrader/trading/dynamic-leverage>
    #[prost(int64, optional, tag = "35")]
    pub leverage_id: ::core::option::Option<i64>,
    /// swap is calculated every swapPeriod hours
    #[prost(int32, optional, tag = "36")]
    pub swap_period: ::core::option::Option<i32>,
    /// from which swap period is calculated, in minutes from 00:00.
    #[prost(int32, optional, tag = "37")]
    pub swap_time: ::core::option::Option<i32>,
    /// Count of swapPeriods before first SWAP charge.
    #[prost(int32, optional, tag = "38")]
    pub skip_swap_periods: ::core::option::Option<i32>,
    /// If enabled SWAP will be charged for all days of the week, including Saturday and Sunday.
    #[prost(bool, optional, tag = "39")]
    pub charge_swap_at_weekends: ::core::option::Option<bool>,
    /// Specifies the units in which the base Asset of the Symbol is denominated.
    #[prost(string, optional, tag = "40")]
    pub measurement_units: ::core::option::Option<::prost::alloc::string::String>,
}
/// Lightweight symbol entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaLightSymbol {
    /// The unique identifier of the symbol in specific server environment within cTrader platform. Different brokers might have different IDs.
    #[prost(int64, required, tag = "1")]
    pub symbol_id: i64,
    /// Name of the symbol (e.g. EUR/USD).
    #[prost(string, optional, tag = "2")]
    pub symbol_name: ::core::option::Option<::prost::alloc::string::String>,
    /// If TRUE then symbol is visible for traders.
    #[prost(bool, optional, tag = "3")]
    pub enabled: ::core::option::Option<bool>,
    /// Base asset.
    #[prost(int64, optional, tag = "4")]
    pub base_asset_id: ::core::option::Option<i64>,
    /// Quote asset.
    #[prost(int64, optional, tag = "5")]
    pub quote_asset_id: ::core::option::Option<i64>,
    /// Id of the symbol category used for symbols grouping.
    #[prost(int64, optional, tag = "6")]
    pub symbol_category_id: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "7")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// The number used for sorting Symbols in the UI (lowest number should appear at the top).
    #[prost(double, optional, tag = "8")]
    pub sorting_number: ::core::option::Option<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaArchivedSymbol {
    /// The unique identifier of the symbol in specific server environment within cTrader platform. Different brokers might have different IDs.
    #[prost(int64, required, tag = "1")]
    pub symbol_id: i64,
    /// Name of the symbol (e.g. EUR/USD).
    #[prost(string, required, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// The Unix time in milliseconds of the last update of the symbol.
    #[prost(int64, required, tag = "3")]
    pub utc_last_update_timestamp: i64,
    /// Description of the symbol.
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// Symbol category entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSymbolCategory {
    /// The unique identifier of the symbol category.
    #[prost(int64, required, tag = "1")]
    pub id: i64,
    /// Link to the asset class. One asset class can have many symbol categories.
    #[prost(int64, required, tag = "2")]
    pub asset_class_id: i64,
    /// Category name.
    #[prost(string, required, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// The number used for sorting Symbols in the UI (lowest number should appear at the top).
    #[prost(double, optional, tag = "4")]
    pub sorting_number: ::core::option::Option<f64>,
}
/// Trading account entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaTrader {
    /// The unique trading account ID used to match the responses to the trading account.
    #[prost(int64, required, tag = "1")]
    pub ctid_trader_account_id: i64,
    /// Current account balance in cents (e.g. If USD 100.00 then value = 10000).
    #[prost(int64, required, tag = "2")]
    pub balance: i64,
    /// Balance version used to identify the final balance. Increments each time when the trading account balance is changed.
    #[prost(int64, optional, tag = "3")]
    pub balance_version: ::core::option::Option<i64>,
    /// Amount of broker bonus allocated to the account in cents.
    #[prost(int64, optional, tag = "4")]
    pub manager_bonus: ::core::option::Option<i64>,
    /// Amount of introducing broker bonus allocated to the account in cents.
    #[prost(int64, optional, tag = "5")]
    pub ib_bonus: ::core::option::Option<i64>,
    /// Broker bonus that cannot be withdrew from the account as cash.
    #[prost(int64, optional, tag = "6")]
    pub non_withdrawable_bonus: ::core::option::Option<i64>,
    /// Access rights that an owner has to the account in cTrader platform. See ProtoOAAccessRights for details.
    #[prost(
        enumeration = "ProtoOaAccessRights",
        optional,
        tag = "7",
        default = "FullAccess"
    )]
    pub access_rights: ::core::option::Option<i32>,
    /// Deposit currency of the account.
    #[prost(int64, required, tag = "8")]
    pub deposit_asset_id: i64,
    /// If TRUE then account is Shariah compliant.
    #[prost(bool, optional, tag = "9")]
    pub swap_free: ::core::option::Option<bool>,
    /// Account leverage (e.g. If leverage = 1:50 then value = 5000).
    #[prost(uint32, optional, tag = "10")]
    pub leverage_in_cents: ::core::option::Option<u32>,
    /// Margin computation type for the account (MAX, SUM, NET).
    #[prost(
        enumeration = "ProtoOaTotalMarginCalculationType",
        optional,
        tag = "11"
    )]
    pub total_margin_calculation_type: ::core::option::Option<i32>,
    /// Maximum allowed leverage for the account. Used as validation when a Trader can change leverage value.
    #[prost(uint32, optional, tag = "12")]
    pub max_leverage: ::core::option::Option<u32>,
    /// If TRUE then account is AMF compliant. Use isLimitedRisk and limitedRiskMarginCalculationStrategy.
    #[deprecated]
    #[prost(bool, optional, tag = "13")]
    pub french_risk: ::core::option::Option<bool>,
    /// ID of the account that is unique per server (Broker).
    #[prost(int64, optional, tag = "14")]
    pub trader_login: ::core::option::Option<i64>,
    /// Account type: HEDGED, NETTED, etc.
    #[prost(
        enumeration = "ProtoOaAccountType",
        optional,
        tag = "15",
        default = "Hedged"
    )]
    pub account_type: ::core::option::Option<i32>,
    /// Some whitelabel assigned to trader by broker at the moment of account creation.
    #[prost(string, optional, tag = "16")]
    pub broker_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The Unix timestamp in milliseconds of the account registration. Should be used as minimal date in historical data requests.
    #[prost(int64, optional, tag = "17")]
    pub registration_timestamp: ::core::option::Option<i64>,
    /// If TRUE then account is compliant to use specific margin calculation strategy.
    #[prost(bool, optional, tag = "18")]
    pub is_limited_risk: ::core::option::Option<bool>,
    /// Special strategy used in margin calculations for this account (if account isLimitedRisk).
    #[prost(
        enumeration = "ProtoOaLimitedRiskMarginCalculationStrategy",
        optional,
        tag = "19",
        default = "AccordingToLeverage"
    )]
    pub limited_risk_margin_calculation_strategy: ::core::option::Option<i32>,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects balance, managerBonus, ibBonus, nonWithdrawableBonus.
    #[prost(uint32, optional, tag = "20")]
    pub money_digits: ::core::option::Option<u32>,
    /// If TRUE - Position is fully closed on Stop Out, if FALSE - smart (partial closing) Stop Out is applied, if unspecified  - Stop Out format is determined by Broker.
    #[prost(bool, optional, tag = "21")]
    pub fair_stop_out: ::core::option::Option<bool>,
}
/// Position/order trading details entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaTradeData {
    /// The unique identifier of the symbol in specific server environment within cTrader platform. Different brokers might have different IDs.
    #[prost(int64, required, tag = "1")]
    pub symbol_id: i64,
    /// Volume in cents.
    #[prost(int64, required, tag = "2")]
    pub volume: i64,
    /// Buy, Sell.
    #[prost(enumeration = "ProtoOaTradeSide", required, tag = "3")]
    pub trade_side: i32,
    /// The Unix time in milliseconds when position was opened or order was created.
    #[prost(int64, optional, tag = "4")]
    pub open_timestamp: ::core::option::Option<i64>,
    /// Text label specified during order request.
    #[prost(string, optional, tag = "5")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    /// If TRUE then position/order stop loss is guaranteedStopLoss.
    #[prost(bool, optional, tag = "6")]
    pub guaranteed_stop_loss: ::core::option::Option<bool>,
    /// User-specified comment.
    #[prost(string, optional, tag = "7")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    /// Specifies the units in which the Symbol is denominated.
    #[prost(string, optional, tag = "8")]
    pub measurement_units: ::core::option::Option<::prost::alloc::string::String>,
    /// The Unix time in milliseconds when a Position was closed
    #[prost(uint64, optional, tag = "9")]
    pub close_timestamp: ::core::option::Option<u64>,
}
/// Trade position entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaPosition {
    /// The unique ID of the position. Note: trader might have two positions with the same id if positions are taken from accounts from different brokers.
    #[prost(int64, required, tag = "1")]
    pub position_id: i64,
    /// Position details. See ProtoOATradeData for details.
    #[prost(message, required, tag = "2")]
    pub trade_data: ProtoOaTradeData,
    /// Current status of the position.
    #[prost(enumeration = "ProtoOaPositionStatus", required, tag = "3")]
    pub position_status: i32,
    /// Total amount of charged swap on open position.
    #[prost(int64, required, tag = "4")]
    pub swap: i64,
    /// VWAP price of the position based on all executions (orders) linked to the position.
    #[prost(double, optional, tag = "5")]
    pub price: ::core::option::Option<f64>,
    /// Current stop loss price.
    #[prost(double, optional, tag = "6")]
    pub stop_loss: ::core::option::Option<f64>,
    /// Current take profit price.
    #[prost(double, optional, tag = "7")]
    pub take_profit: ::core::option::Option<f64>,
    /// The Unix time in milliseconds of the last change of the position, including amend SL/TP of the position, execution of related order, cancel or related order, etc.
    #[prost(int64, optional, tag = "8")]
    pub utc_last_update_timestamp: ::core::option::Option<i64>,
    /// Current unrealized commission related to the position.
    #[prost(int64, optional, tag = "9")]
    pub commission: ::core::option::Option<i64>,
    /// Rate for used margin computation. Represented as Base/Deposit.
    #[prost(double, optional, tag = "10")]
    pub margin_rate: ::core::option::Option<f64>,
    /// Amount of unrealized commission related to following of strategy provider.
    #[prost(int64, optional, tag = "11")]
    pub mirroring_commission: ::core::option::Option<i64>,
    /// If TRUE then position stop loss is guaranteedStopLoss.
    #[prost(bool, optional, tag = "12")]
    pub guaranteed_stop_loss: ::core::option::Option<bool>,
    /// Amount of margin used for the position in deposit currency.
    #[prost(uint64, optional, tag = "13")]
    pub used_margin: ::core::option::Option<u64>,
    /// Stop trigger method for SL/TP of the position.
    #[prost(
        enumeration = "ProtoOaOrderTriggerMethod",
        optional,
        tag = "14",
        default = "Trade"
    )]
    pub stop_loss_trigger_method: ::core::option::Option<i32>,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects usedMargin.
    #[prost(uint32, optional, tag = "15")]
    pub money_digits: ::core::option::Option<u32>,
    /// If TRUE then the Trailing Stop Loss is applied.
    #[prost(bool, optional, tag = "16")]
    pub trailing_stop_loss: ::core::option::Option<bool>,
}
/// Trade order entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaOrder {
    /// The unique ID of the order. Note: trader might have two orders with the same id if orders are taken from accounts from different brokers.
    #[prost(int64, required, tag = "1")]
    pub order_id: i64,
    /// Detailed trade data.
    #[prost(message, required, tag = "2")]
    pub trade_data: ProtoOaTradeData,
    /// Order type.
    #[prost(enumeration = "ProtoOaOrderType", required, tag = "3")]
    pub order_type: i32,
    /// Order status.
    #[prost(enumeration = "ProtoOaOrderStatus", required, tag = "4")]
    pub order_status: i32,
    /// The Unix time in milliseconds of expiration if the order has time in force GTD.
    #[prost(int64, optional, tag = "6")]
    pub expiration_timestamp: ::core::option::Option<i64>,
    /// Price at which an order was executed. For order with FILLED status.
    #[prost(double, optional, tag = "7")]
    pub execution_price: ::core::option::Option<f64>,
    /// Part of the volume that was filled.
    #[prost(int64, optional, tag = "8")]
    pub executed_volume: ::core::option::Option<i64>,
    /// The Unix time in milliseconds of the last update of the order.
    #[prost(int64, optional, tag = "9")]
    pub utc_last_update_timestamp: ::core::option::Option<i64>,
    /// Used for Market Range order with combination of slippageInPoints to specify price range were order can be executed.
    #[prost(double, optional, tag = "10")]
    pub base_slippage_price: ::core::option::Option<f64>,
    /// Used for Market Range and STOP_LIMIT orders to to specify price range were order can be executed.
    #[prost(int64, optional, tag = "11")]
    pub slippage_in_points: ::core::option::Option<i64>,
    /// If TRUE then the order is closing part of whole position. Must have specified positionId.
    #[prost(bool, optional, tag = "12")]
    pub closing_order: ::core::option::Option<bool>,
    /// Valid only for LIMIT orders.
    #[prost(double, optional, tag = "13")]
    pub limit_price: ::core::option::Option<f64>,
    /// Valid only for STOP and STOP_LIMIT orders.
    #[prost(double, optional, tag = "14")]
    pub stop_price: ::core::option::Option<f64>,
    /// Absolute stopLoss price.
    #[prost(double, optional, tag = "15")]
    pub stop_loss: ::core::option::Option<f64>,
    /// Absolute takeProfit price.
    #[prost(double, optional, tag = "16")]
    pub take_profit: ::core::option::Option<f64>,
    /// Optional ClientOrderId. Max Length = 50 chars.
    #[prost(string, optional, tag = "17")]
    pub client_order_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Order time in force. Depends on order type.
    #[prost(
        enumeration = "ProtoOaTimeInForce",
        optional,
        tag = "18",
        default = "ImmediateOrCancel"
    )]
    pub time_in_force: ::core::option::Option<i32>,
    /// ID of the position linked to the order (e.g. closing order, order that increase volume of a specific position, etc.).
    #[prost(int64, optional, tag = "19")]
    pub position_id: ::core::option::Option<i64>,
    /// Relative stopLoss that can be specified instead of absolute as one. Specified in 1/100_000 of unit of a price. For BUY stopLoss = entryPrice - relativeStopLoss, for SELL stopLoss = entryPrice + relativeStopLoss.
    #[prost(int64, optional, tag = "20")]
    pub relative_stop_loss: ::core::option::Option<i64>,
    /// Relative takeProfit that can be specified instead of absolute one. Specified in 1/100_000 of unit of a price. For BUY takeProfit = entryPrice + relativeTakeProfit, for SELL takeProfit = entryPrice - relativeTakeProfit.
    #[prost(int64, optional, tag = "21")]
    pub relative_take_profit: ::core::option::Option<i64>,
    /// If TRUE then order was stopped out from server side.
    #[prost(bool, optional, tag = "22")]
    pub is_stop_out: ::core::option::Option<bool>,
    /// If TRUE then order is trailingStopLoss. Valid for STOP_LOSS_TAKE_PROFIT order.
    #[prost(bool, optional, tag = "23")]
    pub trailing_stop_loss: ::core::option::Option<bool>,
    /// Trigger method for the order. Valid only for STOP and STOP_LIMIT orders.
    #[prost(
        enumeration = "ProtoOaOrderTriggerMethod",
        optional,
        tag = "24",
        default = "Trade"
    )]
    pub stop_trigger_method: ::core::option::Option<i32>,
}
/// Bonus deposit/withdrawal entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaBonusDepositWithdraw {
    /// Type of the operation. Deposit/Withdrawal.
    #[prost(enumeration = "ProtoOaChangeBonusType", required, tag = "1")]
    pub operation_type: i32,
    /// The unique ID of the bonus deposit/withdrawal operation.
    #[prost(int64, required, tag = "2")]
    pub bonus_history_id: i64,
    /// Total amount of broker bonus after the operation.
    #[prost(int64, required, tag = "3")]
    pub manager_bonus: i64,
    /// Amount of bonus deposited/withdrew by manager.
    #[prost(int64, required, tag = "4")]
    pub manager_delta: i64,
    /// Total amount of introducing broker bonus after the operation.
    #[prost(int64, required, tag = "5")]
    pub ib_bonus: i64,
    /// Amount of bonus deposited/withdrew by introducing broker.
    #[prost(int64, required, tag = "6")]
    pub ib_delta: i64,
    /// The Unix time in milliseconds when the bonus operation was executed.
    #[prost(int64, required, tag = "7")]
    pub change_bonus_timestamp: i64,
    /// Note added to operation. Visible to the trader.
    #[prost(string, optional, tag = "8")]
    pub external_note: ::core::option::Option<::prost::alloc::string::String>,
    /// ID of introducing broker who deposited/withdrew bonus.
    #[prost(int64, optional, tag = "9")]
    pub introducing_broker_id: ::core::option::Option<i64>,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects managerBonus, managerDelta, ibBonus, ibDelta.
    #[prost(uint32, optional, tag = "10")]
    pub money_digits: ::core::option::Option<u32>,
}
/// Account deposit/withdrawal operation entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaDepositWithdraw {
    /// Type of the operation. Deposit/Withdrawal.
    #[prost(enumeration = "ProtoOaChangeBalanceType", required, tag = "1")]
    pub operation_type: i32,
    /// The unique ID of the deposit/withdrawal operation.
    #[prost(int64, required, tag = "2")]
    pub balance_history_id: i64,
    /// Account balance after the operation was executed.
    #[prost(int64, required, tag = "3")]
    pub balance: i64,
    /// Amount of deposit/withdrawal operation.
    #[prost(int64, required, tag = "4")]
    pub delta: i64,
    /// The Unix time in milliseconds when deposit/withdrawal operation was executed.
    #[prost(int64, required, tag = "5")]
    pub change_balance_timestamp: i64,
    /// Note added to operation. Visible to the trader.
    #[prost(string, optional, tag = "6")]
    pub external_note: ::core::option::Option<::prost::alloc::string::String>,
    /// Balance version used to identify the final balance. Increments each time when the trading account balance is changed.
    #[prost(int64, optional, tag = "7")]
    pub balance_version: ::core::option::Option<i64>,
    /// Total account equity after balance operation was executed.
    #[prost(int64, optional, tag = "8")]
    pub equity: ::core::option::Option<i64>,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects usedMargin.
    #[prost(uint32, optional, tag = "9")]
    pub money_digits: ::core::option::Option<u32>,
}
/// Execution entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaDeal {
    /// The unique ID of the execution deal.
    #[prost(int64, required, tag = "1")]
    pub deal_id: i64,
    /// Source order of the deal.
    #[prost(int64, required, tag = "2")]
    pub order_id: i64,
    /// Source position of the deal.
    #[prost(int64, required, tag = "3")]
    pub position_id: i64,
    /// Volume sent for execution, in cents.
    #[prost(int64, required, tag = "4")]
    pub volume: i64,
    /// Filled volume, in cents.
    #[prost(int64, required, tag = "5")]
    pub filled_volume: i64,
    /// The unique identifier of the symbol in specific server environment within cTrader platform. Different servers have different IDs.
    #[prost(int64, required, tag = "6")]
    pub symbol_id: i64,
    /// The Unix time in milliseconds when the deal was sent for execution.
    #[prost(int64, required, tag = "7")]
    pub create_timestamp: i64,
    /// The Unix time in milliseconds when the deal was executed.
    #[prost(int64, required, tag = "8")]
    pub execution_timestamp: i64,
    /// The Unix time in milliseconds when the deal was created, executed or rejected.
    #[prost(int64, optional, tag = "9")]
    pub utc_last_update_timestamp: ::core::option::Option<i64>,
    /// Execution price.
    #[prost(double, optional, tag = "10")]
    pub execution_price: ::core::option::Option<f64>,
    /// Buy/Sell.
    #[prost(enumeration = "ProtoOaTradeSide", required, tag = "11")]
    pub trade_side: i32,
    /// Status of the deal.
    #[prost(enumeration = "ProtoOaDealStatus", required, tag = "12")]
    pub deal_status: i32,
    /// Rate for used margin computation. Represented as Base/Deposit.
    #[prost(double, optional, tag = "13")]
    pub margin_rate: ::core::option::Option<f64>,
    /// Amount of trading commission associated with the deal.
    #[prost(int64, optional, tag = "14")]
    pub commission: ::core::option::Option<i64>,
    /// Base to USD conversion rate on the time of deal execution.
    #[prost(double, optional, tag = "15")]
    pub base_to_usd_conversion_rate: ::core::option::Option<f64>,
    /// Closing position detail. Valid only for closing deal.
    #[prost(message, optional, tag = "16")]
    pub close_position_detail: ::core::option::Option<ProtoOaClosePositionDetail>,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects managerBonus, managerDelta, ibBonus, ibDelta.
    #[prost(uint32, optional, tag = "17")]
    pub money_digits: ::core::option::Option<u32>,
}
/// Deal details for ProtoOADealOffsetListReq.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaDealOffset {
    /// The unique ID of the execution Deal.
    #[prost(int64, required, tag = "1")]
    pub deal_id: i64,
    /// Matched volume, in cents.
    #[prost(int64, required, tag = "2")]
    pub volume: i64,
    /// The Unix time in milliseconds when the offset Deal was executed.
    #[prost(int64, optional, tag = "3")]
    pub execution_timestamp: ::core::option::Option<i64>,
    /// Execution price of the offset Deal.
    #[prost(double, optional, tag = "4")]
    pub execution_price: ::core::option::Option<f64>,
}
/// Trading details for closing deal
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaClosePositionDetail {
    /// Position price at the moment of filling the closing order.
    #[prost(double, required, tag = "1")]
    pub entry_price: f64,
    /// Amount of realized gross profit after closing deal execution.
    #[prost(int64, required, tag = "2")]
    pub gross_profit: i64,
    /// Amount of realized swap in cents related to closed volume.
    #[prost(int64, required, tag = "3")]
    pub swap: i64,
    /// Amount of realized commission in cents related to closed volume.
    #[prost(int64, required, tag = "4")]
    pub commission: i64,
    /// Account balance after closing deal execution.
    #[prost(int64, required, tag = "5")]
    pub balance: i64,
    /// Quote/Deposit currency conversion rate on the time of closing deal execution.
    #[prost(double, optional, tag = "6")]
    pub quote_to_deposit_conversion_rate: ::core::option::Option<f64>,
    /// Closed volume in cents.
    #[prost(int64, optional, tag = "7")]
    pub closed_volume: ::core::option::Option<i64>,
    /// Balance version of the account related to closing deal operation.
    #[prost(int64, optional, tag = "8")]
    pub balance_version: ::core::option::Option<i64>,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects grossProfit, swap, commission, balance, pnlConversionFee.
    #[prost(uint32, optional, tag = "9")]
    pub money_digits: ::core::option::Option<u32>,
    /// Fee for conversion applied to the Deal in account's ccy when trader symbol's quote asset id <> ProtoOATrader.depositAssetId.}
    #[prost(int64, optional, tag = "10")]
    pub pnl_conversion_fee: ::core::option::Option<i64>,
}
/// Historical Trendbar entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaTrendbar {
    /// Bar volume in cents.
    #[prost(int64, required, tag = "3")]
    pub volume: i64,
    /// Bar period.
    #[prost(
        enumeration = "ProtoOaTrendbarPeriod",
        optional,
        tag = "4",
        default = "M1"
    )]
    pub period: ::core::option::Option<i32>,
    /// Low price of the bar.
    #[prost(int64, optional, tag = "5")]
    pub low: ::core::option::Option<i64>,
    /// Delta between open and low price. open = low + deltaOpen.
    #[prost(uint64, optional, tag = "6")]
    pub delta_open: ::core::option::Option<u64>,
    /// Delta between close and low price. close = low + deltaClose.
    #[prost(uint64, optional, tag = "7")]
    pub delta_close: ::core::option::Option<u64>,
    /// Delta between high and low price. high = low + deltaHigh.
    #[prost(uint64, optional, tag = "8")]
    pub delta_high: ::core::option::Option<u64>,
    /// The Unix time in minutes of the bar, equal to the timestamp of the open tick.
    #[prost(uint32, optional, tag = "9")]
    pub utc_timestamp_in_minutes: ::core::option::Option<u32>,
}
/// Expected margin computation entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaExpectedMargin {
    /// Volume in cents used for computation of expected margin.
    #[prost(int64, required, tag = "1")]
    pub volume: i64,
    /// Buy margin amount in cents.
    #[prost(int64, required, tag = "2")]
    pub buy_margin: i64,
    /// Sell margin amount in cents.
    #[prost(int64, required, tag = "3")]
    pub sell_margin: i64,
}
/// Historical tick data type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaTickData {
    /// The Unix time in milliseconds of the tick. See ProtoOAGetTickDataRes.tickData for details.
    #[prost(int64, required, tag = "1")]
    pub timestamp: i64,
    /// Tick price.
    #[prost(int64, required, tag = "2")]
    pub tick: i64,
}
/// Trader profile entity. Empty due to GDPR
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaCtidProfile {
    #[prost(int64, required, tag = "1")]
    pub user_id: i64,
}
/// Trader account entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaCtidTraderAccount {
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.cTrader platform. Different brokers might have different ids
    #[prost(uint64, required, tag = "1")]
    pub ctid_trader_account_id: u64,
    /// If TRUE then the account is belong to Live environment and live host must be used to authorize it
    #[prost(bool, optional, tag = "2")]
    pub is_live: ::core::option::Option<bool>,
    /// TraderLogin for a specific account. Value is displayed on Client App UI
    #[prost(int64, optional, tag = "3")]
    pub trader_login: ::core::option::Option<i64>,
    /// The Unix time in milliseconds of the last ProtoOAClosePositionDetail happened to this account.
    #[prost(int64, optional, tag = "4")]
    pub last_closing_deal_timestamp: ::core::option::Option<i64>,
    /// The Unix time in milliseconds of the last ProtoOADepositWithdraw happened to this account.
    #[prost(int64, optional, tag = "5")]
    pub last_balance_update_timestamp: ::core::option::Option<i64>,
}
/// Asset class entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaAssetClass {
    /// Unique asset ID.
    #[prost(int64, optional, tag = "1")]
    pub id: ::core::option::Option<i64>,
    /// Asset class name.
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The number used for sorting Asset Classes in the UI (lowest number should appear at the top).
    #[prost(double, optional, tag = "3")]
    pub sorting_number: ::core::option::Option<f64>,
}
/// Depth of market entity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaDepthQuote {
    /// Quote ID.
    #[prost(uint64, required, tag = "1")]
    pub id: u64,
    /// Quote size in cents.
    #[prost(uint64, required, tag = "3")]
    pub size: u64,
    /// Bid price for bid quotes.
    #[prost(uint64, optional, tag = "4")]
    pub bid: ::core::option::Option<u64>,
    /// Ask price for ask quotes.
    #[prost(uint64, optional, tag = "5")]
    pub ask: ::core::option::Option<u64>,
}
/// Margin call entity, specifies threshold for exact margin call type.
/// Only 3 instances of margin calls are supported, identified by marginCallType. See ProtoOANotificationType for details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaMarginCall {
    /// Type of margin call. All margin calls are similar, only difference is in marginLevelThreshold.
    #[prost(enumeration = "ProtoOaNotificationType", required, tag = "1")]
    pub margin_call_type: i32,
    /// Margin level threshold for margin call.
    #[prost(double, required, tag = "2")]
    pub margin_level_threshold: f64,
    /// The Unix time in milliseconds of the last update of the margin call.
    #[prost(int64, optional, tag = "3")]
    pub utc_last_update_timestamp: ::core::option::Option<i64>,
}
/// Request for authorizing an application to work with the cTrader platform Proxies.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaApplicationAuthReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaApplicationAuthReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The unique Client ID provided during the registration.
    #[prost(string, required, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    /// The unique Client Secret provided during the registration.
    #[prost(string, required, tag = "3")]
    pub client_secret: ::prost::alloc::string::String,
}
/// Response to the ProtoOAApplicationAuthReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaApplicationAuthRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaApplicationAuthRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
}
/// Request for the authorizing trading account session. Requires established authorized connection with the client application using ProtoOAApplicationAuthReq.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaAccountAuthReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaAccountAuthReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The unique identifier of the trading account in cTrader platform.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The Access Token issued for providing access to the trading account.
    #[prost(string, required, tag = "3")]
    pub access_token: ::prost::alloc::string::String,
}
/// Response to the ProtoOAApplicationAuthRes request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaAccountAuthRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaAccountAuthRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The unique identifier of the trading account in cTrader platform.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// Generic response when an ERROR occurred.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaErrorRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaErrorRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The unique identifier of the trading account in cTrader platform.
    #[prost(int64, optional, tag = "2")]
    pub ctid_trader_account_id: ::core::option::Option<i64>,
    /// The name of the ProtoErrorCode or the other custom ErrorCodes (e.g. ProtoCHErrorCode).
    #[prost(string, required, tag = "3")]
    pub error_code: ::prost::alloc::string::String,
    /// The error description.
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// The Unix time in seconds when the current maintenance session will be ended.
    #[prost(int64, optional, tag = "5")]
    pub maintenance_end_timestamp: ::core::option::Option<i64>,
}
/// Event that is sent when the connection with the client application is cancelled by the server.
/// All the sessions for the trader accounts will be terminated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaClientDisconnectEvent {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaClientDisconnectEvent"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The disconnection reason explained. For example: The application access was blocked by cTrader Administrator.
    #[prost(string, optional, tag = "2")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
}
/// Event that is sent when a session to a specific trader account is terminated by the server but the existing connections with the other trader accounts are maintained.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaAccountsTokenInvalidatedEvent {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaAccountsTokenInvalidatedEvent"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The unique identifier of the trading account in cTrader platform.
    #[prost(int64, repeated, packed = "false", tag = "2")]
    pub ctid_trader_account_ids: ::prost::alloc::vec::Vec<i64>,
    /// The disconnection reason explained. For example: Access Token is expired or recalled.
    #[prost(string, optional, tag = "3")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
}
/// Request for getting the proxy version. Can be used to check the current version of the Open API scheme.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaVersionReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaVersionReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
}
/// Response to the ProtoOAVersionReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaVersionRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaVersionRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The current version of the server application.
    #[prost(string, required, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// Request for sending a new trading order.
/// Allowed only if the accessToken has the "trade" permissions for the trading account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaNewOrderReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaNewOrderReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The unique identifier of the trading account in cTrader platform.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique identifier of a symbol in cTrader platform.
    #[prost(int64, required, tag = "3")]
    pub symbol_id: i64,
    /// The type of an order - MARKET, LIMIT, STOP, MARKET_RANGE, STOP_LIMIT.
    #[prost(enumeration = "ProtoOaOrderType", required, tag = "4")]
    pub order_type: i32,
    /// The trade direction - BUY or SELL.
    #[prost(enumeration = "ProtoOaTradeSide", required, tag = "5")]
    pub trade_side: i32,
    /// The volume represented in 0.01 of a unit (e.g. US$ 10.00 = 1000).
    #[prost(int64, required, tag = "6")]
    pub volume: i64,
    /// The limit price, can be specified for the LIMIT order only.
    #[prost(double, optional, tag = "7")]
    pub limit_price: ::core::option::Option<f64>,
    /// Stop Price, can be specified for the STOP and the STOP_LIMIT orders only.
    #[prost(double, optional, tag = "8")]
    pub stop_price: ::core::option::Option<f64>,
    /// The specific order execution or expiration instruction - GOOD_TILL_DATE, GOOD_TILL_CANCEL, IMMEDIATE_OR_CANCEL, FILL_OR_KILL, MARKET_ON_OPEN.
    #[prost(
        enumeration = "ProtoOaTimeInForce",
        optional,
        tag = "9",
        default = "GoodTillCancel"
    )]
    pub time_in_force: ::core::option::Option<i32>,
    /// The Unix time in milliseconds of Order expiration time. Should be set for the Good Till Date orders.
    #[prost(int64, optional, tag = "10")]
    pub expiration_timestamp: ::core::option::Option<i64>,
    /// The absolute Stop Loss price (1.23456 for example). Unsupported for MARKET orders; for these orders, please use relativeStopLoss.
    #[prost(double, optional, tag = "11")]
    pub stop_loss: ::core::option::Option<f64>,
    /// The absolute Take Profit price (1.23456 for example). Unsupported for MARKET orders; for these orders, please use relativeTakeProfit.
    #[prost(double, optional, tag = "12")]
    pub take_profit: ::core::option::Option<f64>,
    /// User-specified comment. MaxLength = 512.
    #[prost(string, optional, tag = "13")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    /// Base price to calculate relative slippage price for MARKET_RANGE order.
    #[prost(double, optional, tag = "14")]
    pub base_slippage_price: ::core::option::Option<f64>,
    /// Slippage distance for MARKET_RANGE and STOP_LIMIT order.
    #[prost(int32, optional, tag = "15")]
    pub slippage_in_points: ::core::option::Option<i32>,
    /// User-specified label. MaxLength = 100.
    #[prost(string, optional, tag = "16")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    /// Reference to the existing position if the Order is intended to modify it.
    #[prost(int64, optional, tag = "17")]
    pub position_id: ::core::option::Option<i64>,
    /// Optional user-specific clientOrderId (similar to FIX ClOrderID). MaxLength = 50.
    #[prost(string, optional, tag = "18")]
    pub client_order_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Relative Stop Loss that can be specified instead of the absolute as one. Specified in 1/100000 of unit of a price. For BUY stopLoss = entryPrice - relativeStopLoss, for SELL stopLoss = entryPrice + relativeStopLoss.
    #[prost(int64, optional, tag = "19")]
    pub relative_stop_loss: ::core::option::Option<i64>,
    /// Relative Take Profit that can be specified instead of the absolute one. Specified in 1/100000 of unit of a price. For BUY takeProfit = entryPrice + relativeTakeProfit, for SELL takeProfit = entryPrice - relativeTakeProfit.
    #[prost(int64, optional, tag = "20")]
    pub relative_take_profit: ::core::option::Option<i64>,
    /// If TRUE then stopLoss is guaranteed. Available for the French Risk or the Guaranteed Stop Loss Accounts.
    #[prost(bool, optional, tag = "21")]
    pub guaranteed_stop_loss: ::core::option::Option<bool>,
    /// If TRUE then the Stop Loss is Trailing.
    #[prost(bool, optional, tag = "22")]
    pub trailing_stop_loss: ::core::option::Option<bool>,
    /// Trigger method for the STOP or the STOP_LIMIT pending order.
    #[prost(
        enumeration = "ProtoOaOrderTriggerMethod",
        optional,
        tag = "23",
        default = "Trade"
    )]
    pub stop_trigger_method: ::core::option::Option<i32>,
}
/// Event that is sent following the successful order acceptance or execution by the server
/// Acts as response to the ProtoOANewOrderReq, ProtoOACancelOrderReq, ProtoOAAmendOrderReq, ProtoOAAmendPositionSLTPReq, ProtoOAClosePositionReq requests.
/// Also, the event is sent when a Deposit/Withdrawal took place.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaExecutionEvent {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaExecutionEvent"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Type of the order operation. For example: ACCEPTED, FILLED, etc.
    #[prost(enumeration = "ProtoOaExecutionType", required, tag = "3")]
    pub execution_type: i32,
    /// Reference to the position linked with the execution
    #[prost(message, optional, tag = "4")]
    pub position: ::core::option::Option<ProtoOaPosition>,
    /// Reference to the initial order. This field will not appear if `executionType = 9, 10, 12`.
    #[prost(message, optional, tag = "5")]
    pub order: ::core::option::Option<ProtoOaOrder>,
    /// Reference to the deal (execution).
    #[prost(message, optional, tag = "6")]
    pub deal: ::core::option::Option<ProtoOaDeal>,
    /// Reference to the Bonus Deposit or Withdrawal operation.
    #[prost(message, optional, tag = "7")]
    pub bonus_deposit_withdraw: ::core::option::Option<ProtoOaBonusDepositWithdraw>,
    /// Reference to the Deposit or Withdrawal operation.
    #[prost(message, optional, tag = "8")]
    pub deposit_withdraw: ::core::option::Option<ProtoOaDepositWithdraw>,
    /// The name of the ProtoErrorCode or the other custom ErrorCodes (e.g. ProtoCHErrorCode).
    #[prost(string, optional, tag = "9")]
    pub error_code: ::core::option::Option<::prost::alloc::string::String>,
    /// If TRUE then the event is generated by the server logic instead of the trader request. (e.g. stop-out).
    #[prost(bool, optional, tag = "10")]
    pub is_server_event: ::core::option::Option<bool>,
}
/// Request for cancelling existing pending order.
/// Allowed only if the accessToken has "trade" permissions for the trading account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaCancelOrderReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaCancelOrderReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the order.
    #[prost(int64, required, tag = "3")]
    pub order_id: i64,
}
/// Request for amending the existing pending order.
/// Allowed only if the Access Token has "trade" permissions for the trading account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaAmendOrderReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaAmendOrderReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the order.
    #[prost(int64, required, tag = "3")]
    pub order_id: i64,
    /// Volume, represented in 0.01 of a unit (e.g. cents).
    #[prost(int64, optional, tag = "4")]
    pub volume: ::core::option::Option<i64>,
    /// The Limit Price, can be specified for the LIMIT order only.
    #[prost(double, optional, tag = "5")]
    pub limit_price: ::core::option::Option<f64>,
    /// The Stop Price, can be specified for the STOP and the STOP_LIMIT orders.
    #[prost(double, optional, tag = "6")]
    pub stop_price: ::core::option::Option<f64>,
    /// The Unix timestamp in milliseconds of Order expiration time. Should be set for the Good Till Date orders.
    #[prost(int64, optional, tag = "7")]
    pub expiration_timestamp: ::core::option::Option<i64>,
    /// The absolute Stop Loss price (e.g. 1.23456). Not supported for the MARKET orders.
    #[prost(double, optional, tag = "8")]
    pub stop_loss: ::core::option::Option<f64>,
    /// The absolute Take Profit price (e.g. 1.23456). Not supported for the MARKET orders.
    #[prost(double, optional, tag = "9")]
    pub take_profit: ::core::option::Option<f64>,
    /// Slippage distance for the MARKET_RANGE and the STOP_LIMIT orders.
    #[prost(int32, optional, tag = "10")]
    pub slippage_in_points: ::core::option::Option<i32>,
    /// The relative Stop Loss can be specified instead of the absolute one. Specified in 1/100000 of a unit of price. For BUY stopLoss = entryPrice - relativeStopLoss, for SELL stopLoss = entryPrice + relativeStopLoss.
    #[prost(int64, optional, tag = "11")]
    pub relative_stop_loss: ::core::option::Option<i64>,
    /// The relative Take Profit can be specified instead of the absolute one. Specified in 1/100000 of a unit of price. For BUY takeProfit = entryPrice + relativeTakeProfit, for SELL takeProfit = entryPrice - relativeTakeProfit.
    #[prost(int64, optional, tag = "12")]
    pub relative_take_profit: ::core::option::Option<i64>,
    /// If TRUE then the Stop Loss is guaranteed. Available for the French Risk or the Guaranteed Stop Loss Accounts.
    #[prost(bool, optional, tag = "13")]
    pub guaranteed_stop_loss: ::core::option::Option<bool>,
    /// If TRUE then the Trailing Stop Loss is applied.
    #[prost(bool, optional, tag = "14")]
    pub trailing_stop_loss: ::core::option::Option<bool>,
    /// Trigger method for the STOP or the STOP_LIMIT pending order.
    #[prost(
        enumeration = "ProtoOaOrderTriggerMethod",
        optional,
        tag = "15",
        default = "Trade"
    )]
    pub stop_trigger_method: ::core::option::Option<i32>,
}
/// Request for amending StopLoss and TakeProfit of existing position.
/// Allowed only if the accessToken has "trade" permissions for the trading account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaAmendPositionSltpReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaAmendPositionSltpReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the position to amend.
    #[prost(int64, required, tag = "3")]
    pub position_id: i64,
    /// Absolute Stop Loss price (1.23456 for example).
    #[prost(double, optional, tag = "4")]
    pub stop_loss: ::core::option::Option<f64>,
    /// Absolute Take Profit price (1.26543 for example).
    #[prost(double, optional, tag = "5")]
    pub take_profit: ::core::option::Option<f64>,
    /// If TRUE then the Stop Loss is guaranteed. Available for the French Risk or the Guaranteed Stop Loss Accounts.
    #[prost(bool, optional, tag = "7")]
    pub guaranteed_stop_loss: ::core::option::Option<bool>,
    /// If TRUE then the Trailing Stop Loss is applied.
    #[prost(bool, optional, tag = "8")]
    pub trailing_stop_loss: ::core::option::Option<bool>,
    /// The Stop trigger method for the Stop Loss/Take Profit order.
    #[prost(
        enumeration = "ProtoOaOrderTriggerMethod",
        optional,
        tag = "9",
        default = "Trade"
    )]
    pub stop_loss_trigger_method: ::core::option::Option<i32>,
}
/// Request for closing or partially closing of an existing position.
/// Allowed only if the accessToken has "trade" permissions for the trading account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaClosePositionReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaClosePositionReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the position to close.
    #[prost(int64, required, tag = "3")]
    pub position_id: i64,
    /// Volume to close, represented in 0.01 of a unit (e.g. cents).
    #[prost(int64, required, tag = "4")]
    pub volume: i64,
}
/// Event that is sent when the level of the Trailing Stop Loss is changed due to the price level changes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaTrailingSlChangedEvent {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaTrailingSlChangedEvent"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the position.
    #[prost(int64, required, tag = "3")]
    pub position_id: i64,
    /// The unique ID of the order.
    #[prost(int64, required, tag = "4")]
    pub order_id: i64,
    /// New value of the Stop Loss price.
    #[prost(double, required, tag = "5")]
    pub stop_price: f64,
    /// The Unix timestamp in milliseconds when the Stop Loss was updated.
    #[prost(int64, required, tag = "6")]
    pub utc_last_update_timestamp: i64,
}
/// Request for the list of assets available for a trading account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaAssetListReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaAssetListReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// Response to the ProtoOAAssetListReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaAssetListRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaAssetListRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The list of assets.
    #[prost(message, repeated, tag = "3")]
    pub asset: ::prost::alloc::vec::Vec<ProtoOaAsset>,
}
/// Request for a list of symbols available for a trading account. Symbol entries are returned with the limited set of fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSymbolsListReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSymbolsListReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Whether to include old archived symbols into response.
    #[prost(bool, optional, tag = "3", default = "false")]
    pub include_archived_symbols: ::core::option::Option<bool>,
}
/// Response to the ProtoOASymbolsListReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSymbolsListRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSymbolsListRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The list of symbols.
    #[prost(message, repeated, tag = "3")]
    pub symbol: ::prost::alloc::vec::Vec<ProtoOaLightSymbol>,
    /// The list of archived symbols.
    #[prost(message, repeated, tag = "4")]
    pub archived_symbol: ::prost::alloc::vec::Vec<ProtoOaArchivedSymbol>,
}
/// Request for getting a full symbol entity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSymbolByIdReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSymbolByIdReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the symbol in cTrader platform.
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub symbol_id: ::prost::alloc::vec::Vec<i64>,
}
/// Response to the ProtoOASymbolByIdReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSymbolByIdRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSymbolByIdRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Symbol entity with the full set of fields.
    #[prost(message, repeated, tag = "3")]
    pub symbol: ::prost::alloc::vec::Vec<ProtoOaSymbol>,
    /// Archived symbols.
    #[prost(message, repeated, tag = "4")]
    pub archived_symbol: ::prost::alloc::vec::Vec<ProtoOaArchivedSymbol>,
}
/// Request for getting a conversion chain between two assets that consists of several symbols. Use when no direct quote is available
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSymbolsForConversionReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSymbolsForConversionReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The ID of the first asset in the conversation chain. e.g.: for EUR/USD the firstAssetId is EUR ID and lastAssetId is USD ID.
    #[prost(int64, required, tag = "3")]
    pub first_asset_id: i64,
    /// The ID of the last asset in the conversation chain. e.g.: for EUR/USD the firstAssetId is EUR ID and lastAssetId is USD ID.
    #[prost(int64, required, tag = "4")]
    pub last_asset_id: i64,
}
/// Response to the ProtoOASymbolsForConversionReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSymbolsForConversionRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSymbolsForConversionRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Conversion chain of the symbols (e.g. EUR/USD, USD/JPY, GBP/JPY -> EUR/GBP).
    #[prost(message, repeated, tag = "3")]
    pub symbol: ::prost::alloc::vec::Vec<ProtoOaLightSymbol>,
}
/// Event that is sent when the symbol is changed on the Server side.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSymbolChangedEvent {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSymbolChangedEvent"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub symbol_id: ::prost::alloc::vec::Vec<i64>,
}
/// Request for a list of asset classes available for the trading account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaAssetClassListReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaAssetClassListReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// Response to the ProtoOAAssetListReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaAssetClassListRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaAssetClassListRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// List of the asset classes.
    #[prost(message, repeated, tag = "3")]
    pub asset_class: ::prost::alloc::vec::Vec<ProtoOaAssetClass>,
}
/// Request for getting data of Trader Account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaTraderReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaTraderReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// Response to the ProtoOATraderReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaTraderRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaTraderRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The Trader account information.
    #[prost(message, required, tag = "3")]
    pub trader: ProtoOaTrader,
}
/// Event that is sent when a Trader is updated on Server side.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaTraderUpdatedEvent {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaTraderUpdateEvent"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The Trader account information.
    #[prost(message, required, tag = "3")]
    pub trader: ProtoOaTrader,
}
/// Request for getting Trader current open positions and pending orders data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaReconcileReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaReconcileReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// If TRUE, then current protection orders are returned separately, otherwise you can use position.stopLoss and position.takeProfit fields.
    #[prost(bool, optional, tag = "3")]
    pub return_protection_orders: ::core::option::Option<bool>,
}
/// The response to the ProtoOAReconcileReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaReconcileRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaReconcileRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The list of trading account open positions.
    #[prost(message, repeated, tag = "3")]
    pub position: ::prost::alloc::vec::Vec<ProtoOaPosition>,
    /// The list of trading account pending orders.
    #[prost(message, repeated, tag = "4")]
    pub order: ::prost::alloc::vec::Vec<ProtoOaOrder>,
}
/// Event that is sent when errors occur during the order requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaOrderErrorEvent {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaOrderErrorEvent"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "5")]
    pub ctid_trader_account_id: i64,
    /// The name of the ProtoErrorCode or the other custom ErrorCodes (e.g. ProtoCHErrorCode).
    #[prost(string, required, tag = "2")]
    pub error_code: ::prost::alloc::string::String,
    /// The unique ID of the order.
    #[prost(int64, optional, tag = "3")]
    pub order_id: ::core::option::Option<i64>,
    /// The unique ID of the position.
    #[prost(int64, optional, tag = "6")]
    pub position_id: ::core::option::Option<i64>,
    /// The error description.
    #[prost(string, optional, tag = "7")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// Request for getting trading account deals historical data (execution details).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaDealListReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaDealListReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The UNIX time from which the search starts >=0 (1-1-1970). Validation: toTimestamp - fromTimestamp <= 604800000 (1 week).
    #[prost(int64, required, tag = "3")]
    pub from_timestamp: i64,
    /// The UNIX time where to stop searching <= 2147483646000 (19-1-2038).
    #[prost(int64, required, tag = "4")]
    pub to_timestamp: i64,
    /// The maximum number of the deals to return.
    #[prost(int32, optional, tag = "5")]
    pub max_rows: ::core::option::Option<i32>,
}
/// The response to the ProtoOADealListReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaDealListRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaDealListRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The list of the deals.
    #[prost(message, repeated, tag = "3")]
    pub deal: ::prost::alloc::vec::Vec<ProtoOaDeal>,
    /// If TRUE then the number of records by filter is larger than chunkSize, the response contains the number of records that is equal to chunkSize.
    #[prost(bool, required, tag = "4")]
    pub has_more: bool,
}
/// Request for getting Trader's orders filtered by timestamp
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaOrderListReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaOrderListReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The Unix time from which the search starts >=0 (1st Jan 1970). Validation: toTimestamp - fromTimestamp <= 604800000 (1 week).
    #[prost(int64, required, tag = "3")]
    pub from_timestamp: i64,
    /// The Unix time where to stop searching <= 2147483646000 (19th Jan 2038).
    #[prost(int64, required, tag = "4")]
    pub to_timestamp: i64,
}
/// The response to the ProtoOAOrderListReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaOrderListRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaOrderListRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The list of the orders.
    #[prost(message, repeated, tag = "3")]
    pub order: ::prost::alloc::vec::Vec<ProtoOaOrder>,
    /// If TRUE then the number of records by filter is larger than chunkSize, the response contains the number of records that is equal to chunkSize.
    #[prost(bool, required, tag = "4")]
    pub has_more: bool,
}
/// Request for getting the margin estimate. Can be used before sending a new order request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaExpectedMarginReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaExpectedMarginReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, required, tag = "3")]
    pub symbol_id: i64,
    /// Volume represented in 0.01 of a unit (e.g. cents).
    #[prost(int64, repeated, packed = "false", tag = "4")]
    pub volume: ::prost::alloc::vec::Vec<i64>,
}
/// The response to the ProtoOAExpectedMarginReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaExpectedMarginRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaExpectedMarginRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The buy and sell margin estimate.
    #[prost(message, repeated, tag = "3")]
    pub margin: ::prost::alloc::vec::Vec<ProtoOaExpectedMargin>,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects usedMargin.
    #[prost(uint32, optional, tag = "4")]
    pub money_digits: ::core::option::Option<u32>,
}
/// Event that is sent when the margin allocated to a specific position is changed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaMarginChangedEvent {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaMarginChangedEvent"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the position.
    #[prost(uint64, required, tag = "3")]
    pub position_id: u64,
    /// The new value of the margin used.
    #[prost(uint64, required, tag = "4")]
    pub used_margin: u64,
    /// Specifies the exponent of the monetary values. E.g. moneyDigits = 8 must be interpret as business value multiplied by 10^8, then real balance would be 10053099944 / 10^8 = 100.53099944. Affects usedMargin.
    #[prost(uint32, optional, tag = "5")]
    pub money_digits: ::core::option::Option<u32>,
}
/// Request for getting trading account historical data of deposits and withdrawals.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaCashFlowHistoryListReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaCashFlowHistoryListReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The Unix time from which the search starts >=0 (1st Jan 1970). Validation: toTimestamp - fromTimestamp <= 604800000 (1 week).
    #[prost(int64, required, tag = "3")]
    pub from_timestamp: i64,
    /// The Unix time where to stop searching <= 2147483646000 (19th Jan 2038).
    #[prost(int64, required, tag = "4")]
    pub to_timestamp: i64,
}
/// Response to the ProtoOACashFlowHistoryListReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaCashFlowHistoryListRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaCashFlowHistoryListRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The list of deposit and withdrawal operations.
    #[prost(message, repeated, tag = "3")]
    pub deposit_withdraw: ::prost::alloc::vec::Vec<ProtoOaDepositWithdraw>,
}
/// Request for getting the list of granted trading account for the access token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaGetAccountListByAccessTokenReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaGetAccountsByAccessTokenReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The Access Token issued for providing access to the trading account.
    #[prost(string, required, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
}
/// Response to the ProtoOAGetAccountListByAccessTokenReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaGetAccountListByAccessTokenRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaGetAccountsByAccessTokenRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The Access Token issued for providing access to the trading account.
    #[prost(string, required, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
    /// SCOPE_VIEW, SCOPE_TRADE.
    #[prost(enumeration = "ProtoOaClientPermissionScope", optional, tag = "3")]
    pub permission_scope: ::core::option::Option<i32>,
    /// The list of the accounts.
    #[prost(message, repeated, tag = "4")]
    pub ctid_trader_account: ::prost::alloc::vec::Vec<ProtoOaCtidTraderAccount>,
}
/// Request for subscribing on spot events of the specified symbol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSubscribeSpotsReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSubscribeSpotsReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub symbol_id: ::prost::alloc::vec::Vec<i64>,
    /// If TRUE you will also receive the timestamp in ProtoOASpotEvent.
    #[prost(bool, optional, tag = "4")]
    pub subscribe_to_spot_timestamp: ::core::option::Option<bool>,
}
/// Response to the ProtoOASubscribeSpotsReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSubscribeSpotsRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSubscribeSpotsRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// Request for unsubscribing from the spot events of the specified symbol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaUnsubscribeSpotsReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaUnsubscribeSpotsReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub symbol_id: ::prost::alloc::vec::Vec<i64>,
}
/// Response to the ProtoOASubscribeSpotsRes request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaUnsubscribeSpotsRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaUnsubscribeSpotsRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// * Event that is sent when a new spot event is generated on the server side.
/// Requires subscription on the spot events, see ProtoOASubscribeSpotsReq.
/// First event, received after subscription will contain latest spot prices even if market is closed
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSpotEvent {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSpotEvent"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, required, tag = "3")]
    pub symbol_id: i64,
    /// Bid price. Specified in 1/100_000 of unit of a price. (e.g. 1.23 -> 123_000)
    #[prost(uint64, optional, tag = "4")]
    pub bid: ::core::option::Option<u64>,
    /// Ask price. Specified in 1/100_000 of unit of a price.
    #[prost(uint64, optional, tag = "5")]
    pub ask: ::core::option::Option<u64>,
    /// Returns live trend bar. Requires subscription on the trend bars.
    #[prost(message, repeated, tag = "6")]
    pub trendbar: ::prost::alloc::vec::Vec<ProtoOaTrendbar>,
    /// Last session close. Specified in 1/100_000 of unit of a price.
    #[prost(uint64, optional, tag = "7")]
    pub session_close: ::core::option::Option<u64>,
    /// The Unix timestamp for spot.
    #[prost(int64, optional, tag = "8")]
    pub timestamp: ::core::option::Option<i64>,
}
/// Request for subscribing for live trend bars.
/// Requires subscription on the spot events, see ProtoOASubscribeSpotsReq.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSubscribeLiveTrendbarReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSubscribeLiveTrendbarReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Specifies period of trend bar series (e.g. M1, M10, etc.).
    #[prost(enumeration = "ProtoOaTrendbarPeriod", required, tag = "3")]
    pub period: i32,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, required, tag = "4")]
    pub symbol_id: i64,
}
/// Response to the ProtoOASubscribeLiveTrendbarReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSubscribeLiveTrendbarRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSubscribeLiveTrendbarRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// Request for unsubscribing from the live trend bars.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaUnsubscribeLiveTrendbarReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaUnsubscribeLiveTrendbarReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Specifies period of trend bar series (e.g. M1, M10, etc.).
    #[prost(enumeration = "ProtoOaTrendbarPeriod", required, tag = "3")]
    pub period: i32,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, required, tag = "4")]
    pub symbol_id: i64,
}
/// Response to the ProtoOASubscribeLiveTrendbarReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaUnsubscribeLiveTrendbarRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaUnsubscribeLiveTrendbarRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// Request for getting historical trend bars for the symbol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaGetTrendbarsReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaGetTrendbarsReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The exact time of starting the search in milliseconds. Must be bigger or equal to zero (1-1-1970). Validation: toTimestamp - fromTimestamp <= X, where X depends on series period: M1, M2, M3, M4, M5: 3024000000 (5 weeks); M10, M15, M30, H1: 21168000000 (35 weeks), H4, H12, D1: 31622400000 (1 year); W1, MN1: 158112000000 (5 years).
    #[prost(int64, required, tag = "3")]
    pub from_timestamp: i64,
    /// The exact time of finishing the search in milliseconds. Smaller or equal to 2147483646000 (19-1-2038).
    #[prost(int64, required, tag = "4")]
    pub to_timestamp: i64,
    /// Specifies period of trend bar series (e.g. M1, M10, etc.).
    #[prost(enumeration = "ProtoOaTrendbarPeriod", required, tag = "5")]
    pub period: i32,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, required, tag = "6")]
    pub symbol_id: i64,
    /// Limit number of trend bars in response back from toTimestamp.
    #[prost(uint32, optional, tag = "7")]
    pub count: ::core::option::Option<u32>,
}
/// Response to the ProtoOAGetTrendbarsReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaGetTrendbarsRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaGetTrendbarsRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Specifies period of trend bar series (e.g. M1, M10, etc.).
    #[prost(enumeration = "ProtoOaTrendbarPeriod", required, tag = "3")]
    pub period: i32,
    /// Equals to toTimestamp from the request.
    #[prost(int64, required, tag = "4")]
    pub timestamp: i64,
    /// The list of trend bars.
    #[prost(message, repeated, tag = "5")]
    pub trendbar: ::prost::alloc::vec::Vec<ProtoOaTrendbar>,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, optional, tag = "6")]
    pub symbol_id: ::core::option::Option<i64>,
}
/// Request for getting historical tick data for the symbol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaGetTickDataReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaGetTickdataReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, required, tag = "3")]
    pub symbol_id: i64,
    /// Bid/Ask (1/2).
    #[prost(enumeration = "ProtoOaQuoteType", required, tag = "4")]
    pub r#type: i32,
    /// The Unix time in milliseconds of starting the search. Must be bigger or equal to zero (1st Jan 1970). Validation: toTimestamp - fromTimestamp <= 604800000 (1 week).
    #[prost(int64, required, tag = "5")]
    pub from_timestamp: i64,
    /// The Unix time in milliseconds of finishing the search. <= 2147483646000 (19th Jan 2038).
    #[prost(int64, required, tag = "6")]
    pub to_timestamp: i64,
}
/// Response to the ProtoOAGetTickDataReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaGetTickDataRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaGetTickdataRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// TThe list of ticks is in chronological order. The first tick contains Unix timestamp in milliseconds while all subsequent ticks have the time difference in milliseconds between the previous and the current one.
    #[prost(message, repeated, tag = "3")]
    pub tick_data: ::prost::alloc::vec::Vec<ProtoOaTickData>,
    /// If TRUE then the number of records by filter is larger than chunkSize, the response contains the number of records that is equal to chunkSize.
    #[prost(bool, required, tag = "4")]
    pub has_more: bool,
}
/// Request for getting trader profile details. Limited due to GDRP requirements.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaGetCtidProfileByTokenReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaGetCtidProfileByTokenReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The Access Token issued for providing access to the trading account.
    #[prost(string, required, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
}
/// Response to the ProtoOAGetCtidProfileByTokenReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaGetCtidProfileByTokenRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaGetCtidProfileByTokenRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Trader profile.
    #[prost(message, required, tag = "2")]
    pub profile: ProtoOaCtidProfile,
}
/// Event that is sent when the structure of depth of market is changed.
/// Requires subscription on the depth of markets for the symbol, see ProtoOASubscribeDepthQuotesReq.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaDepthEvent {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaDepthEvent"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(uint64, required, tag = "3")]
    pub symbol_id: u64,
    /// The list of changes in the depth of market quotes.
    #[prost(message, repeated, tag = "4")]
    pub new_quotes: ::prost::alloc::vec::Vec<ProtoOaDepthQuote>,
    /// The list of quotes to delete.
    #[prost(uint64, repeated, tag = "5")]
    pub deleted_quotes: ::prost::alloc::vec::Vec<u64>,
}
/// Request for subscribing on depth of market of the specified symbol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSubscribeDepthQuotesReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSubscribeDepthQuotesReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub symbol_id: ::prost::alloc::vec::Vec<i64>,
}
/// Response to the ProtoOASubscribeDepthQuotesReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSubscribeDepthQuotesRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSubscribeDepthQuotesRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// Request for unsubscribing from the depth of market of the specified symbol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaUnsubscribeDepthQuotesReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaUnsubscribeDepthQuotesReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Unique identifier of the Symbol in cTrader platform.
    #[prost(int64, repeated, packed = "false", tag = "3")]
    pub symbol_id: ::prost::alloc::vec::Vec<i64>,
}
/// Response to the ProtoOAUnsubscribeDepthQuotesReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaUnsubscribeDepthQuotesRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaUnsubscribeDepthQuotesRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// Request for a list of symbol categories available for a trading account
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSymbolCategoryListReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSymbolCategoryReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// Response to the ProtoSymbolCategoryListReq request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaSymbolCategoryListRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaSymbolCategoryRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trading account. Used to match responses to trading accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The list of symbol categories.
    #[prost(message, repeated, tag = "3")]
    pub symbol_category: ::prost::alloc::vec::Vec<ProtoOaSymbolCategory>,
}
/// Request for logout of trading account session.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaAccountLogoutReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaAccountLogoutReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The unique identifier of the trading account in cTrader platform.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// Response to the ProtoOATraderLogoutReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaAccountLogoutRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaAccountLogoutRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The unique identifier of the trading account in cTrader platform.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// Event that is sent when the established session for an account is dropped on the server side. A new session must be authorized for the account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaAccountDisconnectEvent {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaAccountDisconnectEvent"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The unique identifier of the trading account in cTrader platform.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// Request for a list of existing margin call thresholds configured for a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaMarginCallListReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaMarginCallListReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// Response with a list of existing user Margin Calls, usually contains 3 items.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaMarginCallListRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaMarginCallListRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub margin_call: ::prost::alloc::vec::Vec<ProtoOaMarginCall>,
}
/// Request to modify marginLevelThreshold of specified marginCallType for ctidTraderAccountId.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaMarginCallUpdateReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaMarginCallUpdateReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    #[prost(message, required, tag = "3")]
    pub margin_call: ProtoOaMarginCall,
}
/// If this response received, it means that margin call was successfully updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaMarginCallUpdateRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaMarginCallUpdateRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
}
/// Event that is sent when a Margin Call threshold configuration is updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaMarginCallUpdateEvent {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaMarginCallUpdateEvent"
    )]
    pub payload_type: ::core::option::Option<i32>,
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    #[prost(message, required, tag = "3")]
    pub margin_call: ProtoOaMarginCall,
}
/// Event that is sent when account margin level reaches target marginLevelThreshold. Event is sent no more than once every 10 minutes to avoid spamming.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaMarginCallTriggerEvent {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaMarginCallTriggerEvent"
    )]
    pub payload_type: ::core::option::Option<i32>,
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    #[prost(message, required, tag = "3")]
    pub margin_call: ProtoOaMarginCall,
}
/// * Request for getting a dynamic leverage entity referenced in ProtoOASymbol.leverageId.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaGetDynamicLeverageByIdReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaGetDynamicLeverageReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    #[prost(int64, required, tag = "3")]
    pub leverage_id: i64,
}
/// * Response to the ProtoOAGetDynamicLeverageByIDReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaGetDynamicLeverageByIdRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaGetDynamicLeverageRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    #[prost(message, required, tag = "3")]
    pub leverage: ProtoOaDynamicLeverage,
}
/// Request to refresh the access token using refresh token of granted trader account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaRefreshTokenReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaRefreshTokenReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The Refresh Token issued for updating Access Token.
    #[prost(string, required, tag = "2")]
    pub refresh_token: ::prost::alloc::string::String,
}
/// Response to the ProtoOARefreshTokenReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaRefreshTokenRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaRefreshTokenRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The Access Token issued for providing access to the Trader's Account.
    #[prost(string, required, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
    /// bearer
    #[prost(string, required, tag = "3")]
    pub token_type: ::prost::alloc::string::String,
    /// Access Token expiration in seconds.
    #[prost(int64, required, tag = "4")]
    pub expires_in: i64,
    /// Your new Refresh Token.
    #[prost(string, required, tag = "5")]
    pub refresh_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaHoliday {
    /// Unique ID of holiday.
    #[prost(int64, required, tag = "1")]
    pub holiday_id: i64,
    /// Name of holiday.
    #[prost(string, required, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Description of holiday.
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Timezone used for holiday.
    #[prost(string, required, tag = "4")]
    pub schedule_time_zone: ::prost::alloc::string::String,
    /// Amount of days from 1st Jan 1970, multiply it by 86400000 to get Unix time in milliseconds.
    #[prost(int64, required, tag = "5")]
    pub holiday_date: i64,
    /// If TRUE, then the holiday happens each year.
    #[prost(bool, required, tag = "6")]
    pub is_recurring: bool,
    /// Amount of seconds from 00:00:00 of the holiday day when holiday actually starts.
    #[prost(int32, optional, tag = "7")]
    pub start_second: ::core::option::Option<i32>,
    /// Amount of seconds from 00:00:00 of the holiday day when holiday actually finishes.
    #[prost(int32, optional, tag = "8")]
    pub end_second: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaDynamicLeverage {
    /// Unique ID of dynamic leverage.
    #[prost(int64, required, tag = "1")]
    pub leverage_id: i64,
    /// Tiers sorted by volume. Last tier's leverage is applied also to volume above specified.
    #[prost(message, repeated, tag = "2")]
    pub tiers: ::prost::alloc::vec::Vec<ProtoOaDynamicLeverageTier>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaDynamicLeverageTier {
    /// Max USD volume (in cents) of the Open Position (per side) to apply specified leverage. Last tier's leverage is applied also to volume above specified.
    #[prost(int64, required, tag = "1")]
    pub volume: i64,
    /// Applied leverage.
    #[prost(int32, required, tag = "2")]
    pub leverage: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaPositionUnrealizedPnL {
    /// The position ID.
    #[prost(int64, required, tag = "1")]
    pub position_id: i64,
    /// The gross unrealized PnL of the position denoted in the account deposit currency.
    #[prost(int64, required, tag = "2")]
    pub gross_unrealized_pn_l: i64,
    /// The net unrealized PnL of the position denoted in the account deposit currency. It does not include potential closing commission.
    #[prost(int64, required, tag = "3")]
    pub net_unrealized_pn_l: i64,
}
/// Request for retrieving the deals related to a position.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaDealListByPositionIdReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaDealListByPositionIdReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the position.
    #[prost(int64, required, tag = "3")]
    pub position_id: i64,
    /// The Unix time in milliseconds of starting the search. Must be bigger or equal to zero (1st Jan 1970). Validation: toTimestamp - fromTimestamp <= 604800000 (1 week).
    #[prost(int64, required, tag = "4")]
    pub from_timestamp: i64,
    /// The Unix time in milliseconds of finishing the search. <= 2147483646000 (19th Jan 2038).
    #[prost(int64, required, tag = "5")]
    pub to_timestamp: i64,
}
/// Response to the ProtoOADealListByPositionIdReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaDealListByPositionIdRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaDealListByPositionIdRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The list of deals.
    #[prost(message, repeated, tag = "3")]
    pub deal: ::prost::alloc::vec::Vec<ProtoOaDeal>,
    /// If TRUE then the number of records by filter is larger than chunkSize, the response contains the number of records that is equal to chunkSize.
    #[prost(bool, required, tag = "4")]
    pub has_more: bool,
}
/// Request for getting Order and its related Deals.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaOrderDetailsReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaOrderDetailsReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the Order.
    #[prost(int64, required, tag = "3")]
    pub order_id: i64,
}
/// Response to the ProtoOAOrderDetailsReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaOrderDetailsRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaOrderDetailsRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Order details.
    #[prost(message, required, tag = "3")]
    pub order: ProtoOaOrder,
    /// All Deals created by filling the specified Order.
    #[prost(message, repeated, tag = "4")]
    pub deal: ::prost::alloc::vec::Vec<ProtoOaDeal>,
}
/// Request for retrieving Orders related to a Position by using Position ID. Filtered by utcLastUpdateTimestamp.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaOrderListByPositionIdReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaOrderListByPositionIdReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the Position.
    #[prost(int64, required, tag = "3")]
    pub position_id: i64,
    /// The Unix time from which the search starts >=0 (1st Jan 1970). Validation: toTimestamp - fromTimestamp <= 604800000 (1 week). Search by utcLastUpdateTimestamp of the Order.
    #[prost(int64, required, tag = "4")]
    pub from_timestamp: i64,
    /// The Unix time where to stop searching <= 2147483646000 (19th Jan 2038). Search by utcLastUpdateTimestamp of the Order.
    #[prost(int64, required, tag = "5")]
    pub to_timestamp: i64,
}
/// Response to ProtoOAOrderListByPositionIdReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaOrderListByPositionIdRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaOrderListByPositionIdRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Orders related to the specified Position, sorted by utcLastUpdateTimestamp in descending order (newest first).
    #[prost(message, repeated, tag = "3")]
    pub order: ::prost::alloc::vec::Vec<ProtoOaOrder>,
    /// If TRUE then the number of records by filter is larger than chunkSize, the response contains the number of records that is equal to chunkSize.
    #[prost(bool, required, tag = "4")]
    pub has_more: bool,
}
/// Request for getting sets of Deals that were offset by a specific Deal and that are offsetting the Deal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaDealOffsetListReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaDealOffsetListReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// The unique ID of the Deal.
    #[prost(int64, required, tag = "3")]
    pub deal_id: i64,
}
/// Response for ProtoOADealOffsetListReq.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaDealOffsetListRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaDealOffsetListRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// Unique identifier of the trader's account. Used to match responses to trader's accounts.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Deals which closed the specified deal.
    #[prost(message, repeated, tag = "3")]
    pub offset_by: ::prost::alloc::vec::Vec<ProtoOaDealOffset>,
    /// Deals which were closed by the specified deal.
    #[prost(message, repeated, tag = "4")]
    pub offsetting: ::prost::alloc::vec::Vec<ProtoOaDealOffset>,
}
/// Request for getting trader's positions' unrealized PnLs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaGetPositionUnrealizedPnLReq {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaGetPositionUnrealizedPnlReq"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The unique identifier of the trader's account in cTrader platform.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
}
/// Response to ProtoOAGetPositionUnrealizedPnLReq request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoOaGetPositionUnrealizedPnLRes {
    #[prost(
        enumeration = "ProtoOaPayloadType",
        optional,
        tag = "1",
        default = "ProtoOaGetPositionUnrealizedPnlRes"
    )]
    pub payload_type: ::core::option::Option<i32>,
    /// The unique identifier of the trader's account in cTrader platform.
    #[prost(int64, required, tag = "2")]
    pub ctid_trader_account_id: i64,
    /// Information about trader's positions' unrealized PnLs.
    #[prost(message, repeated, tag = "3")]
    pub position_unrealized_pn_l: ::prost::alloc::vec::Vec<ProtoOaPositionUnrealizedPnL>,
    /// Specifies the exponent of various monetary values. E.g., moneyDigits = 8 should be interpreted as the value multiplied by 10^8 with the 'real' value equal to 10053099944 / 10^8 = 100.53099944. Affects positionUnrealizedPnL.grossUnrealizedPnL, positionUnrealizedPnL.netUnrealizedPnL.
    #[prost(uint32, required, tag = "4")]
    pub money_digits: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoPayloadType {
    ProtoMessage = 5,
    ErrorRes = 50,
    HeartbeatEvent = 51,
}
impl ProtoPayloadType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoPayloadType::ProtoMessage => "PROTO_MESSAGE",
            ProtoPayloadType::ErrorRes => "ERROR_RES",
            ProtoPayloadType::HeartbeatEvent => "HEARTBEAT_EVENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROTO_MESSAGE" => Some(Self::ProtoMessage),
            "ERROR_RES" => Some(Self::ErrorRes),
            "HEARTBEAT_EVENT" => Some(Self::HeartbeatEvent),
            _ => None,
        }
    }
}
/// COMMON error codes, 1 - 99
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoErrorCode {
    /// Generic error.
    UnknownError = 1,
    /// Message is not supported. Wrong message.
    UnsupportedMessage = 2,
    /// Generic error. Usually used when input value is not correct.
    InvalidRequest = 3,
    /// Deal execution has reached timeout and rejected.
    TimeoutError = 5,
    /// Generic error for requests by id.
    EntityNotFound = 6,
    /// Connection to Server is lost or not supported.
    CantRouteRequest = 7,
    /// Message is too large.
    FrameTooLong = 8,
    /// Market is closed.
    MarketClosed = 9,
    /// Order is blocked (e.g. under execution) and change cannot be applied.
    ConcurrentModification = 10,
    /// Message is blocked by server or rate limit is reached.
    BlockedPayloadType = 11,
}
impl ProtoErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoErrorCode::UnknownError => "UNKNOWN_ERROR",
            ProtoErrorCode::UnsupportedMessage => "UNSUPPORTED_MESSAGE",
            ProtoErrorCode::InvalidRequest => "INVALID_REQUEST",
            ProtoErrorCode::TimeoutError => "TIMEOUT_ERROR",
            ProtoErrorCode::EntityNotFound => "ENTITY_NOT_FOUND",
            ProtoErrorCode::CantRouteRequest => "CANT_ROUTE_REQUEST",
            ProtoErrorCode::FrameTooLong => "FRAME_TOO_LONG",
            ProtoErrorCode::MarketClosed => "MARKET_CLOSED",
            ProtoErrorCode::ConcurrentModification => "CONCURRENT_MODIFICATION",
            ProtoErrorCode::BlockedPayloadType => "BLOCKED_PAYLOAD_TYPE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_ERROR" => Some(Self::UnknownError),
            "UNSUPPORTED_MESSAGE" => Some(Self::UnsupportedMessage),
            "INVALID_REQUEST" => Some(Self::InvalidRequest),
            "TIMEOUT_ERROR" => Some(Self::TimeoutError),
            "ENTITY_NOT_FOUND" => Some(Self::EntityNotFound),
            "CANT_ROUTE_REQUEST" => Some(Self::CantRouteRequest),
            "FRAME_TOO_LONG" => Some(Self::FrameTooLong),
            "MARKET_CLOSED" => Some(Self::MarketClosed),
            "CONCURRENT_MODIFICATION" => Some(Self::ConcurrentModification),
            "BLOCKED_PAYLOAD_TYPE" => Some(Self::BlockedPayloadType),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaPayloadType {
    ProtoOaApplicationAuthReq = 2100,
    ProtoOaApplicationAuthRes = 2101,
    ProtoOaAccountAuthReq = 2102,
    ProtoOaAccountAuthRes = 2103,
    ProtoOaVersionReq = 2104,
    ProtoOaVersionRes = 2105,
    ProtoOaNewOrderReq = 2106,
    ProtoOaTrailingSlChangedEvent = 2107,
    ProtoOaCancelOrderReq = 2108,
    ProtoOaAmendOrderReq = 2109,
    ProtoOaAmendPositionSltpReq = 2110,
    ProtoOaClosePositionReq = 2111,
    ProtoOaAssetListReq = 2112,
    ProtoOaAssetListRes = 2113,
    ProtoOaSymbolsListReq = 2114,
    ProtoOaSymbolsListRes = 2115,
    ProtoOaSymbolByIdReq = 2116,
    ProtoOaSymbolByIdRes = 2117,
    ProtoOaSymbolsForConversionReq = 2118,
    ProtoOaSymbolsForConversionRes = 2119,
    ProtoOaSymbolChangedEvent = 2120,
    ProtoOaTraderReq = 2121,
    ProtoOaTraderRes = 2122,
    ProtoOaTraderUpdateEvent = 2123,
    ProtoOaReconcileReq = 2124,
    ProtoOaReconcileRes = 2125,
    ProtoOaExecutionEvent = 2126,
    ProtoOaSubscribeSpotsReq = 2127,
    ProtoOaSubscribeSpotsRes = 2128,
    ProtoOaUnsubscribeSpotsReq = 2129,
    ProtoOaUnsubscribeSpotsRes = 2130,
    ProtoOaSpotEvent = 2131,
    ProtoOaOrderErrorEvent = 2132,
    ProtoOaDealListReq = 2133,
    ProtoOaDealListRes = 2134,
    ProtoOaSubscribeLiveTrendbarReq = 2135,
    ProtoOaUnsubscribeLiveTrendbarReq = 2136,
    ProtoOaGetTrendbarsReq = 2137,
    ProtoOaGetTrendbarsRes = 2138,
    ProtoOaExpectedMarginReq = 2139,
    ProtoOaExpectedMarginRes = 2140,
    ProtoOaMarginChangedEvent = 2141,
    ProtoOaErrorRes = 2142,
    ProtoOaCashFlowHistoryListReq = 2143,
    ProtoOaCashFlowHistoryListRes = 2144,
    ProtoOaGetTickdataReq = 2145,
    ProtoOaGetTickdataRes = 2146,
    ProtoOaAccountsTokenInvalidatedEvent = 2147,
    ProtoOaClientDisconnectEvent = 2148,
    ProtoOaGetAccountsByAccessTokenReq = 2149,
    ProtoOaGetAccountsByAccessTokenRes = 2150,
    ProtoOaGetCtidProfileByTokenReq = 2151,
    ProtoOaGetCtidProfileByTokenRes = 2152,
    ProtoOaAssetClassListReq = 2153,
    ProtoOaAssetClassListRes = 2154,
    ProtoOaDepthEvent = 2155,
    ProtoOaSubscribeDepthQuotesReq = 2156,
    ProtoOaSubscribeDepthQuotesRes = 2157,
    ProtoOaUnsubscribeDepthQuotesReq = 2158,
    ProtoOaUnsubscribeDepthQuotesRes = 2159,
    ProtoOaSymbolCategoryReq = 2160,
    ProtoOaSymbolCategoryRes = 2161,
    ProtoOaAccountLogoutReq = 2162,
    ProtoOaAccountLogoutRes = 2163,
    ProtoOaAccountDisconnectEvent = 2164,
    ProtoOaSubscribeLiveTrendbarRes = 2165,
    ProtoOaUnsubscribeLiveTrendbarRes = 2166,
    ProtoOaMarginCallListReq = 2167,
    ProtoOaMarginCallListRes = 2168,
    ProtoOaMarginCallUpdateReq = 2169,
    ProtoOaMarginCallUpdateRes = 2170,
    ProtoOaMarginCallUpdateEvent = 2171,
    ProtoOaMarginCallTriggerEvent = 2172,
    ProtoOaRefreshTokenReq = 2173,
    ProtoOaRefreshTokenRes = 2174,
    ProtoOaOrderListReq = 2175,
    ProtoOaOrderListRes = 2176,
    ProtoOaGetDynamicLeverageReq = 2177,
    ProtoOaGetDynamicLeverageRes = 2178,
    ProtoOaDealListByPositionIdReq = 2179,
    ProtoOaDealListByPositionIdRes = 2180,
    ProtoOaOrderDetailsReq = 2181,
    ProtoOaOrderDetailsRes = 2182,
    ProtoOaOrderListByPositionIdReq = 2183,
    ProtoOaOrderListByPositionIdRes = 2184,
    ProtoOaDealOffsetListReq = 2185,
    ProtoOaDealOffsetListRes = 2186,
    ProtoOaGetPositionUnrealizedPnlReq = 2187,
    ProtoOaGetPositionUnrealizedPnlRes = 2188,
}
impl ProtoOaPayloadType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaPayloadType::ProtoOaApplicationAuthReq => "PROTO_OA_APPLICATION_AUTH_REQ",
            ProtoOaPayloadType::ProtoOaApplicationAuthRes => "PROTO_OA_APPLICATION_AUTH_RES",
            ProtoOaPayloadType::ProtoOaAccountAuthReq => "PROTO_OA_ACCOUNT_AUTH_REQ",
            ProtoOaPayloadType::ProtoOaAccountAuthRes => "PROTO_OA_ACCOUNT_AUTH_RES",
            ProtoOaPayloadType::ProtoOaVersionReq => "PROTO_OA_VERSION_REQ",
            ProtoOaPayloadType::ProtoOaVersionRes => "PROTO_OA_VERSION_RES",
            ProtoOaPayloadType::ProtoOaNewOrderReq => "PROTO_OA_NEW_ORDER_REQ",
            ProtoOaPayloadType::ProtoOaTrailingSlChangedEvent => {
                "PROTO_OA_TRAILING_SL_CHANGED_EVENT"
            }
            ProtoOaPayloadType::ProtoOaCancelOrderReq => "PROTO_OA_CANCEL_ORDER_REQ",
            ProtoOaPayloadType::ProtoOaAmendOrderReq => "PROTO_OA_AMEND_ORDER_REQ",
            ProtoOaPayloadType::ProtoOaAmendPositionSltpReq => "PROTO_OA_AMEND_POSITION_SLTP_REQ",
            ProtoOaPayloadType::ProtoOaClosePositionReq => "PROTO_OA_CLOSE_POSITION_REQ",
            ProtoOaPayloadType::ProtoOaAssetListReq => "PROTO_OA_ASSET_LIST_REQ",
            ProtoOaPayloadType::ProtoOaAssetListRes => "PROTO_OA_ASSET_LIST_RES",
            ProtoOaPayloadType::ProtoOaSymbolsListReq => "PROTO_OA_SYMBOLS_LIST_REQ",
            ProtoOaPayloadType::ProtoOaSymbolsListRes => "PROTO_OA_SYMBOLS_LIST_RES",
            ProtoOaPayloadType::ProtoOaSymbolByIdReq => "PROTO_OA_SYMBOL_BY_ID_REQ",
            ProtoOaPayloadType::ProtoOaSymbolByIdRes => "PROTO_OA_SYMBOL_BY_ID_RES",
            ProtoOaPayloadType::ProtoOaSymbolsForConversionReq => {
                "PROTO_OA_SYMBOLS_FOR_CONVERSION_REQ"
            }
            ProtoOaPayloadType::ProtoOaSymbolsForConversionRes => {
                "PROTO_OA_SYMBOLS_FOR_CONVERSION_RES"
            }
            ProtoOaPayloadType::ProtoOaSymbolChangedEvent => "PROTO_OA_SYMBOL_CHANGED_EVENT",
            ProtoOaPayloadType::ProtoOaTraderReq => "PROTO_OA_TRADER_REQ",
            ProtoOaPayloadType::ProtoOaTraderRes => "PROTO_OA_TRADER_RES",
            ProtoOaPayloadType::ProtoOaTraderUpdateEvent => "PROTO_OA_TRADER_UPDATE_EVENT",
            ProtoOaPayloadType::ProtoOaReconcileReq => "PROTO_OA_RECONCILE_REQ",
            ProtoOaPayloadType::ProtoOaReconcileRes => "PROTO_OA_RECONCILE_RES",
            ProtoOaPayloadType::ProtoOaExecutionEvent => "PROTO_OA_EXECUTION_EVENT",
            ProtoOaPayloadType::ProtoOaSubscribeSpotsReq => "PROTO_OA_SUBSCRIBE_SPOTS_REQ",
            ProtoOaPayloadType::ProtoOaSubscribeSpotsRes => "PROTO_OA_SUBSCRIBE_SPOTS_RES",
            ProtoOaPayloadType::ProtoOaUnsubscribeSpotsReq => "PROTO_OA_UNSUBSCRIBE_SPOTS_REQ",
            ProtoOaPayloadType::ProtoOaUnsubscribeSpotsRes => "PROTO_OA_UNSUBSCRIBE_SPOTS_RES",
            ProtoOaPayloadType::ProtoOaSpotEvent => "PROTO_OA_SPOT_EVENT",
            ProtoOaPayloadType::ProtoOaOrderErrorEvent => "PROTO_OA_ORDER_ERROR_EVENT",
            ProtoOaPayloadType::ProtoOaDealListReq => "PROTO_OA_DEAL_LIST_REQ",
            ProtoOaPayloadType::ProtoOaDealListRes => "PROTO_OA_DEAL_LIST_RES",
            ProtoOaPayloadType::ProtoOaSubscribeLiveTrendbarReq => {
                "PROTO_OA_SUBSCRIBE_LIVE_TRENDBAR_REQ"
            }
            ProtoOaPayloadType::ProtoOaUnsubscribeLiveTrendbarReq => {
                "PROTO_OA_UNSUBSCRIBE_LIVE_TRENDBAR_REQ"
            }
            ProtoOaPayloadType::ProtoOaGetTrendbarsReq => "PROTO_OA_GET_TRENDBARS_REQ",
            ProtoOaPayloadType::ProtoOaGetTrendbarsRes => "PROTO_OA_GET_TRENDBARS_RES",
            ProtoOaPayloadType::ProtoOaExpectedMarginReq => "PROTO_OA_EXPECTED_MARGIN_REQ",
            ProtoOaPayloadType::ProtoOaExpectedMarginRes => "PROTO_OA_EXPECTED_MARGIN_RES",
            ProtoOaPayloadType::ProtoOaMarginChangedEvent => "PROTO_OA_MARGIN_CHANGED_EVENT",
            ProtoOaPayloadType::ProtoOaErrorRes => "PROTO_OA_ERROR_RES",
            ProtoOaPayloadType::ProtoOaCashFlowHistoryListReq => {
                "PROTO_OA_CASH_FLOW_HISTORY_LIST_REQ"
            }
            ProtoOaPayloadType::ProtoOaCashFlowHistoryListRes => {
                "PROTO_OA_CASH_FLOW_HISTORY_LIST_RES"
            }
            ProtoOaPayloadType::ProtoOaGetTickdataReq => "PROTO_OA_GET_TICKDATA_REQ",
            ProtoOaPayloadType::ProtoOaGetTickdataRes => "PROTO_OA_GET_TICKDATA_RES",
            ProtoOaPayloadType::ProtoOaAccountsTokenInvalidatedEvent => {
                "PROTO_OA_ACCOUNTS_TOKEN_INVALIDATED_EVENT"
            }
            ProtoOaPayloadType::ProtoOaClientDisconnectEvent => "PROTO_OA_CLIENT_DISCONNECT_EVENT",
            ProtoOaPayloadType::ProtoOaGetAccountsByAccessTokenReq => {
                "PROTO_OA_GET_ACCOUNTS_BY_ACCESS_TOKEN_REQ"
            }
            ProtoOaPayloadType::ProtoOaGetAccountsByAccessTokenRes => {
                "PROTO_OA_GET_ACCOUNTS_BY_ACCESS_TOKEN_RES"
            }
            ProtoOaPayloadType::ProtoOaGetCtidProfileByTokenReq => {
                "PROTO_OA_GET_CTID_PROFILE_BY_TOKEN_REQ"
            }
            ProtoOaPayloadType::ProtoOaGetCtidProfileByTokenRes => {
                "PROTO_OA_GET_CTID_PROFILE_BY_TOKEN_RES"
            }
            ProtoOaPayloadType::ProtoOaAssetClassListReq => "PROTO_OA_ASSET_CLASS_LIST_REQ",
            ProtoOaPayloadType::ProtoOaAssetClassListRes => "PROTO_OA_ASSET_CLASS_LIST_RES",
            ProtoOaPayloadType::ProtoOaDepthEvent => "PROTO_OA_DEPTH_EVENT",
            ProtoOaPayloadType::ProtoOaSubscribeDepthQuotesReq => {
                "PROTO_OA_SUBSCRIBE_DEPTH_QUOTES_REQ"
            }
            ProtoOaPayloadType::ProtoOaSubscribeDepthQuotesRes => {
                "PROTO_OA_SUBSCRIBE_DEPTH_QUOTES_RES"
            }
            ProtoOaPayloadType::ProtoOaUnsubscribeDepthQuotesReq => {
                "PROTO_OA_UNSUBSCRIBE_DEPTH_QUOTES_REQ"
            }
            ProtoOaPayloadType::ProtoOaUnsubscribeDepthQuotesRes => {
                "PROTO_OA_UNSUBSCRIBE_DEPTH_QUOTES_RES"
            }
            ProtoOaPayloadType::ProtoOaSymbolCategoryReq => "PROTO_OA_SYMBOL_CATEGORY_REQ",
            ProtoOaPayloadType::ProtoOaSymbolCategoryRes => "PROTO_OA_SYMBOL_CATEGORY_RES",
            ProtoOaPayloadType::ProtoOaAccountLogoutReq => "PROTO_OA_ACCOUNT_LOGOUT_REQ",
            ProtoOaPayloadType::ProtoOaAccountLogoutRes => "PROTO_OA_ACCOUNT_LOGOUT_RES",
            ProtoOaPayloadType::ProtoOaAccountDisconnectEvent => {
                "PROTO_OA_ACCOUNT_DISCONNECT_EVENT"
            }
            ProtoOaPayloadType::ProtoOaSubscribeLiveTrendbarRes => {
                "PROTO_OA_SUBSCRIBE_LIVE_TRENDBAR_RES"
            }
            ProtoOaPayloadType::ProtoOaUnsubscribeLiveTrendbarRes => {
                "PROTO_OA_UNSUBSCRIBE_LIVE_TRENDBAR_RES"
            }
            ProtoOaPayloadType::ProtoOaMarginCallListReq => "PROTO_OA_MARGIN_CALL_LIST_REQ",
            ProtoOaPayloadType::ProtoOaMarginCallListRes => "PROTO_OA_MARGIN_CALL_LIST_RES",
            ProtoOaPayloadType::ProtoOaMarginCallUpdateReq => "PROTO_OA_MARGIN_CALL_UPDATE_REQ",
            ProtoOaPayloadType::ProtoOaMarginCallUpdateRes => "PROTO_OA_MARGIN_CALL_UPDATE_RES",
            ProtoOaPayloadType::ProtoOaMarginCallUpdateEvent => "PROTO_OA_MARGIN_CALL_UPDATE_EVENT",
            ProtoOaPayloadType::ProtoOaMarginCallTriggerEvent => {
                "PROTO_OA_MARGIN_CALL_TRIGGER_EVENT"
            }
            ProtoOaPayloadType::ProtoOaRefreshTokenReq => "PROTO_OA_REFRESH_TOKEN_REQ",
            ProtoOaPayloadType::ProtoOaRefreshTokenRes => "PROTO_OA_REFRESH_TOKEN_RES",
            ProtoOaPayloadType::ProtoOaOrderListReq => "PROTO_OA_ORDER_LIST_REQ",
            ProtoOaPayloadType::ProtoOaOrderListRes => "PROTO_OA_ORDER_LIST_RES",
            ProtoOaPayloadType::ProtoOaGetDynamicLeverageReq => "PROTO_OA_GET_DYNAMIC_LEVERAGE_REQ",
            ProtoOaPayloadType::ProtoOaGetDynamicLeverageRes => "PROTO_OA_GET_DYNAMIC_LEVERAGE_RES",
            ProtoOaPayloadType::ProtoOaDealListByPositionIdReq => {
                "PROTO_OA_DEAL_LIST_BY_POSITION_ID_REQ"
            }
            ProtoOaPayloadType::ProtoOaDealListByPositionIdRes => {
                "PROTO_OA_DEAL_LIST_BY_POSITION_ID_RES"
            }
            ProtoOaPayloadType::ProtoOaOrderDetailsReq => "PROTO_OA_ORDER_DETAILS_REQ",
            ProtoOaPayloadType::ProtoOaOrderDetailsRes => "PROTO_OA_ORDER_DETAILS_RES",
            ProtoOaPayloadType::ProtoOaOrderListByPositionIdReq => {
                "PROTO_OA_ORDER_LIST_BY_POSITION_ID_REQ"
            }
            ProtoOaPayloadType::ProtoOaOrderListByPositionIdRes => {
                "PROTO_OA_ORDER_LIST_BY_POSITION_ID_RES"
            }
            ProtoOaPayloadType::ProtoOaDealOffsetListReq => "PROTO_OA_DEAL_OFFSET_LIST_REQ",
            ProtoOaPayloadType::ProtoOaDealOffsetListRes => "PROTO_OA_DEAL_OFFSET_LIST_RES",
            ProtoOaPayloadType::ProtoOaGetPositionUnrealizedPnlReq => {
                "PROTO_OA_GET_POSITION_UNREALIZED_PNL_REQ"
            }
            ProtoOaPayloadType::ProtoOaGetPositionUnrealizedPnlRes => {
                "PROTO_OA_GET_POSITION_UNREALIZED_PNL_RES"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROTO_OA_APPLICATION_AUTH_REQ" => Some(Self::ProtoOaApplicationAuthReq),
            "PROTO_OA_APPLICATION_AUTH_RES" => Some(Self::ProtoOaApplicationAuthRes),
            "PROTO_OA_ACCOUNT_AUTH_REQ" => Some(Self::ProtoOaAccountAuthReq),
            "PROTO_OA_ACCOUNT_AUTH_RES" => Some(Self::ProtoOaAccountAuthRes),
            "PROTO_OA_VERSION_REQ" => Some(Self::ProtoOaVersionReq),
            "PROTO_OA_VERSION_RES" => Some(Self::ProtoOaVersionRes),
            "PROTO_OA_NEW_ORDER_REQ" => Some(Self::ProtoOaNewOrderReq),
            "PROTO_OA_TRAILING_SL_CHANGED_EVENT" => Some(Self::ProtoOaTrailingSlChangedEvent),
            "PROTO_OA_CANCEL_ORDER_REQ" => Some(Self::ProtoOaCancelOrderReq),
            "PROTO_OA_AMEND_ORDER_REQ" => Some(Self::ProtoOaAmendOrderReq),
            "PROTO_OA_AMEND_POSITION_SLTP_REQ" => Some(Self::ProtoOaAmendPositionSltpReq),
            "PROTO_OA_CLOSE_POSITION_REQ" => Some(Self::ProtoOaClosePositionReq),
            "PROTO_OA_ASSET_LIST_REQ" => Some(Self::ProtoOaAssetListReq),
            "PROTO_OA_ASSET_LIST_RES" => Some(Self::ProtoOaAssetListRes),
            "PROTO_OA_SYMBOLS_LIST_REQ" => Some(Self::ProtoOaSymbolsListReq),
            "PROTO_OA_SYMBOLS_LIST_RES" => Some(Self::ProtoOaSymbolsListRes),
            "PROTO_OA_SYMBOL_BY_ID_REQ" => Some(Self::ProtoOaSymbolByIdReq),
            "PROTO_OA_SYMBOL_BY_ID_RES" => Some(Self::ProtoOaSymbolByIdRes),
            "PROTO_OA_SYMBOLS_FOR_CONVERSION_REQ" => Some(Self::ProtoOaSymbolsForConversionReq),
            "PROTO_OA_SYMBOLS_FOR_CONVERSION_RES" => Some(Self::ProtoOaSymbolsForConversionRes),
            "PROTO_OA_SYMBOL_CHANGED_EVENT" => Some(Self::ProtoOaSymbolChangedEvent),
            "PROTO_OA_TRADER_REQ" => Some(Self::ProtoOaTraderReq),
            "PROTO_OA_TRADER_RES" => Some(Self::ProtoOaTraderRes),
            "PROTO_OA_TRADER_UPDATE_EVENT" => Some(Self::ProtoOaTraderUpdateEvent),
            "PROTO_OA_RECONCILE_REQ" => Some(Self::ProtoOaReconcileReq),
            "PROTO_OA_RECONCILE_RES" => Some(Self::ProtoOaReconcileRes),
            "PROTO_OA_EXECUTION_EVENT" => Some(Self::ProtoOaExecutionEvent),
            "PROTO_OA_SUBSCRIBE_SPOTS_REQ" => Some(Self::ProtoOaSubscribeSpotsReq),
            "PROTO_OA_SUBSCRIBE_SPOTS_RES" => Some(Self::ProtoOaSubscribeSpotsRes),
            "PROTO_OA_UNSUBSCRIBE_SPOTS_REQ" => Some(Self::ProtoOaUnsubscribeSpotsReq),
            "PROTO_OA_UNSUBSCRIBE_SPOTS_RES" => Some(Self::ProtoOaUnsubscribeSpotsRes),
            "PROTO_OA_SPOT_EVENT" => Some(Self::ProtoOaSpotEvent),
            "PROTO_OA_ORDER_ERROR_EVENT" => Some(Self::ProtoOaOrderErrorEvent),
            "PROTO_OA_DEAL_LIST_REQ" => Some(Self::ProtoOaDealListReq),
            "PROTO_OA_DEAL_LIST_RES" => Some(Self::ProtoOaDealListRes),
            "PROTO_OA_SUBSCRIBE_LIVE_TRENDBAR_REQ" => Some(Self::ProtoOaSubscribeLiveTrendbarReq),
            "PROTO_OA_UNSUBSCRIBE_LIVE_TRENDBAR_REQ" => {
                Some(Self::ProtoOaUnsubscribeLiveTrendbarReq)
            }
            "PROTO_OA_GET_TRENDBARS_REQ" => Some(Self::ProtoOaGetTrendbarsReq),
            "PROTO_OA_GET_TRENDBARS_RES" => Some(Self::ProtoOaGetTrendbarsRes),
            "PROTO_OA_EXPECTED_MARGIN_REQ" => Some(Self::ProtoOaExpectedMarginReq),
            "PROTO_OA_EXPECTED_MARGIN_RES" => Some(Self::ProtoOaExpectedMarginRes),
            "PROTO_OA_MARGIN_CHANGED_EVENT" => Some(Self::ProtoOaMarginChangedEvent),
            "PROTO_OA_ERROR_RES" => Some(Self::ProtoOaErrorRes),
            "PROTO_OA_CASH_FLOW_HISTORY_LIST_REQ" => Some(Self::ProtoOaCashFlowHistoryListReq),
            "PROTO_OA_CASH_FLOW_HISTORY_LIST_RES" => Some(Self::ProtoOaCashFlowHistoryListRes),
            "PROTO_OA_GET_TICKDATA_REQ" => Some(Self::ProtoOaGetTickdataReq),
            "PROTO_OA_GET_TICKDATA_RES" => Some(Self::ProtoOaGetTickdataRes),
            "PROTO_OA_ACCOUNTS_TOKEN_INVALIDATED_EVENT" => {
                Some(Self::ProtoOaAccountsTokenInvalidatedEvent)
            }
            "PROTO_OA_CLIENT_DISCONNECT_EVENT" => Some(Self::ProtoOaClientDisconnectEvent),
            "PROTO_OA_GET_ACCOUNTS_BY_ACCESS_TOKEN_REQ" => {
                Some(Self::ProtoOaGetAccountsByAccessTokenReq)
            }
            "PROTO_OA_GET_ACCOUNTS_BY_ACCESS_TOKEN_RES" => {
                Some(Self::ProtoOaGetAccountsByAccessTokenRes)
            }
            "PROTO_OA_GET_CTID_PROFILE_BY_TOKEN_REQ" => Some(Self::ProtoOaGetCtidProfileByTokenReq),
            "PROTO_OA_GET_CTID_PROFILE_BY_TOKEN_RES" => Some(Self::ProtoOaGetCtidProfileByTokenRes),
            "PROTO_OA_ASSET_CLASS_LIST_REQ" => Some(Self::ProtoOaAssetClassListReq),
            "PROTO_OA_ASSET_CLASS_LIST_RES" => Some(Self::ProtoOaAssetClassListRes),
            "PROTO_OA_DEPTH_EVENT" => Some(Self::ProtoOaDepthEvent),
            "PROTO_OA_SUBSCRIBE_DEPTH_QUOTES_REQ" => Some(Self::ProtoOaSubscribeDepthQuotesReq),
            "PROTO_OA_SUBSCRIBE_DEPTH_QUOTES_RES" => Some(Self::ProtoOaSubscribeDepthQuotesRes),
            "PROTO_OA_UNSUBSCRIBE_DEPTH_QUOTES_REQ" => Some(Self::ProtoOaUnsubscribeDepthQuotesReq),
            "PROTO_OA_UNSUBSCRIBE_DEPTH_QUOTES_RES" => Some(Self::ProtoOaUnsubscribeDepthQuotesRes),
            "PROTO_OA_SYMBOL_CATEGORY_REQ" => Some(Self::ProtoOaSymbolCategoryReq),
            "PROTO_OA_SYMBOL_CATEGORY_RES" => Some(Self::ProtoOaSymbolCategoryRes),
            "PROTO_OA_ACCOUNT_LOGOUT_REQ" => Some(Self::ProtoOaAccountLogoutReq),
            "PROTO_OA_ACCOUNT_LOGOUT_RES" => Some(Self::ProtoOaAccountLogoutRes),
            "PROTO_OA_ACCOUNT_DISCONNECT_EVENT" => Some(Self::ProtoOaAccountDisconnectEvent),
            "PROTO_OA_SUBSCRIBE_LIVE_TRENDBAR_RES" => Some(Self::ProtoOaSubscribeLiveTrendbarRes),
            "PROTO_OA_UNSUBSCRIBE_LIVE_TRENDBAR_RES" => {
                Some(Self::ProtoOaUnsubscribeLiveTrendbarRes)
            }
            "PROTO_OA_MARGIN_CALL_LIST_REQ" => Some(Self::ProtoOaMarginCallListReq),
            "PROTO_OA_MARGIN_CALL_LIST_RES" => Some(Self::ProtoOaMarginCallListRes),
            "PROTO_OA_MARGIN_CALL_UPDATE_REQ" => Some(Self::ProtoOaMarginCallUpdateReq),
            "PROTO_OA_MARGIN_CALL_UPDATE_RES" => Some(Self::ProtoOaMarginCallUpdateRes),
            "PROTO_OA_MARGIN_CALL_UPDATE_EVENT" => Some(Self::ProtoOaMarginCallUpdateEvent),
            "PROTO_OA_MARGIN_CALL_TRIGGER_EVENT" => Some(Self::ProtoOaMarginCallTriggerEvent),
            "PROTO_OA_REFRESH_TOKEN_REQ" => Some(Self::ProtoOaRefreshTokenReq),
            "PROTO_OA_REFRESH_TOKEN_RES" => Some(Self::ProtoOaRefreshTokenRes),
            "PROTO_OA_ORDER_LIST_REQ" => Some(Self::ProtoOaOrderListReq),
            "PROTO_OA_ORDER_LIST_RES" => Some(Self::ProtoOaOrderListRes),
            "PROTO_OA_GET_DYNAMIC_LEVERAGE_REQ" => Some(Self::ProtoOaGetDynamicLeverageReq),
            "PROTO_OA_GET_DYNAMIC_LEVERAGE_RES" => Some(Self::ProtoOaGetDynamicLeverageRes),
            "PROTO_OA_DEAL_LIST_BY_POSITION_ID_REQ" => Some(Self::ProtoOaDealListByPositionIdReq),
            "PROTO_OA_DEAL_LIST_BY_POSITION_ID_RES" => Some(Self::ProtoOaDealListByPositionIdRes),
            "PROTO_OA_ORDER_DETAILS_REQ" => Some(Self::ProtoOaOrderDetailsReq),
            "PROTO_OA_ORDER_DETAILS_RES" => Some(Self::ProtoOaOrderDetailsRes),
            "PROTO_OA_ORDER_LIST_BY_POSITION_ID_REQ" => Some(Self::ProtoOaOrderListByPositionIdReq),
            "PROTO_OA_ORDER_LIST_BY_POSITION_ID_RES" => Some(Self::ProtoOaOrderListByPositionIdRes),
            "PROTO_OA_DEAL_OFFSET_LIST_REQ" => Some(Self::ProtoOaDealOffsetListReq),
            "PROTO_OA_DEAL_OFFSET_LIST_RES" => Some(Self::ProtoOaDealOffsetListRes),
            "PROTO_OA_GET_POSITION_UNREALIZED_PNL_REQ" => {
                Some(Self::ProtoOaGetPositionUnrealizedPnlReq)
            }
            "PROTO_OA_GET_POSITION_UNREALIZED_PNL_RES" => {
                Some(Self::ProtoOaGetPositionUnrealizedPnlRes)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaDayOfWeek {
    None = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}
impl ProtoOaDayOfWeek {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaDayOfWeek::None => "NONE",
            ProtoOaDayOfWeek::Monday => "MONDAY",
            ProtoOaDayOfWeek::Tuesday => "TUESDAY",
            ProtoOaDayOfWeek::Wednesday => "WEDNESDAY",
            ProtoOaDayOfWeek::Thursday => "THURSDAY",
            ProtoOaDayOfWeek::Friday => "FRIDAY",
            ProtoOaDayOfWeek::Saturday => "SATURDAY",
            ProtoOaDayOfWeek::Sunday => "SUNDAY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "MONDAY" => Some(Self::Monday),
            "TUESDAY" => Some(Self::Tuesday),
            "WEDNESDAY" => Some(Self::Wednesday),
            "THURSDAY" => Some(Self::Thursday),
            "FRIDAY" => Some(Self::Friday),
            "SATURDAY" => Some(Self::Saturday),
            "SUNDAY" => Some(Self::Sunday),
            _ => None,
        }
    }
}
///   Enum for specifying type of trading commission
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaCommissionType {
    /// USD per million USD volume - usually used for FX. Example: 50 USD for 1 mil USD of trading volume. In cents.
    UsdPerMillionUsd = 1,
    /// USD per 1 lot - usually used for CFDs and futures for commodities, and indices. Example: 15 USD for 1 contract. In cents.
    UsdPerLot = 2,
    /// Percentage of trading volume - usually used for Equities. Example: 0.005% of notional trading volume. Multiplied by 100,000.
    PercentageOfValue = 3,
    /// Quote ccy of Symbol per 1 lot - will be used for CFDs and futures for commodities, and indices. Example: 15 EUR for 1 contract of DAX. In cents.
    QuoteCcyPerLot = 4,
}
impl ProtoOaCommissionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaCommissionType::UsdPerMillionUsd => "USD_PER_MILLION_USD",
            ProtoOaCommissionType::UsdPerLot => "USD_PER_LOT",
            ProtoOaCommissionType::PercentageOfValue => "PERCENTAGE_OF_VALUE",
            ProtoOaCommissionType::QuoteCcyPerLot => "QUOTE_CCY_PER_LOT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USD_PER_MILLION_USD" => Some(Self::UsdPerMillionUsd),
            "USD_PER_LOT" => Some(Self::UsdPerLot),
            "PERCENTAGE_OF_VALUE" => Some(Self::PercentageOfValue),
            "QUOTE_CCY_PER_LOT" => Some(Self::QuoteCcyPerLot),
            _ => None,
        }
    }
}
/// Enum for specifying stop loss and take profit distances
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaSymbolDistanceType {
    SymbolDistanceInPoints = 1,
    SymbolDistanceInPercentage = 2,
}
impl ProtoOaSymbolDistanceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaSymbolDistanceType::SymbolDistanceInPoints => "SYMBOL_DISTANCE_IN_POINTS",
            ProtoOaSymbolDistanceType::SymbolDistanceInPercentage => {
                "SYMBOL_DISTANCE_IN_PERCENTAGE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SYMBOL_DISTANCE_IN_POINTS" => Some(Self::SymbolDistanceInPoints),
            "SYMBOL_DISTANCE_IN_PERCENTAGE" => Some(Self::SymbolDistanceInPercentage),
            _ => None,
        }
    }
}
/// Enum for specifying type of minimum trading commission
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaMinCommissionType {
    Currency = 1,
    QuoteCurrency = 2,
}
impl ProtoOaMinCommissionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaMinCommissionType::Currency => "CURRENCY",
            ProtoOaMinCommissionType::QuoteCurrency => "QUOTE_CURRENCY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CURRENCY" => Some(Self::Currency),
            "QUOTE_CURRENCY" => Some(Self::QuoteCurrency),
            _ => None,
        }
    }
}
/// Enum for specifying symbol trading mode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaTradingMode {
    Enabled = 0,
    DisabledWithoutPendingsExecution = 1,
    DisabledWithPendingsExecution = 2,
    CloseOnlyMode = 3,
}
impl ProtoOaTradingMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaTradingMode::Enabled => "ENABLED",
            ProtoOaTradingMode::DisabledWithoutPendingsExecution => {
                "DISABLED_WITHOUT_PENDINGS_EXECUTION"
            }
            ProtoOaTradingMode::DisabledWithPendingsExecution => "DISABLED_WITH_PENDINGS_EXECUTION",
            ProtoOaTradingMode::CloseOnlyMode => "CLOSE_ONLY_MODE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENABLED" => Some(Self::Enabled),
            "DISABLED_WITHOUT_PENDINGS_EXECUTION" => Some(Self::DisabledWithoutPendingsExecution),
            "DISABLED_WITH_PENDINGS_EXECUTION" => Some(Self::DisabledWithPendingsExecution),
            "CLOSE_ONLY_MODE" => Some(Self::CloseOnlyMode),
            _ => None,
        }
    }
}
/// Enum for specifying SWAP calculation type for symbol.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaSwapCalculationType {
    /// Specifies type of SWAP computation as PIPS (0)
    Pips = 0,
    /// Specifies type of SWAP computation as PERCENTAGE (1, annual, in percent)
    Percentage = 1,
}
impl ProtoOaSwapCalculationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaSwapCalculationType::Pips => "PIPS",
            ProtoOaSwapCalculationType::Percentage => "PERCENTAGE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PIPS" => Some(Self::Pips),
            "PERCENTAGE" => Some(Self::Percentage),
            _ => None,
        }
    }
}
/// Enum for specifying access right for a trader
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaAccessRights {
    /// Enable all trading.
    FullAccess = 0,
    /// Only closing trading request are enabled.
    CloseOnly = 1,
    /// View only access.
    NoTrading = 2,
    /// No access.
    NoLogin = 3,
}
impl ProtoOaAccessRights {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaAccessRights::FullAccess => "FULL_ACCESS",
            ProtoOaAccessRights::CloseOnly => "CLOSE_ONLY",
            ProtoOaAccessRights::NoTrading => "NO_TRADING",
            ProtoOaAccessRights::NoLogin => "NO_LOGIN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FULL_ACCESS" => Some(Self::FullAccess),
            "CLOSE_ONLY" => Some(Self::CloseOnly),
            "NO_TRADING" => Some(Self::NoTrading),
            "NO_LOGIN" => Some(Self::NoLogin),
            _ => None,
        }
    }
}
/// Enum for specifying margin calculation type for an account
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaTotalMarginCalculationType {
    Max = 0,
    Sum = 1,
    Net = 2,
}
impl ProtoOaTotalMarginCalculationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaTotalMarginCalculationType::Max => "MAX",
            ProtoOaTotalMarginCalculationType::Sum => "SUM",
            ProtoOaTotalMarginCalculationType::Net => "NET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MAX" => Some(Self::Max),
            "SUM" => Some(Self::Sum),
            "NET" => Some(Self::Net),
            _ => None,
        }
    }
}
/// Enum for specifying type of an account
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaAccountType {
    /// Allows multiple positions on a trading account for a symbol.
    Hedged = 0,
    /// Only one position per symbol is allowed on a trading account.
    Netted = 1,
    /// Spread betting type account.
    SpreadBetting = 2,
}
impl ProtoOaAccountType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaAccountType::Hedged => "HEDGED",
            ProtoOaAccountType::Netted => "NETTED",
            ProtoOaAccountType::SpreadBetting => "SPREAD_BETTING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HEDGED" => Some(Self::Hedged),
            "NETTED" => Some(Self::Netted),
            "SPREAD_BETTING" => Some(Self::SpreadBetting),
            _ => None,
        }
    }
}
/// Type of notification, currently only 3 instances of marginCall are supported.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaNotificationType {
    /// one of three margin calls, they are all similar.
    MarginLevelThreshold1 = 61,
    /// one of three margin calls, they are all similar.
    MarginLevelThreshold2 = 62,
    /// one of three margin calls, they are all similar.
    MarginLevelThreshold3 = 63,
}
impl ProtoOaNotificationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaNotificationType::MarginLevelThreshold1 => "MARGIN_LEVEL_THRESHOLD_1",
            ProtoOaNotificationType::MarginLevelThreshold2 => "MARGIN_LEVEL_THRESHOLD_2",
            ProtoOaNotificationType::MarginLevelThreshold3 => "MARGIN_LEVEL_THRESHOLD_3",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MARGIN_LEVEL_THRESHOLD_1" => Some(Self::MarginLevelThreshold1),
            "MARGIN_LEVEL_THRESHOLD_2" => Some(Self::MarginLevelThreshold2),
            "MARGIN_LEVEL_THRESHOLD_3" => Some(Self::MarginLevelThreshold3),
            _ => None,
        }
    }
}
/// Error code ENUM
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaErrorCode {
    /// Authorization
    ///
    /// When token used for account authorization is expired.
    OaAuthTokenExpired = 1,
    /// When account is not authorized.
    AccountNotAuthorized = 2,
    /// When such account no longer exists.
    RetNoSuchLogin = 12,
    /// When client tries to authorize after it was already authorized.
    AlreadyLoggedIn = 14,
    /// When account is disabled.
    RetAccountDisabled = 64,
    /// Open API client is not activated or wrong client credentials.
    ChClientAuthFailure = 101,
    /// When a command is sent for not authorized Open API client.
    ChClientNotAuthenticated = 102,
    /// Client is trying to authenticate twice.
    ChClientAlreadyAuthenticated = 103,
    /// Access token is invalid.
    ChAccessTokenInvalid = 104,
    /// Trading service is not available.
    ChServerNotReachable = 105,
    /// Trading account is not found.
    ChCtidTraderAccountNotFound = 106,
    /// Could not find this client id.
    ChOaClientNotFound = 107,
    /// General
    ///
    /// Request frequency is reached.
    RequestFrequencyExceeded = 108,
    /// Server is under maintenance.
    ServerIsUnderMaintenance = 109,
    /// Operations are not allowed for this account.
    ChannelIsBlocked = 110,
    /// Limit of connections is reached for this Open API client.
    ConnectionsLimitExceeded = 67,
    /// Not allowed to increase risk for Positions with Guaranteed Stop Loss.
    WorseGslNotAllowed = 68,
    /// Trading disabled because symbol has holiday.
    SymbolHasHoliday = 69,
    /// Pricing
    ///
    /// When trying to subscribe to depth, trendbars, etc. without spot subscription.
    NotSubscribedToSpots = 112,
    /// When subscription is requested for an active.
    AlreadySubscribed = 113,
    /// Symbol not found.
    SymbolNotFound = 114,
    /// Note: to be merged with SYMBOL_NOT_FOUND.
    UnknownSymbol = 115,
    /// When requested period (from,to) is too large or invalid values are set to from/to.
    IncorrectBoundaries = 35,
    /// Trading
    ///
    /// Trading cannot be done as not quotes are available. Applicable for Book B.
    NoQuotes = 117,
    /// Not enough funds to allocate margin.
    NotEnoughMoney = 118,
    /// Max exposure limit is reached for a {trader, symbol, side}.
    MaxExposureReached = 119,
    /// Position not found.
    PositionNotFound = 120,
    /// Order not found.
    OrderNotFound = 121,
    /// When trying to close a position that it is not open.
    PositionNotOpen = 122,
    /// Position in the state that does not allow to perform an operation.
    PositionLocked = 123,
    /// Trading account reached its limit for max number of open positions and orders.
    TooManyPositions = 124,
    /// Invalid volume.
    TradingBadVolume = 125,
    /// Invalid stop price.
    TradingBadStops = 126,
    /// Invalid price (e.g. negative).
    TradingBadPrices = 127,
    /// Invalid stake volume (e.g. negative).
    TradingBadStake = 128,
    /// Invalid protection prices.
    ProtectionIsTooCloseToMarket = 129,
    /// Invalid expiration.
    TradingBadExpirationDate = 130,
    /// Unable to apply changes as position has an order under execution.
    PendingExecution = 131,
    /// Trading is blocked for the symbol.
    TradingDisabled = 132,
    /// Trading account is in read only mode.
    TradingNotAllowed = 133,
    /// Unable to cancel order.
    UnableToCancelOrder = 134,
    /// Unable to amend order.
    UnableToAmendOrder = 135,
    /// Short selling is not allowed.
    ShortSellingNotAllowed = 136,
}
impl ProtoOaErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaErrorCode::OaAuthTokenExpired => "OA_AUTH_TOKEN_EXPIRED",
            ProtoOaErrorCode::AccountNotAuthorized => "ACCOUNT_NOT_AUTHORIZED",
            ProtoOaErrorCode::RetNoSuchLogin => "RET_NO_SUCH_LOGIN",
            ProtoOaErrorCode::AlreadyLoggedIn => "ALREADY_LOGGED_IN",
            ProtoOaErrorCode::RetAccountDisabled => "RET_ACCOUNT_DISABLED",
            ProtoOaErrorCode::ChClientAuthFailure => "CH_CLIENT_AUTH_FAILURE",
            ProtoOaErrorCode::ChClientNotAuthenticated => "CH_CLIENT_NOT_AUTHENTICATED",
            ProtoOaErrorCode::ChClientAlreadyAuthenticated => "CH_CLIENT_ALREADY_AUTHENTICATED",
            ProtoOaErrorCode::ChAccessTokenInvalid => "CH_ACCESS_TOKEN_INVALID",
            ProtoOaErrorCode::ChServerNotReachable => "CH_SERVER_NOT_REACHABLE",
            ProtoOaErrorCode::ChCtidTraderAccountNotFound => "CH_CTID_TRADER_ACCOUNT_NOT_FOUND",
            ProtoOaErrorCode::ChOaClientNotFound => "CH_OA_CLIENT_NOT_FOUND",
            ProtoOaErrorCode::RequestFrequencyExceeded => "REQUEST_FREQUENCY_EXCEEDED",
            ProtoOaErrorCode::ServerIsUnderMaintenance => "SERVER_IS_UNDER_MAINTENANCE",
            ProtoOaErrorCode::ChannelIsBlocked => "CHANNEL_IS_BLOCKED",
            ProtoOaErrorCode::ConnectionsLimitExceeded => "CONNECTIONS_LIMIT_EXCEEDED",
            ProtoOaErrorCode::WorseGslNotAllowed => "WORSE_GSL_NOT_ALLOWED",
            ProtoOaErrorCode::SymbolHasHoliday => "SYMBOL_HAS_HOLIDAY",
            ProtoOaErrorCode::NotSubscribedToSpots => "NOT_SUBSCRIBED_TO_SPOTS",
            ProtoOaErrorCode::AlreadySubscribed => "ALREADY_SUBSCRIBED",
            ProtoOaErrorCode::SymbolNotFound => "SYMBOL_NOT_FOUND",
            ProtoOaErrorCode::UnknownSymbol => "UNKNOWN_SYMBOL",
            ProtoOaErrorCode::IncorrectBoundaries => "INCORRECT_BOUNDARIES",
            ProtoOaErrorCode::NoQuotes => "NO_QUOTES",
            ProtoOaErrorCode::NotEnoughMoney => "NOT_ENOUGH_MONEY",
            ProtoOaErrorCode::MaxExposureReached => "MAX_EXPOSURE_REACHED",
            ProtoOaErrorCode::PositionNotFound => "POSITION_NOT_FOUND",
            ProtoOaErrorCode::OrderNotFound => "ORDER_NOT_FOUND",
            ProtoOaErrorCode::PositionNotOpen => "POSITION_NOT_OPEN",
            ProtoOaErrorCode::PositionLocked => "POSITION_LOCKED",
            ProtoOaErrorCode::TooManyPositions => "TOO_MANY_POSITIONS",
            ProtoOaErrorCode::TradingBadVolume => "TRADING_BAD_VOLUME",
            ProtoOaErrorCode::TradingBadStops => "TRADING_BAD_STOPS",
            ProtoOaErrorCode::TradingBadPrices => "TRADING_BAD_PRICES",
            ProtoOaErrorCode::TradingBadStake => "TRADING_BAD_STAKE",
            ProtoOaErrorCode::ProtectionIsTooCloseToMarket => "PROTECTION_IS_TOO_CLOSE_TO_MARKET",
            ProtoOaErrorCode::TradingBadExpirationDate => "TRADING_BAD_EXPIRATION_DATE",
            ProtoOaErrorCode::PendingExecution => "PENDING_EXECUTION",
            ProtoOaErrorCode::TradingDisabled => "TRADING_DISABLED",
            ProtoOaErrorCode::TradingNotAllowed => "TRADING_NOT_ALLOWED",
            ProtoOaErrorCode::UnableToCancelOrder => "UNABLE_TO_CANCEL_ORDER",
            ProtoOaErrorCode::UnableToAmendOrder => "UNABLE_TO_AMEND_ORDER",
            ProtoOaErrorCode::ShortSellingNotAllowed => "SHORT_SELLING_NOT_ALLOWED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OA_AUTH_TOKEN_EXPIRED" => Some(Self::OaAuthTokenExpired),
            "ACCOUNT_NOT_AUTHORIZED" => Some(Self::AccountNotAuthorized),
            "RET_NO_SUCH_LOGIN" => Some(Self::RetNoSuchLogin),
            "ALREADY_LOGGED_IN" => Some(Self::AlreadyLoggedIn),
            "RET_ACCOUNT_DISABLED" => Some(Self::RetAccountDisabled),
            "CH_CLIENT_AUTH_FAILURE" => Some(Self::ChClientAuthFailure),
            "CH_CLIENT_NOT_AUTHENTICATED" => Some(Self::ChClientNotAuthenticated),
            "CH_CLIENT_ALREADY_AUTHENTICATED" => Some(Self::ChClientAlreadyAuthenticated),
            "CH_ACCESS_TOKEN_INVALID" => Some(Self::ChAccessTokenInvalid),
            "CH_SERVER_NOT_REACHABLE" => Some(Self::ChServerNotReachable),
            "CH_CTID_TRADER_ACCOUNT_NOT_FOUND" => Some(Self::ChCtidTraderAccountNotFound),
            "CH_OA_CLIENT_NOT_FOUND" => Some(Self::ChOaClientNotFound),
            "REQUEST_FREQUENCY_EXCEEDED" => Some(Self::RequestFrequencyExceeded),
            "SERVER_IS_UNDER_MAINTENANCE" => Some(Self::ServerIsUnderMaintenance),
            "CHANNEL_IS_BLOCKED" => Some(Self::ChannelIsBlocked),
            "CONNECTIONS_LIMIT_EXCEEDED" => Some(Self::ConnectionsLimitExceeded),
            "WORSE_GSL_NOT_ALLOWED" => Some(Self::WorseGslNotAllowed),
            "SYMBOL_HAS_HOLIDAY" => Some(Self::SymbolHasHoliday),
            "NOT_SUBSCRIBED_TO_SPOTS" => Some(Self::NotSubscribedToSpots),
            "ALREADY_SUBSCRIBED" => Some(Self::AlreadySubscribed),
            "SYMBOL_NOT_FOUND" => Some(Self::SymbolNotFound),
            "UNKNOWN_SYMBOL" => Some(Self::UnknownSymbol),
            "INCORRECT_BOUNDARIES" => Some(Self::IncorrectBoundaries),
            "NO_QUOTES" => Some(Self::NoQuotes),
            "NOT_ENOUGH_MONEY" => Some(Self::NotEnoughMoney),
            "MAX_EXPOSURE_REACHED" => Some(Self::MaxExposureReached),
            "POSITION_NOT_FOUND" => Some(Self::PositionNotFound),
            "ORDER_NOT_FOUND" => Some(Self::OrderNotFound),
            "POSITION_NOT_OPEN" => Some(Self::PositionNotOpen),
            "POSITION_LOCKED" => Some(Self::PositionLocked),
            "TOO_MANY_POSITIONS" => Some(Self::TooManyPositions),
            "TRADING_BAD_VOLUME" => Some(Self::TradingBadVolume),
            "TRADING_BAD_STOPS" => Some(Self::TradingBadStops),
            "TRADING_BAD_PRICES" => Some(Self::TradingBadPrices),
            "TRADING_BAD_STAKE" => Some(Self::TradingBadStake),
            "PROTECTION_IS_TOO_CLOSE_TO_MARKET" => Some(Self::ProtectionIsTooCloseToMarket),
            "TRADING_BAD_EXPIRATION_DATE" => Some(Self::TradingBadExpirationDate),
            "PENDING_EXECUTION" => Some(Self::PendingExecution),
            "TRADING_DISABLED" => Some(Self::TradingDisabled),
            "TRADING_NOT_ALLOWED" => Some(Self::TradingNotAllowed),
            "UNABLE_TO_CANCEL_ORDER" => Some(Self::UnableToCancelOrder),
            "UNABLE_TO_AMEND_ORDER" => Some(Self::UnableToAmendOrder),
            "SHORT_SELLING_NOT_ALLOWED" => Some(Self::ShortSellingNotAllowed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaLimitedRiskMarginCalculationStrategy {
    AccordingToLeverage = 0,
    AccordingToGsl = 1,
    AccordingToGslAndLeverage = 2,
}
impl ProtoOaLimitedRiskMarginCalculationStrategy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaLimitedRiskMarginCalculationStrategy::AccordingToLeverage => {
                "ACCORDING_TO_LEVERAGE"
            }
            ProtoOaLimitedRiskMarginCalculationStrategy::AccordingToGsl => "ACCORDING_TO_GSL",
            ProtoOaLimitedRiskMarginCalculationStrategy::AccordingToGslAndLeverage => {
                "ACCORDING_TO_GSL_AND_LEVERAGE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCORDING_TO_LEVERAGE" => Some(Self::AccordingToLeverage),
            "ACCORDING_TO_GSL" => Some(Self::AccordingToGsl),
            "ACCORDING_TO_GSL_AND_LEVERAGE" => Some(Self::AccordingToGslAndLeverage),
            _ => None,
        }
    }
}
/// Price quote type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaQuoteType {
    Bid = 1,
    Ask = 2,
}
impl ProtoOaQuoteType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaQuoteType::Bid => "BID",
            ProtoOaQuoteType::Ask => "ASK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BID" => Some(Self::Bid),
            "ASK" => Some(Self::Ask),
            _ => None,
        }
    }
}
/// Open API application permission in regards to token ENUM
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaClientPermissionScope {
    /// Allows to use only view commends. Trade is prohibited.
    ScopeView = 0,
    /// Allows to use all commands.
    ScopeTrade = 1,
}
impl ProtoOaClientPermissionScope {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaClientPermissionScope::ScopeView => "SCOPE_VIEW",
            ProtoOaClientPermissionScope::ScopeTrade => "SCOPE_TRADE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SCOPE_VIEW" => Some(Self::ScopeView),
            "SCOPE_TRADE" => Some(Self::ScopeTrade),
            _ => None,
        }
    }
}
/// Trendbar period ENUM
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaTrendbarPeriod {
    M1 = 1,
    M2 = 2,
    M3 = 3,
    M4 = 4,
    M5 = 5,
    M10 = 6,
    M15 = 7,
    M30 = 8,
    H1 = 9,
    H4 = 10,
    H12 = 11,
    D1 = 12,
    W1 = 13,
    Mn1 = 14,
}
impl ProtoOaTrendbarPeriod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaTrendbarPeriod::M1 => "M1",
            ProtoOaTrendbarPeriod::M2 => "M2",
            ProtoOaTrendbarPeriod::M3 => "M3",
            ProtoOaTrendbarPeriod::M4 => "M4",
            ProtoOaTrendbarPeriod::M5 => "M5",
            ProtoOaTrendbarPeriod::M10 => "M10",
            ProtoOaTrendbarPeriod::M15 => "M15",
            ProtoOaTrendbarPeriod::M30 => "M30",
            ProtoOaTrendbarPeriod::H1 => "H1",
            ProtoOaTrendbarPeriod::H4 => "H4",
            ProtoOaTrendbarPeriod::H12 => "H12",
            ProtoOaTrendbarPeriod::D1 => "D1",
            ProtoOaTrendbarPeriod::W1 => "W1",
            ProtoOaTrendbarPeriod::Mn1 => "MN1",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "M1" => Some(Self::M1),
            "M2" => Some(Self::M2),
            "M3" => Some(Self::M3),
            "M4" => Some(Self::M4),
            "M5" => Some(Self::M5),
            "M10" => Some(Self::M10),
            "M15" => Some(Self::M15),
            "M30" => Some(Self::M30),
            "H1" => Some(Self::H1),
            "H4" => Some(Self::H4),
            "H12" => Some(Self::H12),
            "D1" => Some(Self::D1),
            "W1" => Some(Self::W1),
            "MN1" => Some(Self::Mn1),
            _ => None,
        }
    }
}
/// Position status ENUM
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaPositionStatus {
    PositionStatusOpen = 1,
    PositionStatusClosed = 2,
    /// Empty position is created for pending order.
    PositionStatusCreated = 3,
    PositionStatusError = 4,
}
impl ProtoOaPositionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaPositionStatus::PositionStatusOpen => "POSITION_STATUS_OPEN",
            ProtoOaPositionStatus::PositionStatusClosed => "POSITION_STATUS_CLOSED",
            ProtoOaPositionStatus::PositionStatusCreated => "POSITION_STATUS_CREATED",
            ProtoOaPositionStatus::PositionStatusError => "POSITION_STATUS_ERROR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "POSITION_STATUS_OPEN" => Some(Self::PositionStatusOpen),
            "POSITION_STATUS_CLOSED" => Some(Self::PositionStatusClosed),
            "POSITION_STATUS_CREATED" => Some(Self::PositionStatusCreated),
            "POSITION_STATUS_ERROR" => Some(Self::PositionStatusError),
            _ => None,
        }
    }
}
/// Trade side ENUM. Used for order, position, deal
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaTradeSide {
    Buy = 1,
    Sell = 2,
}
impl ProtoOaTradeSide {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaTradeSide::Buy => "BUY",
            ProtoOaTradeSide::Sell => "SELL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BUY" => Some(Self::Buy),
            "SELL" => Some(Self::Sell),
            _ => None,
        }
    }
}
/// Order type ENUM
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaOrderType {
    Market = 1,
    Limit = 2,
    Stop = 3,
    StopLossTakeProfit = 4,
    MarketRange = 5,
    StopLimit = 6,
}
impl ProtoOaOrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaOrderType::Market => "MARKET",
            ProtoOaOrderType::Limit => "LIMIT",
            ProtoOaOrderType::Stop => "STOP",
            ProtoOaOrderType::StopLossTakeProfit => "STOP_LOSS_TAKE_PROFIT",
            ProtoOaOrderType::MarketRange => "MARKET_RANGE",
            ProtoOaOrderType::StopLimit => "STOP_LIMIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MARKET" => Some(Self::Market),
            "LIMIT" => Some(Self::Limit),
            "STOP" => Some(Self::Stop),
            "STOP_LOSS_TAKE_PROFIT" => Some(Self::StopLossTakeProfit),
            "MARKET_RANGE" => Some(Self::MarketRange),
            "STOP_LIMIT" => Some(Self::StopLimit),
            _ => None,
        }
    }
}
/// Order time in force ENUM
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaTimeInForce {
    GoodTillDate = 1,
    GoodTillCancel = 2,
    ImmediateOrCancel = 3,
    FillOrKill = 4,
    MarketOnOpen = 5,
}
impl ProtoOaTimeInForce {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaTimeInForce::GoodTillDate => "GOOD_TILL_DATE",
            ProtoOaTimeInForce::GoodTillCancel => "GOOD_TILL_CANCEL",
            ProtoOaTimeInForce::ImmediateOrCancel => "IMMEDIATE_OR_CANCEL",
            ProtoOaTimeInForce::FillOrKill => "FILL_OR_KILL",
            ProtoOaTimeInForce::MarketOnOpen => "MARKET_ON_OPEN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GOOD_TILL_DATE" => Some(Self::GoodTillDate),
            "GOOD_TILL_CANCEL" => Some(Self::GoodTillCancel),
            "IMMEDIATE_OR_CANCEL" => Some(Self::ImmediateOrCancel),
            "FILL_OR_KILL" => Some(Self::FillOrKill),
            "MARKET_ON_OPEN" => Some(Self::MarketOnOpen),
            _ => None,
        }
    }
}
/// Order status ENUM
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaOrderStatus {
    /// Order request validated and accepted for execution.
    OrderStatusAccepted = 1,
    /// Order is fully filled.
    OrderStatusFilled = 2,
    /// Order is rejected due to validation.
    OrderStatusRejected = 3,
    /// Order expired. Might be valid for orders with partially filled volume that were expired on LP.
    OrderStatusExpired = 4,
    /// Order is cancelled. Might be valid for orders with partially filled volume that were cancelled by LP.
    OrderStatusCancelled = 5,
}
impl ProtoOaOrderStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaOrderStatus::OrderStatusAccepted => "ORDER_STATUS_ACCEPTED",
            ProtoOaOrderStatus::OrderStatusFilled => "ORDER_STATUS_FILLED",
            ProtoOaOrderStatus::OrderStatusRejected => "ORDER_STATUS_REJECTED",
            ProtoOaOrderStatus::OrderStatusExpired => "ORDER_STATUS_EXPIRED",
            ProtoOaOrderStatus::OrderStatusCancelled => "ORDER_STATUS_CANCELLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_STATUS_ACCEPTED" => Some(Self::OrderStatusAccepted),
            "ORDER_STATUS_FILLED" => Some(Self::OrderStatusFilled),
            "ORDER_STATUS_REJECTED" => Some(Self::OrderStatusRejected),
            "ORDER_STATUS_EXPIRED" => Some(Self::OrderStatusExpired),
            "ORDER_STATUS_CANCELLED" => Some(Self::OrderStatusCancelled),
            _ => None,
        }
    }
}
/// Stop Order and Stop Lost triggering method ENUM
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaOrderTriggerMethod {
    /// Stop Order: buy is triggered by ask, sell by bid; Stop Loss Order: for buy position is triggered by bid and for sell position by ask.
    Trade = 1,
    /// Stop Order: buy is triggered by bid, sell by ask; Stop Loss Order: for buy position is triggered by ask and for sell position by bid.
    Opposite = 2,
    /// The same as TRADE, but trigger is checked after the second consecutive tick.
    DoubleTrade = 3,
    /// The same as OPPOSITE, but trigger is checked after the second consecutive tick.
    DoubleOpposite = 4,
}
impl ProtoOaOrderTriggerMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaOrderTriggerMethod::Trade => "TRADE",
            ProtoOaOrderTriggerMethod::Opposite => "OPPOSITE",
            ProtoOaOrderTriggerMethod::DoubleTrade => "DOUBLE_TRADE",
            ProtoOaOrderTriggerMethod::DoubleOpposite => "DOUBLE_OPPOSITE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRADE" => Some(Self::Trade),
            "OPPOSITE" => Some(Self::Opposite),
            "DOUBLE_TRADE" => Some(Self::DoubleTrade),
            "DOUBLE_OPPOSITE" => Some(Self::DoubleOpposite),
            _ => None,
        }
    }
}
/// Execution event type ENUM
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaExecutionType {
    /// Order passed validation.
    OrderAccepted = 2,
    /// Order filled.
    OrderFilled = 3,
    /// Pending order is changed with a new one.
    OrderReplaced = 4,
    /// Order cancelled.
    OrderCancelled = 5,
    /// Order with GTD time in force is expired.
    OrderExpired = 6,
    /// Order is rejected due to validations.
    OrderRejected = 7,
    /// Cancel order request is rejected.
    OrderCancelRejected = 8,
    /// Type related to SWAP execution events.
    Swap = 9,
    /// Type related to event of deposit or withdrawal cash flow operation.
    DepositWithdraw = 10,
    /// Order is partially filled.
    OrderPartialFill = 11,
    /// Type related to event of bonus deposit or bonus withdrawal.
    BonusDepositWithdraw = 12,
}
impl ProtoOaExecutionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaExecutionType::OrderAccepted => "ORDER_ACCEPTED",
            ProtoOaExecutionType::OrderFilled => "ORDER_FILLED",
            ProtoOaExecutionType::OrderReplaced => "ORDER_REPLACED",
            ProtoOaExecutionType::OrderCancelled => "ORDER_CANCELLED",
            ProtoOaExecutionType::OrderExpired => "ORDER_EXPIRED",
            ProtoOaExecutionType::OrderRejected => "ORDER_REJECTED",
            ProtoOaExecutionType::OrderCancelRejected => "ORDER_CANCEL_REJECTED",
            ProtoOaExecutionType::Swap => "SWAP",
            ProtoOaExecutionType::DepositWithdraw => "DEPOSIT_WITHDRAW",
            ProtoOaExecutionType::OrderPartialFill => "ORDER_PARTIAL_FILL",
            ProtoOaExecutionType::BonusDepositWithdraw => "BONUS_DEPOSIT_WITHDRAW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_ACCEPTED" => Some(Self::OrderAccepted),
            "ORDER_FILLED" => Some(Self::OrderFilled),
            "ORDER_REPLACED" => Some(Self::OrderReplaced),
            "ORDER_CANCELLED" => Some(Self::OrderCancelled),
            "ORDER_EXPIRED" => Some(Self::OrderExpired),
            "ORDER_REJECTED" => Some(Self::OrderRejected),
            "ORDER_CANCEL_REJECTED" => Some(Self::OrderCancelRejected),
            "SWAP" => Some(Self::Swap),
            "DEPOSIT_WITHDRAW" => Some(Self::DepositWithdraw),
            "ORDER_PARTIAL_FILL" => Some(Self::OrderPartialFill),
            "BONUS_DEPOSIT_WITHDRAW" => Some(Self::BonusDepositWithdraw),
            _ => None,
        }
    }
}
/// Balance operation entity. Covers all cash movement operations related to account, trading, IB operations, mirroring, etc.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaChangeBalanceType {
    /// Cash deposit.
    BalanceDeposit = 0,
    /// Cash withdrawal.
    BalanceWithdraw = 1,
    /// Received mirroring commission.
    BalanceDepositStrategyCommissionInner = 3,
    /// Paid mirroring commission.
    BalanceWithdrawStrategyCommissionInner = 4,
    /// For IB account. Commissions paid by trader.
    BalanceDepositIbCommissions = 5,
    /// For IB account. Withdrawal of commissions shared with broker.
    BalanceWithdrawIbSharedPercentage = 6,
    /// For IB account. Commissions paid by sub-ibs.
    BalanceDepositIbSharedPercentageFromSubIb = 7,
    /// For IB account. Commissions paid by broker.
    BalanceDepositIbSharedPercentageFromBroker = 8,
    /// Deposit rebate for trading volume for period.
    BalanceDepositRebate = 9,
    /// Withdrawal of rebate.
    BalanceWithdrawRebate = 10,
    /// Mirroring commission.
    BalanceDepositStrategyCommissionOuter = 11,
    /// Mirroring commission.
    BalanceWithdrawStrategyCommissionOuter = 12,
    /// For IB account. Share commission with the Broker.
    BalanceWithdrawBonusCompensation = 13,
    /// IB commissions.
    BalanceWithdrawIbSharedPercentageToBroker = 14,
    /// Deposit dividends payments.
    BalanceDepositDividends = 15,
    /// Negative dividend charge for short position.
    BalanceWithdrawDividends = 16,
    /// Charge for guaranteedStopLoss.
    BalanceWithdrawGslCharge = 17,
    /// Charge of rollover fee for Shariah compliant accounts.
    BalanceWithdrawRollover = 18,
    /// Broker operation to deposit bonus.
    BalanceDepositNonwithdrawableBonus = 19,
    /// Broker operation to withdrawal bonus.
    BalanceWithdrawNonwithdrawableBonus = 20,
    /// Deposits of negative SWAP.
    BalanceDepositSwap = 21,
    /// SWAP charges.
    BalanceWithdrawSwap = 22,
    /// Mirroring commission.
    BalanceDepositManagementFee = 27,
    /// Mirroring commission. Deprecated since 7.1 in favor of BALANCE_WITHDRAW_COPY_FEE (34).
    BalanceWithdrawManagementFee = 28,
    /// Mirroring commission.
    BalanceDepositPerformanceFee = 29,
    /// Withdraw for subaccount creation (cTrader Copy).
    BalanceWithdrawForSubaccount = 30,
    /// Deposit to subaccount on creation (cTrader Copy).
    BalanceDepositToSubaccount = 31,
    /// Manual user's withdraw from subaccount (cTrader Copy), to parent account.
    BalanceWithdrawFromSubaccount = 32,
    /// Manual user's deposit to subaccount (cTrader Copy), from parent account.
    BalanceDepositFromSubaccount = 33,
    /// Withdrawal fees to Strategy Provider.
    BalanceWithdrawCopyFee = 34,
    /// Withdraw of inactivity fee from the balance.
    BalanceWithdrawInactivityFee = 35,
    /// Deposit within the same server (from another account).
    BalanceDepositTransfer = 36,
    /// Withdraw within the same server (to another account).
    BalanceWithdrawTransfer = 37,
    /// Bonus being converted from virtual bonus to real deposit.
    BalanceDepositConvertedBonus = 38,
    /// Applies if negative balance protection is configured by broker, should make balance = 0.
    BalanceDepositNegativeBalanceProtection = 39,
}
impl ProtoOaChangeBalanceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaChangeBalanceType::BalanceDeposit => "BALANCE_DEPOSIT",
            ProtoOaChangeBalanceType::BalanceWithdraw => "BALANCE_WITHDRAW",
            ProtoOaChangeBalanceType::BalanceDepositStrategyCommissionInner => {
                "BALANCE_DEPOSIT_STRATEGY_COMMISSION_INNER"
            }
            ProtoOaChangeBalanceType::BalanceWithdrawStrategyCommissionInner => {
                "BALANCE_WITHDRAW_STRATEGY_COMMISSION_INNER"
            }
            ProtoOaChangeBalanceType::BalanceDepositIbCommissions => {
                "BALANCE_DEPOSIT_IB_COMMISSIONS"
            }
            ProtoOaChangeBalanceType::BalanceWithdrawIbSharedPercentage => {
                "BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE"
            }
            ProtoOaChangeBalanceType::BalanceDepositIbSharedPercentageFromSubIb => {
                "BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_SUB_IB"
            }
            ProtoOaChangeBalanceType::BalanceDepositIbSharedPercentageFromBroker => {
                "BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_BROKER"
            }
            ProtoOaChangeBalanceType::BalanceDepositRebate => "BALANCE_DEPOSIT_REBATE",
            ProtoOaChangeBalanceType::BalanceWithdrawRebate => "BALANCE_WITHDRAW_REBATE",
            ProtoOaChangeBalanceType::BalanceDepositStrategyCommissionOuter => {
                "BALANCE_DEPOSIT_STRATEGY_COMMISSION_OUTER"
            }
            ProtoOaChangeBalanceType::BalanceWithdrawStrategyCommissionOuter => {
                "BALANCE_WITHDRAW_STRATEGY_COMMISSION_OUTER"
            }
            ProtoOaChangeBalanceType::BalanceWithdrawBonusCompensation => {
                "BALANCE_WITHDRAW_BONUS_COMPENSATION"
            }
            ProtoOaChangeBalanceType::BalanceWithdrawIbSharedPercentageToBroker => {
                "BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE_TO_BROKER"
            }
            ProtoOaChangeBalanceType::BalanceDepositDividends => "BALANCE_DEPOSIT_DIVIDENDS",
            ProtoOaChangeBalanceType::BalanceWithdrawDividends => "BALANCE_WITHDRAW_DIVIDENDS",
            ProtoOaChangeBalanceType::BalanceWithdrawGslCharge => "BALANCE_WITHDRAW_GSL_CHARGE",
            ProtoOaChangeBalanceType::BalanceWithdrawRollover => "BALANCE_WITHDRAW_ROLLOVER",
            ProtoOaChangeBalanceType::BalanceDepositNonwithdrawableBonus => {
                "BALANCE_DEPOSIT_NONWITHDRAWABLE_BONUS"
            }
            ProtoOaChangeBalanceType::BalanceWithdrawNonwithdrawableBonus => {
                "BALANCE_WITHDRAW_NONWITHDRAWABLE_BONUS"
            }
            ProtoOaChangeBalanceType::BalanceDepositSwap => "BALANCE_DEPOSIT_SWAP",
            ProtoOaChangeBalanceType::BalanceWithdrawSwap => "BALANCE_WITHDRAW_SWAP",
            ProtoOaChangeBalanceType::BalanceDepositManagementFee => {
                "BALANCE_DEPOSIT_MANAGEMENT_FEE"
            }
            ProtoOaChangeBalanceType::BalanceWithdrawManagementFee => {
                "BALANCE_WITHDRAW_MANAGEMENT_FEE"
            }
            ProtoOaChangeBalanceType::BalanceDepositPerformanceFee => {
                "BALANCE_DEPOSIT_PERFORMANCE_FEE"
            }
            ProtoOaChangeBalanceType::BalanceWithdrawForSubaccount => {
                "BALANCE_WITHDRAW_FOR_SUBACCOUNT"
            }
            ProtoOaChangeBalanceType::BalanceDepositToSubaccount => "BALANCE_DEPOSIT_TO_SUBACCOUNT",
            ProtoOaChangeBalanceType::BalanceWithdrawFromSubaccount => {
                "BALANCE_WITHDRAW_FROM_SUBACCOUNT"
            }
            ProtoOaChangeBalanceType::BalanceDepositFromSubaccount => {
                "BALANCE_DEPOSIT_FROM_SUBACCOUNT"
            }
            ProtoOaChangeBalanceType::BalanceWithdrawCopyFee => "BALANCE_WITHDRAW_COPY_FEE",
            ProtoOaChangeBalanceType::BalanceWithdrawInactivityFee => {
                "BALANCE_WITHDRAW_INACTIVITY_FEE"
            }
            ProtoOaChangeBalanceType::BalanceDepositTransfer => "BALANCE_DEPOSIT_TRANSFER",
            ProtoOaChangeBalanceType::BalanceWithdrawTransfer => "BALANCE_WITHDRAW_TRANSFER",
            ProtoOaChangeBalanceType::BalanceDepositConvertedBonus => {
                "BALANCE_DEPOSIT_CONVERTED_BONUS"
            }
            ProtoOaChangeBalanceType::BalanceDepositNegativeBalanceProtection => {
                "BALANCE_DEPOSIT_NEGATIVE_BALANCE_PROTECTION"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BALANCE_DEPOSIT" => Some(Self::BalanceDeposit),
            "BALANCE_WITHDRAW" => Some(Self::BalanceWithdraw),
            "BALANCE_DEPOSIT_STRATEGY_COMMISSION_INNER" => {
                Some(Self::BalanceDepositStrategyCommissionInner)
            }
            "BALANCE_WITHDRAW_STRATEGY_COMMISSION_INNER" => {
                Some(Self::BalanceWithdrawStrategyCommissionInner)
            }
            "BALANCE_DEPOSIT_IB_COMMISSIONS" => Some(Self::BalanceDepositIbCommissions),
            "BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE" => {
                Some(Self::BalanceWithdrawIbSharedPercentage)
            }
            "BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_SUB_IB" => {
                Some(Self::BalanceDepositIbSharedPercentageFromSubIb)
            }
            "BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_BROKER" => {
                Some(Self::BalanceDepositIbSharedPercentageFromBroker)
            }
            "BALANCE_DEPOSIT_REBATE" => Some(Self::BalanceDepositRebate),
            "BALANCE_WITHDRAW_REBATE" => Some(Self::BalanceWithdrawRebate),
            "BALANCE_DEPOSIT_STRATEGY_COMMISSION_OUTER" => {
                Some(Self::BalanceDepositStrategyCommissionOuter)
            }
            "BALANCE_WITHDRAW_STRATEGY_COMMISSION_OUTER" => {
                Some(Self::BalanceWithdrawStrategyCommissionOuter)
            }
            "BALANCE_WITHDRAW_BONUS_COMPENSATION" => Some(Self::BalanceWithdrawBonusCompensation),
            "BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE_TO_BROKER" => {
                Some(Self::BalanceWithdrawIbSharedPercentageToBroker)
            }
            "BALANCE_DEPOSIT_DIVIDENDS" => Some(Self::BalanceDepositDividends),
            "BALANCE_WITHDRAW_DIVIDENDS" => Some(Self::BalanceWithdrawDividends),
            "BALANCE_WITHDRAW_GSL_CHARGE" => Some(Self::BalanceWithdrawGslCharge),
            "BALANCE_WITHDRAW_ROLLOVER" => Some(Self::BalanceWithdrawRollover),
            "BALANCE_DEPOSIT_NONWITHDRAWABLE_BONUS" => {
                Some(Self::BalanceDepositNonwithdrawableBonus)
            }
            "BALANCE_WITHDRAW_NONWITHDRAWABLE_BONUS" => {
                Some(Self::BalanceWithdrawNonwithdrawableBonus)
            }
            "BALANCE_DEPOSIT_SWAP" => Some(Self::BalanceDepositSwap),
            "BALANCE_WITHDRAW_SWAP" => Some(Self::BalanceWithdrawSwap),
            "BALANCE_DEPOSIT_MANAGEMENT_FEE" => Some(Self::BalanceDepositManagementFee),
            "BALANCE_WITHDRAW_MANAGEMENT_FEE" => Some(Self::BalanceWithdrawManagementFee),
            "BALANCE_DEPOSIT_PERFORMANCE_FEE" => Some(Self::BalanceDepositPerformanceFee),
            "BALANCE_WITHDRAW_FOR_SUBACCOUNT" => Some(Self::BalanceWithdrawForSubaccount),
            "BALANCE_DEPOSIT_TO_SUBACCOUNT" => Some(Self::BalanceDepositToSubaccount),
            "BALANCE_WITHDRAW_FROM_SUBACCOUNT" => Some(Self::BalanceWithdrawFromSubaccount),
            "BALANCE_DEPOSIT_FROM_SUBACCOUNT" => Some(Self::BalanceDepositFromSubaccount),
            "BALANCE_WITHDRAW_COPY_FEE" => Some(Self::BalanceWithdrawCopyFee),
            "BALANCE_WITHDRAW_INACTIVITY_FEE" => Some(Self::BalanceWithdrawInactivityFee),
            "BALANCE_DEPOSIT_TRANSFER" => Some(Self::BalanceDepositTransfer),
            "BALANCE_WITHDRAW_TRANSFER" => Some(Self::BalanceWithdrawTransfer),
            "BALANCE_DEPOSIT_CONVERTED_BONUS" => Some(Self::BalanceDepositConvertedBonus),
            "BALANCE_DEPOSIT_NEGATIVE_BALANCE_PROTECTION" => {
                Some(Self::BalanceDepositNegativeBalanceProtection)
            }
            _ => None,
        }
    }
}
/// Deal status ENUM
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaDealStatus {
    /// Deal filled.
    Filled = 2,
    /// Deal is partially filled.
    PartiallyFilled = 3,
    /// Deal is correct but was rejected by liquidity provider (e.g. no liquidity).
    Rejected = 4,
    /// Deal rejected by server (e.g. no price quotes).
    InternallyRejected = 5,
    /// Deal is rejected by LP due to error (e.g. symbol is unknown).
    Error = 6,
    /// Liquidity provider did not sent response on the deal during specified execution time period.
    Missed = 7,
}
impl ProtoOaDealStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaDealStatus::Filled => "FILLED",
            ProtoOaDealStatus::PartiallyFilled => "PARTIALLY_FILLED",
            ProtoOaDealStatus::Rejected => "REJECTED",
            ProtoOaDealStatus::InternallyRejected => "INTERNALLY_REJECTED",
            ProtoOaDealStatus::Error => "ERROR",
            ProtoOaDealStatus::Missed => "MISSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FILLED" => Some(Self::Filled),
            "PARTIALLY_FILLED" => Some(Self::PartiallyFilled),
            "REJECTED" => Some(Self::Rejected),
            "INTERNALLY_REJECTED" => Some(Self::InternallyRejected),
            "ERROR" => Some(Self::Error),
            "MISSED" => Some(Self::Missed),
            _ => None,
        }
    }
}
/// Bonus operation type ENUM
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoOaChangeBonusType {
    BonusDeposit = 0,
    BonusWithdraw = 1,
}
impl ProtoOaChangeBonusType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtoOaChangeBonusType::BonusDeposit => "BONUS_DEPOSIT",
            ProtoOaChangeBonusType::BonusWithdraw => "BONUS_WITHDRAW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BONUS_DEPOSIT" => Some(Self::BonusDeposit),
            "BONUS_WITHDRAW" => Some(Self::BonusWithdraw),
            _ => None,
        }
    }
}
