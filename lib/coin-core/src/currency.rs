use crate::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRatesFromBase {
    pub datetime: DateTime,
    pub base: Currency,
    pub rates: ExchangeRatesTo,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRatesToBase {
    pub datetime: DateTime,
    pub base: Currency,
    pub rates_to: ExchangeRatesTo,
    pub rates_from: ExchangeRatesFrom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRatesTo {
    pub target: Currency, // Euro
    pub eur: f64,
    pub aud_to_target: f64,
    pub bgn_to_target: f64,
    pub brl_to_target: f64,
    pub cad_to_target: f64,
    pub chf_to_target: f64,
    pub cny_to_target: f64,
    pub czk_to_target: f64,
    pub dkk_to_target: f64,
    pub gbp_to_target: f64,
    pub hkd_to_target: f64,
    pub huf_to_target: f64,
    pub idr_to_target: f64,
    pub ils_to_target: f64,
    pub inr_to_target: f64,
    pub isk_to_target: f64,
    pub jpy_to_target: f64,
    pub krw_to_target: f64,
    pub mxn_to_target: f64,
    pub myr_to_target: f64,
    pub nok_to_target: f64,
    pub nzd_to_target: f64,
    pub php_to_target: f64,
    pub pln_to_target: f64,
    pub ron_to_target: f64,
    pub sek_to_target: f64,
    pub sgd_to_target: f64,
    pub thb_to_target: f64,
    pub try_to_target: f64,
    pub usd_to_target: f64,
    pub zar_to_target: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRatesFrom {
    pub source: Currency, // Euro
    pub eur: f64,
    pub base_to_aud: f64, // 1.2 AUD
    pub base_to_bgn: f64,
    pub base_to_brl: f64,
    pub base_to_cad: f64,
    pub base_to_chf: f64,
    pub base_to_cny: f64,
    pub base_to_czk: f64,
    pub base_to_dkk: f64,
    pub base_to_gbp: f64,
    pub base_to_hkd: f64,
    pub base_to_huf: f64,
    pub base_to_idr: f64,
    pub base_to_ils: f64,
    pub base_to_inr: f64,
    pub base_to_isk: f64,
    pub base_to_jpy: f64,
    pub base_to_krw: f64,
    pub base_to_mxn: f64,
    pub base_to_myr: f64,
    pub base_to_nok: f64,
    pub base_to_nzd: f64,
    pub base_to_php: f64,
    pub base_to_pln: f64,
    pub base_to_ron: f64,
    pub base_to_sek: f64,
    pub base_to_sgd: f64,
    pub base_to_thb: f64,
    pub base_to_try: f64,
    pub base_to_usd: f64,
    pub base_to_zar: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Currency {
    AUD,
    BGN,
    BRL,
    CAD,
    CHF,
    CNY,
    CZK,
    DKK,
    EUR,
    GBP,
    HKD,
    HUF,
    IDR,
    ILS,
    INR,
    ISK,
    JPY,
    KRW,
    MXN,
    MYR,
    NOK,
    NZD,
    PHP,
    PLN,
    RON,
    SEK,
    SGD,
    THB,
    TRY,
    USD,
    ZAR,
}
