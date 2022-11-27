use tracing::Level as Level_Tracing;

use crate::Level;

pub fn get_level(level: Level) -> Level_Tracing {
  match level {
    Level::Trace => Level_Tracing::TRACE,
    Level::Debug => Level_Tracing::DEBUG,
    Level::Info => Level_Tracing::INFO,
    Level::Warn => Level_Tracing::WARN,
    Level::Error => Level_Tracing::ERROR,
  }
}
