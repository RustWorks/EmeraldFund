use anyhow::Result;
use polars::{df, frame::DataFrame};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use super::timestamp::get_unix_time;

pub fn generate_candles(seed: u64, len: usize) -> Result<DataFrame> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    let mut opens = Vec::with_capacity(len);
    let mut highs = Vec::with_capacity(len);
    let mut lows = Vec::with_capacity(len);
    let mut closes = Vec::with_capacity(len);
    let mut volumes = Vec::with_capacity(len);
    let mut timestamps = Vec::with_capacity(len);

    if len == 0 {
        return Ok(DataFrame::new(vec![])?);
    }

    // Start with a random price for the first candle
    let mut prev_close = rng.gen_range(0.0..1.0);
    let mut timestamp = get_unix_time(); // Starting timestamp

    for _ in 0..len {
        let open: f64 = prev_close;
        let close = (open + rng.gen_range(-0.05..0.05)).max(0.0); // Ensure price doesn't go negative
        let high = close + rng.gen_range(0.00..0.02);
        let low = close - rng.gen_range(0.00..0.02);
        let volume = rng.gen_range(50.0..150.0); // Random volume between 50 and 150

        opens.push(open);
        highs.push(high);
        lows.push(low);
        closes.push(close);
        volumes.push(volume);
        timestamps.push(timestamp);

        prev_close = close;
        timestamp += 60; // Increase timestamp by 1 minute
    }

    let df = df![
        "open" => opens,
        "high" => highs,
        "low" => lows,
        "close" => closes,
        "volume" => volumes,
        "timestamp" => timestamps,
    ]?;

    Ok(df)
}
