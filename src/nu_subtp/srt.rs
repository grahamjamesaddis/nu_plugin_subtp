use std::time::Duration;

use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_protocol::{Category, ErrorLabel, Example, LabeledError, Signature, Type, Value};
use nu_protocol::{Span, record};
use subtp::srt::{LinePosition, SubRip};
use subtp::srt::{SrtSubtitle, SrtTimestamp};

use super::ToValue;

use super::SubtpPlugin;
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

pub fn run_srt(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let span = call.head;
    let input_string = input.as_str()?;

    let parse_result: SubRip =
        subtp::srt::SubRip::parse(input_string).map_err(|e| LabeledError {
            labels: Box::new(vec![ErrorLabel {
                text: "Error parsing srt".into(),
                span,
            }]),
            msg: e.to_string(),
            code: None,
            url: None,
            help: None,
            inner: Box::new(Vec::default()),
        })?;

    // Ok(NuValue::from_web_vtt(&parse_result, span))
    Ok(parse_result.to_value(span))
}

impl ToValue for SubRip {
    fn to_value(&self, span: nu_protocol::Span) -> Value {
        Value::list(
            self.subtitles
                .iter()
                .map(|srt_subtitle| srt_subtitle.to_value(span))
                .collect(),
            span,
        )
    }
}

impl ToValue for SrtSubtitle {
    fn to_value(&self, span: nu_protocol::Span) -> Value {
        let mut subtitle = record!();

        subtitle.push("Sequence", self.sequence.to_value(span));
        subtitle.push("Stqrt", self.start.to_value(span));
        subtitle.push("End", self.end.to_value(span));
        subtitle.push("Text", self.text.to_value(span));
        if let Some(line_position) = self.line_position {
            subtitle.push("Line Position", line_position.to_value(span));
        }

        subtitle.to_value(span)
    }
}
impl ToValue for LinePosition {
    fn to_value(&self, span: Span) -> Value {
        record! {
            "x1"=>self.x1.to_value(span),
            "x2"=>self.x2.to_value(span),
            "y1"=>self.y1.to_value(span),
            "y2"=>self.y2.to_value(span),
        }
        .to_value(span)
    }
}
impl ToValue for SrtTimestamp {
    fn to_value(&self, span: Span) -> Value {
        let duration = Duration::new(
            ((self.hours * 60 + self.minutes) * 60 + self.seconds) as u64,
            self.milliseconds as u32 * 1_000_000,
        );
        let nanoseconds: i64 = (duration.as_secs() * 1_000_000_000) as i64;
        Value::duration(nanoseconds, span)
    }
}
