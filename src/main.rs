use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_subtp::SubtpPlugin;
mod nu_subtp;

fn main() {
    serve_plugin(&SubtpPlugin, MsgPackSerializer)
}
