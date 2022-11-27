#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::ToNapiValue;
use tracing_subscriber::FmtSubscriber;

mod helper;
use helper::get_level;

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
    let lvl = get_level(level);
    let subscriber = FmtSubscriber::builder().with_max_level(lvl).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    self.level = Some(level);
  }

  #[napi(constructor)]
  pub fn new() -> Self {
    Tracing { level: None }
  }
}
