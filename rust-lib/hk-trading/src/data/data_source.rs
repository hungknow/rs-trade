use async_trait::async_trait;
use std::sync::Arc;

use chrono::{DateTime, Utc};

use crate::errors::TaError;

use super::Candle;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum Resolution {
    #[default]
    M1,
    M5,
    M15,
    M30,
    H1,
    H4,
    D1,
    W1,
}

impl Resolution {
    pub fn to_seconds(&self) -> i64 {
        match self {
            Resolution::M1 => 60,
            Resolution::M5 => 300,
            Resolution::M15 => 900,
            Resolution::M30 => 1800,
            Resolution::H1 => 3600,
            Resolution::H4 => 14400,
            Resolution::D1 => 86400,
            Resolution::W1 => 604800,
        }
    }

    pub fn to_milliseconds(&self) -> i64 {
        self.to_seconds() * 1000
    }

    pub fn from_seconds(seconds: i64) -> Option<Resolution> {
        match seconds {
            60 => Some(Resolution::M1),
            300 => Some(Resolution::M5),
            900 => Some(Resolution::M15),
            1800 => Some(Resolution::M30),
            3600 => Some(Resolution::H1),
            14400 => Some(Resolution::H4),
            86400 => Some(Resolution::D1),
            604800 => Some(Resolution::W1),
            _ => None,
        }
    }
}

pub struct DataSourceMeta {
    pub symbol: String,
    // pub source: String,
    pub resolution: Resolution,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

pub struct DataSourceGet<'a> {
    pub symbol: &'a str,
    pub resolution: Resolution,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait CandleDataSource {
    fn get_metadata(self) -> Vec<DataSourceMeta>;
    async fn get_data_source_from<'a>(&self, option: DataSourceGet<'a>) -> Result<Candles, TaError> ;
    // fn register_vent(&self, sender mpsc::Sender<CandleDisplayDataSourceEvent>);
}

#[derive(Clone, PartialEq, Debug)]
pub struct CandleDisplayDataSourceState {
    pub symbol: String,
    pub resolution: Resolution,
    pub display_start_time: DateTime<Utc>,
    pub display_end_time: DateTime<Utc>,
    // pub data_start_time: DateTime<Utc>,
    // pub data_end_time: DateTime<Utc>,
    pub candles: Arc<Box<Candles>>,

    pub loading_data_time: Option<(DateTime<Utc>, DateTime<Utc>)>,
    // pub loading_data_end_time: Option<DateTime<Utc>>,
    // pub is_loading_data: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CandlesNewChunkEvent {
    pub old_state: Arc<CandleDisplayDataSourceState>,
    pub new_state: Arc<CandleDisplayDataSourceState>,
    pub new_candle_index: usize,
}

pub struct CandlesCandlesUpdatedEvent {
    pub old_state: Arc<CandleDisplayDataSourceState>,
    pub new_state: Arc<CandleDisplayDataSourceState>,
    pub candle_index: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DisplayTimeRangeChangedEvent {
    pub old_state: Arc<CandleDisplayDataSourceState>,
    pub new_state: Arc<CandleDisplayDataSourceState>,
    pub from_time: DateTime<Utc>,
    pub to_time: DateTime<Utc>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CandleUpdatedEvent {
    pub old_state: Arc<CandleDisplayDataSourceState>,
    pub new_state: Arc<CandleDisplayDataSourceState>,
    pub index: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub enum CandleDisplayDataSourceEvent {
    DisplayTimeRangeChanged(DisplayTimeRangeChangedEvent),
    CandleUpdated(CandleUpdatedEvent),
    CandlesNewChunk(CandlesNewChunkEvent),
}

// impl Identifier for CandleDisplayDataSourceEvent {
//     fn id(&self) -> &str {
//         match self {
//             CandleDisplayDataSourceEvent::DisplayTimeRangeChanged(_) => {
//                 "DisplayTimeRangeChanged".to_string()
//             }
//             CandleDisplayDataSourceEvent::CandleUpdated(_) => "CandleUpdated".to_string(),
//             CandleDisplayDataSourceEvent::CandlesNewChunk(_) => "CandlesNewChunk".to_string(),
//         }
//     }
// }

pub enum CandleDisplayDataSourceError {}
// One source for symbol
pub trait CandleDisplayDataSource {
    // fn get_state(self) -> Arc<CandleDisplayDataSourceState>;
    fn set_display_range(
        state: Arc<CandleDisplayDataSourceState>,
        from_time: DateTime<Utc>,
        to_time: DateTime<Utc>,
    ) -> Result<CandleDisplayDataSourceEvent, CandleDisplayDataSourceError>;
    // fn on(
    //     &mut self,
    //     event_name: CandleDisplayDataSourceEventName,
    //     callback: fn(CandleDisplayDataSourceEvent) -> (),
    // );
    // fn get_display_data(self) -> CandleData<'static>;
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Candles {
    // DESC order
    pub open_time: Vec<DateTime<Utc>>,
    pub open: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub close: Vec<f64>,
    pub volume: Vec<Option<f64>>,
    pub trade_count: Vec<Option<f64>>,
}

impl Candles {
    pub fn new() -> Candles {
        Candles {
            open_time: vec![],
            open: vec![],
            high: vec![],
            low: vec![],
            close: vec![],
            volume: vec![],
            trade_count: vec![],
        }
    }

    pub fn push_candle(&mut self, candle: &Candle) {
        self.open_time.push(candle.open_time);
        self.open.push(candle.open);
        self.high.push(candle.high);
        self.low.push(candle.low);
        self.close.push(candle.close);
        self.volume.push(candle.volume);
        self.trade_count.push(candle.trade_count);
    }

    #[inline]
    pub fn get_last_close_time(&self) -> Option<DateTime<Utc>> {
        if self.open_time.len() > 0 {
            Some(self.open_time[0])
        } else {
            None
        }
    }

    #[inline]
    pub fn get_oldest_close_time(&self) -> Option<DateTime<Utc>> {
        if self.open_time.len() > 0 {
            Some(self.open_time[self.open_time.len() - 1])
        } else {
            None
        }
    }
}

// Test, generate candle data source
// Notify if there's new

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candle_data_source() {
        let open = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let high = vec![1.1, 2.1, 3.1, 4.1, 5.1];
        let low = vec![0.9, 1.9, 2.9, 3.9, 4.9];
        let close = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let volume = vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)];
        let trade_count = vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)];
        let close_time = vec![Utc::now(), Utc::now(), Utc::now(), Utc::now(), Utc::now()];

        let candle_data_source = Candles {
            open_time: close_time,
            open,
            high,
            low,
            close,
            volume,
            trade_count,
        };

        // assert_eq!(candle_data_source.open, open);
        // assert_eq!(candle_data_source.high, high);
        // assert_eq!(candle_data_source.low, low);
        // assert_eq!(candle_data_source.close, close);
        // assert_eq!(candle_data_source.volume, volume);
        // assert_eq!(candle_data_source.trade_count, trade_count);
    }
}
