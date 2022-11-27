#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::ToNapiValue;
use tracing::Level as Level_Tracing;
use tracing_subscriber::FmtSubscriber;

#[napi]
pub enum Level {
  Trace,
  Debug,
  Info,
  Warn,
  Error,
}

#[napi]
pub struct Tracing {
  pub level: Option<Level>,
}

#[napi]
impl Tracing {
  #[napi]
  pub fn config(&mut self, level: Level) {
    let lvl = match level {
      Level::Trace => Level_Tracing::TRACE,
      Level::Debug => Level_Tracing::DEBUG,
      Level::Info => Level_Tracing::INFO,
      Level::Warn => Level_Tracing::WARN,
      Level::Error => Level_Tracing::ERROR,
    };
    let subscriber = FmtSubscriber::builder().with_max_level(lvl).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    self.level = Some(level);
  }

  #[napi(constructor)]
  pub fn new() -> Self {
    Tracing { level: None }
  }
}
