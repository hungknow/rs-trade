use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::HkError;

use super::Resolution;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct Candle {
    pub open_time: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: Option<f64>,
    pub trade_count: Option<f64>,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct TimestampValue {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Candles {
    // DESC order
    pub open_times: Vec<DateTime<Utc>>,
    pub opens: Vec<f64>,
    pub highs: Vec<f64>,
    pub lows: Vec<f64>,
    pub closes: Vec<f64>,
    pub volumes: Vec<Option<f64>>,
    pub trade_count: Vec<Option<f64>>,
    // the time of candle
    time_desc: Option<bool>,
    resolution: Option<Resolution>,
}

impl Candles {
    pub fn new() -> Candles {
        Candles {
            open_times: vec![],
            opens: vec![],
            highs: vec![],
            lows: vec![],
            closes: vec![],
            volumes: vec![],
            trade_count: vec![],
            time_desc: None,
            resolution: None,
        }
    }

    pub fn set_open_times(&mut self, open_times: Vec<DateTime<Utc>>) -> &mut Self {
        self.open_times = open_times;
        self
    }

    pub fn set_opens(&mut self, opens: Vec<f64>) -> &mut Self {
        self.opens = opens;
        self
    }

    pub fn set_highs(&mut self, highs: Vec<f64>) -> &mut Self {
        self.highs = highs;
        self
    }

    pub fn set_lows(&mut self, lows: Vec<f64>) -> &mut Self {
        self.lows = lows;
        self
    }

    pub fn set_closes(&mut self, closes: Vec<f64>) -> &mut Self {
        self.closes = closes;
        self
    }

    pub fn set_volumes(&mut self, volumes: Vec<Option<f64>>) -> &mut Self {
        self.volumes = volumes;
        self
    }

    pub fn time_desc(&self) -> Option<bool> {
        self.time_desc
    }

    pub fn resolution(&self) -> Option<Resolution> {
        self.resolution
    }

    fn detect_time_desc(&mut self) {
        if self.time_desc == None {
            self.time_desc = if self.open_times.len() > 1 {
                // If the time of the first candle is greater than the time of the second candle, it's DESC
                Some(self.open_times[0] > self.open_times[1])
            } else {
                None
            }
        }
    }

    fn detect_resolution(&mut self) {
        if self.resolution == None {
            self.resolution = if self.open_times.len() >= 2 {
                Resolution::from_seconds((self.open_times[0] - self.open_times[1]).num_seconds())
            } else {
                None
            }
        }
    }
    // Add a new candle to the data source
    // if the open_time of the candle is far from the last open_time, return error
    pub fn push_candle(&mut self, candle: &Candle) -> Result<&mut Self, HkError> {
        // check open_time of the new candle is the continuous of the last candle
        match self.time_desc() {
            Some(true) => {
                if self.open_times.len() > 0 {
                    let last_open_time = self.open_times[0];
                    if last_open_time.timestamp_millis() - candle.open_time.timestamp_millis() != 0
                    {
                        return Err(HkError::InvalidParameter);
                    }
                }
            }
            Some(false) => {
                if self.open_times.len() > 0 {
                    let last_open_time = self.open_times[self.open_times.len() - 1];
                    if last_open_time.timestamp_millis() - candle.open_time.timestamp_millis() != 0
                    {
                        return Err(HkError::HkDataError("".to_owned()));
                    }
                }
            }
            None => {
                if self.open_times.len() >= 2 {
                    return Err(HkError::HkDataError(
                        "failed to detect the time order for candles".to_owned(),
                    ));
                }
            }
        }

        self.open_times.push(candle.open_time);
        self.opens.push(candle.open);
        self.highs.push(candle.high);
        self.lows.push(candle.low);
        self.closes.push(candle.close);
        self.volumes.push(candle.volume);
        self.trade_count.push(candle.trade_count);

        self.detect_resolution();
        self.detect_time_desc();

        Ok(self)
    }

    #[inline]
    pub fn get_last_close_time(&self) -> Option<DateTime<Utc>> {
        if self.open_times.len() > 0 {
            Some(self.open_times[0])
        } else {
            None
        }
    }

    #[inline]
    pub fn get_oldest_close_time(&self) -> Option<DateTime<Utc>> {
        if self.open_times.len() > 0 {
            Some(self.open_times[self.open_times.len() - 1])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::Candles;

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
            open_times: close_time,
            opens: open,
            highs: high,
            lows: low,
            closes: close,
            volumes: volume,
            trade_count,
            time_desc: todo!(),
            resolution: todo!(),
        };

        // assert_eq!(candle_data_source.open, open);
        // assert_eq!(candle_data_source.high, high);
        // assert_eq!(candle_data_source.low, low);
        // assert_eq!(candle_data_source.close, close);
        // assert_eq!(candle_data_source.volume, volume);
        // assert_eq!(candle_data_source.trade_count, trade_count);
    }
}
