use duplicate::duplicate_item;

use nu_protocol::{Record, Span, Value};

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
pub trait ToValue {
    fn to_value(&self, span: Span) -> Value;
}

impl ToValue for Record {
    fn to_value(&self, span: Span) -> Value {
        Value::record(self.clone(), span)
    }
}

#[duplicate_item(types; [String]; [str])]
impl ToValue for types {
    fn to_value(&self, span: Span) -> Value {
        Value::string(self, span)
    }
}

#[duplicate_item(types; [i32]; [u32])]
impl ToValue for types {
    fn to_value(&self, span: Span) -> Value {
        Value::Int {
            val: (*self as i64),
            internal_span: (span),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_value_for_i64() {
        assert_eq!(
            5.to_value(Span { start: 5, end: 6 }),
            Value::Int {
                val: 5,
                internal_span: Span { start: 5, end: 6 }
            }
        )
    }
    #[test]
    fn to_value_for_i32() {
        assert_eq!(
            (5i32).to_value(Span { start: 5, end: 6 }),
            Value::Int {
                val: 5,
                internal_span: Span { start: 5, end: 6 }
            }
        );
    }
}
impl ToValue for Vec<String> {
    fn to_value(&self, span: Span) -> Value {
        let a: Vec<Value> = self.iter().map(|text| text.to_value(span)).collect();
        a.to_value(span)
    }
}

impl ToValue for Vec<Value> {
    fn to_value(&self, span: Span) -> Value {
        Value::List {
            vals: self.to_vec(),
            internal_span: span,
        }
    }
}
