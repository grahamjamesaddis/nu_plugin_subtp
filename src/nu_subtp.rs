mod common;
mod from_srt;
mod from_vtt;
mod to_srt;

use crate::nu_subtp::from_srt::FromSrt;
use crate::nu_subtp::from_vtt::FromVtt;
use crate::nu_subtp::to_srt::ToSrt;
use nu_plugin::Plugin;

pub struct SubtpPlugin;

impl Plugin for SubtpPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![Box::new(FromVtt), Box::new(FromSrt), Box::new(ToSrt)]
    }
}
