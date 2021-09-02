use crate::models::Timestamp;

pub trait PointSerialize {
    fn serialize(&self) -> String;
    fn serialize_with_timestamp(&self, timestamp: Option<Timestamp>) -> String;
}

// #[derive(InfluxDbWriteable)]
// struct ObligationRecord {
//     time: DateTime<Utc>,
//     slot: u64,
//     amount: u64,
//     market_value: Decimal,
//     #[influxdb(tag)] obligation_pubkey: String,
//     #[influxdb(tag)] token_name: String,
//     #[influxdb(tag)] is_deposit: bool
// }
//
// #[derive(InfluxDbWriteable)]
// struct EnrichedObligationRecord {
//     time: DateTime<Utc>,
//     sync_slot: u64,
//     last_update: u64,
//     risk_factor: u64,
//     deposit_value: Decimal,
//     borrow_value: Decimal,
//     owner: String,
//     allowed_borrow_value: Decimal,
//     unhealthy_borrow_value: Decimal,
//     #[influxdb(tag)] obligation_pubkey: String,
// }
//
// #[derive(InfluxDbWriteable)]
// struct WalletRecord {
//     time: DateTime<Utc>,
//     amount: u64,
//     #[influxdb(tag)] wallet_pubkey: String,
//     #[influxdb(tag)] wallet_token_name: String
// }
//
// #[derive(InfluxDbWriteable)]
// struct ReserveRecord {
//     time: DateTime<Utc>,
//     last_update: u64,
//     liquidity_available_amount: u64,
//     liquidity_borrow_amount: Decimal,
//     liquidity_borrow_rate_wads: Decimal,
//     liquidity_market_price: Decimal,
//     collateral_amount: u64,
//     #[influxdb(tag)] reserve_pubkey: String,
//     #[influxdb(tag)] reserve_name: String
// }