use wasm_bindgen::prelude::*;
use web_time::{SystemTime, Duration};

/// Unitime is a library for handling time using WebAssembly.
#[wasm_bindgen]
pub struct Unitime {
    time: SystemTime
}

#[wasm_bindgen]
impl Unitime {
    /// Creates a new `Unitime` with the current time.
    /// # Examples
    /// ```
    /// const t = new Unitime();
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new() -> Unitime {
        Unitime { time: SystemTime::now() }
    }

    /// Creates a new `Unitime` with the given time in epoch milliseconds
    /// # Examples
    /// ```
    /// const t = new Unitime.fromEpochMil(1693470768154);
    /// ```
    #[wasm_bindgen(js_name = "fromEpochMil")]
    pub fn from_epoch_mil(&mut self, mil: f64) -> Unitime {
        self.time = SystemTime::UNIX_EPOCH + Duration::from_millis(mil as u64);
        Unitime { time: self.time }
    }

    /// Get the total number of elapsed hours since the stored time compared to the current time.
    /// # Examples
    /// ```
    /// const t = new Unitime();
    /// const hours = t.getElapsedHours();
    /// ```
    #[wasm_bindgen(js_name = "getElapsedHours")]
    pub fn get_elapsed_hours(&self) -> i32 {
        let now : SystemTime = SystemTime::now();
        let mut elapsed = now.duration_since(self.time).unwrap();
        let hours = elapsed.as_secs() / 3600;
        elapsed -= Duration::from_secs(hours * 3600);
        hours as i32
    }

    /// Get the number of elapsed minutes, not considering the elapsed hours, since the stored time compared to the current time.
    /// # Examples
    /// ```
    /// const t = new Unitime();
    /// const minutes = t.getElapsedMinutes();
    /// ```
    #[wasm_bindgen(js_name = "getElapsedMinutes")]
    pub fn get_elapsed_minutes(&self) -> i32 {
        let now : SystemTime = SystemTime::now();
        let mut elapsed = now.duration_since(self.time).unwrap();
        let hours = elapsed.as_secs() / 3600;
        elapsed -= Duration::from_secs(hours * 3600);
        let minutes = elapsed.as_secs() / 60;
        elapsed -= Duration::from_secs(minutes * 60);
        minutes as i32
    }

    /// Get the number of elapsed seconds, not considering the elapsed hours and minutes, since the stored time compared to the current time.
    /// # Examples
    /// ```
    /// const t = new Unitime();
    /// const seconds = t.getElapsedSeconds();
    /// ```
    #[wasm_bindgen(js_name = "getElapsedSeconds")]
    pub fn get_elapsed_seconds(&self) -> i32 {
        let now : SystemTime = SystemTime::now();
        let mut elapsed = now.duration_since(self.time).unwrap();
        let hours = elapsed.as_secs() / 3600;
        elapsed -= Duration::from_secs(hours * 3600);
        let minutes = elapsed.as_secs() / 60;
        elapsed -= Duration::from_secs(minutes * 60);
        elapsed.as_secs() as i32
    }

    /// Get the stored time in epoch milliseconds.
    /// # Examples
    /// ```
    /// const t = new Unitime();
    /// const mil = t.epochMil;
    /// ```
    #[wasm_bindgen(getter, js_name = "epochMil")]
    pub fn get_epoch_mil(&self) -> f64 {
        self.time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64
    }

    /// Get the stored time in epoch seconds.
    /// # Examples
    /// ```
    /// const t = new Unitime();
    /// const sec = t.epochSec;
    /// ```
    #[wasm_bindgen(getter, js_name = "epochSec")]
    pub fn get_epoch_sec(&self) -> f32 {
        self.time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs_f32()
    }

    /// Get the total number of elapsed seconds, including hours and minutes, since the stored time compared to the current time.
    /// # Examples
    /// ```
    /// const t = new Unitime();
    /// const totalSec = t.getTotalElapsedSec();
    /// ```
    #[wasm_bindgen(js_name = "getTotalElapsedSec")]
    pub fn get_total_elapsed_sec(&self) -> f64 {
        let now : SystemTime = SystemTime::now();
        let mut elapsed = now.duration_since(self.time).unwrap();
        let hours = elapsed.as_secs() / 3600;
        elapsed -= Duration::from_secs(hours * 3600);
        let minutes = elapsed.as_secs() / 60;
        elapsed -= Duration::from_secs(minutes * 60);
        elapsed.as_secs() as f64 + minutes as f64 * 60.0 + hours as f64 * 3600.0
    }

    /// Get the total number of elapsed minutes, including hours, since the stored time compared to the current time.
    /// # Examples
    /// ```
    /// const t = new Unitime();
    /// const totalMin = t.getTotalElapsedMin();
    /// ```
    #[wasm_bindgen(js_name = "getTotalElapsedMin")]
    pub fn get_total_elapsed_min(&self) -> f64 {
        let now : SystemTime = SystemTime::now();
        let mut elapsed = now.duration_since(self.time).unwrap();
        let hours = elapsed.as_secs() / 3600;
        elapsed -= Duration::from_secs(hours * 3600);
        let minutes = elapsed.as_secs() / 60;
        elapsed -= Duration::from_secs(minutes * 60);
        (hours * 60 + minutes) as f64
    }

    /// Get the duration from the stored time to the current time in HH:MM:SS format, where HH is only included if it is greater than 0.
    /// # Examples
    /// ```
    /// const t = new Unitime();
    /// const str = t.getElapsedStr();
    /// ```
    #[wasm_bindgen(js_name = "getElapsedStr")]
    pub fn get_elapsed_str(&self) -> String {
        let hours = self.get_elapsed_hours();
        let minutes = self.get_elapsed_minutes();
        let seconds = self.get_elapsed_seconds();

        let mut result = String::new();

        if hours != 0 {
            if hours < 10 {
                result.push_str("0");
            }
            result.push_str(&hours.to_string());
            result.push_str(":");
        }

        if minutes < 10 {
            result.push_str("0");
        }

        result.push_str(&minutes.to_string());
        result.push_str(":");
        if seconds < 10 {
            result.push_str("0");
        }
        result.push_str(&seconds.to_string());

        result
    }
}
