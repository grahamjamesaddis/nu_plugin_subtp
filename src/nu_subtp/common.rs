use std::time::Duration;

use duplicate::duplicate_item;

use nu_protocol::{Record, Span, Value};
use subtp::{srt::SrtTimestamp, vtt::VttTimestamp};

pub trait ToValue {
    fn to_value(&self, span: Span) -> Value;
}

impl ToValue for Record {
    fn to_value(&self, span: Span) -> Value {
        Value::record(self.clone(), span)
    }
}
impl ToValue for f32 {
    fn to_value(&self, span: Span) -> Value {
        Value::Float {
            val: *self as f64,
            internal_span: span,
        }
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
#[duplicate_item(types; [SrtTimestamp]; [VttTimestamp])]
impl ToValue for types {
    fn to_value(&self, span: Span) -> Value {
        let duration = Duration::new(
            ((self.hours * 60 + self.minutes) * 60 + self.seconds) as u64,
            self.milliseconds as u32 * 1_000_000,
        );
        let nanoseconds: i64 = duration.as_nanos() as i64;
        Value::duration(nanoseconds, span)
    }
}
