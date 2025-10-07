// "Sequence Number","Transaction Date","Site","Transaction Type","Remaining Trips","Transaction Value","PAYG Balance"
use dateparser::DateTimeUtc;

#[derive(Debug, serde::Deserialize)]
pub struct CSVGautrainRecord {
    #[serde(rename = "Sequence Number")]
    pub sequence_number: String,
    #[serde(rename = "Transaction Date")]
    pub transaction_date: String,
    #[serde(rename = "Site")]
    pub site: String,
    #[serde(rename = "Transaction Type")]
    pub transaction_type: String,
    #[serde(rename = "Remaining Trips")]
    pub remaining_trips: String,
    #[serde(rename = "Transaction Value")]
    pub transaction_value: String,
    #[serde(rename = "PAYG Balance")]
    pub payg_balance: String,
}

#[derive(Debug, Clone)]
pub struct GautrainRecord {
    pub transaction_date: DateTimeUtc,
    pub site: String,
    pub transaction_type: String,
    pub remaining_trips: i32,
    pub transaction_value: f32,
    pub payg_balance: f32,
}

impl From<CSVGautrainRecord> for GautrainRecord {
    fn from(record: CSVGautrainRecord) -> Self {
        GautrainRecord {
            transaction_date: record
                .transaction_date
                .parse::<DateTimeUtc>()
                .expect("Failed to parse date"),
            site: record.site,
            transaction_type: record.transaction_type,
            remaining_trips: record.remaining_trips.parse().unwrap_or_else(|_| {
                panic!(
                    "Failed to parse remaining trips: {}",
                    record.remaining_trips
                )
            }),
            transaction_value: record.transaction_value.parse().unwrap_or_else(|_| {
                panic!(
                    "Failed to parse transaction value: {}",
                    record.transaction_value
                )
            }),
            payg_balance: record.payg_balance.parse().unwrap_or_else(|_| {
                panic!("Failed to parse PAYG balance: {}", record.payg_balance)
            }),
        }
    }
}

impl GautrainRecord {
    pub fn is_topup(&self) -> bool {
        self.transaction_value > 0.0
    }

    pub fn is_same_date(&self, other: &GautrainRecord) -> bool {
        self.transaction_date.0.date_naive() == other.transaction_date.0.date_naive()
    }
}
