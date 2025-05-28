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
        let minutes: u64 = self.hours as u64 * 60 + self.minutes as u64;
        let duration = Duration::new(
            minutes * 60 + self.seconds as u64,
            self.milliseconds as u32 * 1_000_000,
        );
        let nanoseconds: i64 = duration.as_nanos() as i64;
        Value::duration(nanoseconds, span)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn to_value_for_f32() {
        assert_eq!(
            5f32.to_value(Span { start: 5, end: 6 }),
            Value::Float {
                val: 5f64,
                internal_span: Span { start: 5, end: 6 }
            }
        )
    }

    #[test]
    fn to_value_for_str() {
        assert_eq!(
            "Hello".to_value(Span { start: 5, end: 6 }),
            Value::String {
                val: "Hello".to_string(),
                internal_span: Span { start: 5, end: 6 }
            }
        )
    }

    #[test]
    fn to_value_for_string() {
        assert_eq!(
            "Hello".to_string().to_value(Span { start: 5, end: 6 }),
            Value::String {
                val: "Hello".to_string(),
                internal_span: Span { start: 5, end: 6 }
            }
        );
    }

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
            5i32.to_value(Span { start: 5, end: 6 }),
            Value::Int {
                val: 5,
                internal_span: Span { start: 5, end: 6 }
            }
        );
    }

    #[test]
    fn to_value_from_vec_string() {
        assert_eq!(
            vec![
                "hello".to_string().to_value(Span { start: 1, end: 2 }),
                "mum".to_string().to_value(Span { start: 1, end: 2 }),
            ],
            vec![
                Value::String {
                    val: "hello".to_string(),
                    internal_span: Span { start: 1, end: 2 }
                },
                Value::String {
                    val: "mum".to_string(),
                    internal_span: Span { start: 1, end: 2 }
                },
            ]
        )
    }
    #[test]
    fn to_value_for_vec_value() {
        assert_eq!(
            vec![
                Value::String {
                    val: "a".to_string(),
                    internal_span: Span { start: 5, end: 6 }
                },
                Value::String {
                    val: "b".to_string(),
                    internal_span: Span { start: 7, end: 8 }
                },
            ]
            .to_value(Span { start: 1, end: 2 }),
            Value::List {
                vals: vec![
                    Value::String {
                        val: "a".to_string(),
                        internal_span: Span { start: 5, end: 6 }
                    },
                    Value::String {
                        val: "b".to_string(),
                        internal_span: Span { start: 7, end: 8 }
                    },
                ],
                internal_span: Span { start: 1, end: 2 }
            }
        )
    }

    #[test]
    fn to_value_for_srt_timestamp() {
        let hours: u8 = 1;
        let minutes: u8 = 2;
        let seconds: u8 = 3;
        let milliseconds: u16 = 4;

        let nanoseconds: i64 = (((hours as i64 * 60 + minutes as i64) * 60 + seconds as i64)
            * 1_000
            + milliseconds as i64)
            * 1_000_000;

        assert_eq!(
            SrtTimestamp {
                hours: hours,
                minutes: minutes,
                seconds: seconds,
                milliseconds: milliseconds,
            }
            .to_value(Span { start: 1, end: 2 }),
            Value::Duration {
                val: nanoseconds,
                internal_span: Span { start: 1, end: 1 }
            }
        )
    }
    #[test]
    fn to_value_for_vtt_timestamp() {
        let hours: u8 = 1;
        let minutes: u8 = 2;
        let seconds: u8 = 3;
        let milliseconds: u16 = 4;

        let nanoseconds: i64 = (((hours as i64 * 60 + minutes as i64) * 60 + seconds as i64)
            * 1_000
            + milliseconds as i64)
            * 1_000_000;

        assert_eq!(
            VttTimestamp {
                hours: hours,
                minutes: minutes,
                seconds: seconds,
                milliseconds: milliseconds,
            }
            .to_value(Span { start: 1, end: 2 }),
            Value::Duration {
                val: nanoseconds,
                internal_span: Span { start: 1, end: 1 }
            }
        )
    }
}
