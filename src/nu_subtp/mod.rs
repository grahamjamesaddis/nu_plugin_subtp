use duplicate::duplicate_item;
use nu_plugin::{EngineInterface, EvaluatedCall, Plugin, PluginCommand, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, Record, Signature, Span, Type, Value, record};

mod srt;
mod vtt;
use srt::run_srt;
use vtt::run_vtt;
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

struct FromVtt;

impl SimplePluginCommand for FromVtt {
    type Plugin = SubtpPlugin;

    fn name(&self) -> &str {
        "from vtt"
    }

    fn signature(&self) -> nu_protocol::Signature {
        signature(PluginCommand::name(self))
    }

    fn description(&self) -> &str {
        "Parse text as .vtt and create records"
    }

    fn examples(&self) -> Vec<Example> {
        examples("Convert .vtt data into records")
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        run_vtt(call, input)
    }
}

pub struct FromSrt;

impl SimplePluginCommand for FromSrt {
    type Plugin = SubtpPlugin;

    fn name(&self) -> &str {
        "from srt"
    }

    fn signature(&self) -> nu_protocol::Signature {
        signature(PluginCommand::name(self))
    }

    fn description(&self) -> &str {
        "Parse text as .srt and create records"
    }

    fn examples(&self) -> Vec<Example> {
        examples("Convert .srt data into records")
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        run_srt(call, input)
    }
}

fn signature(name: &str) -> nu_protocol::Signature {
    Signature::build(name)
        .input_output_type(Type::String, Type::Record(Box::new([])))
        .category(Category::Formats)
}

fn examples(description: &str) -> Vec<Example> {
    let span = Span::test_data();
    let vec = vec![Example {
        description,
        example: "WEBVTT

00:01.000 --> 00:04.000
Never drink liquid nitrogen.

00:05.000 --> 00:09.000
— It will perforate your stomach.
— You could die.

00:10.000 --> 00:14.000
The Organisation for Sample Public Service Announcements accepts no liability for the content of this advertisement, or for the consequences of any actions taken on the basis of the information provided.
",
        result: Some(Value::record(
            record! {
                    "provider".to_string()=>Value::record(
                        record!{
                            "aws".to_string() => Value::record
                            (    record!{
                                "region".to_string()=>Value::test_string("us-east-1")
                            },span
                            )
                        }
                    ,span
                    ),

                    "resource".to_string()=> Value::record(
                        record!{"aws_instance".to_string()=>
                        Value::record (
                            record!{
                                "web".to_string()=>
                                Value::record(
                                    record! {
                                        "ami".to_string()=>Value::test_string("ami-a1b2c3d4"),
                                        "instance_type".to_string()=>Value::test_string("t2.micro"),
                                    },
                                    span,
                                )
                            },
                            span
                        )}
                        ,
                         span,
            )
                },
            span,
        )),
    }];
    vec
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
