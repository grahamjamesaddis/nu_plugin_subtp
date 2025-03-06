use nu_plugin::{MsgPackSerializer, serve_plugin};
use nu_subtp::SubtpPlugin;
mod nu_subtp;

fn main() {
    serve_plugin(&SubtpPlugin, MsgPackSerializer)
}
