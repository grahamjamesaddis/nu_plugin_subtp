mod common;
mod srt;
mod vtt;

use crate::nu_subtp::srt::FromSrt;
use crate::nu_subtp::vtt::FromVtt;
use nu_plugin::Plugin;

pub struct SubtpPlugin;

impl Plugin for SubtpPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![Box::new(FromVtt), Box::new(FromSrt)]
    }
}
