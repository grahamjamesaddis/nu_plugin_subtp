use std::{i64, time::Duration};

use hcl::value;
use nu_plugin::{EngineInterface, EvaluatedCall, Plugin, PluginCommand, SimplePluginCommand};
use nu_protocol::{
    Category, ErrorLabel, Example, LabeledError, Record, Signature, Span, Type, Value, record,
};
use subtp::vtt::{
    Alignment, Anchor, Line, LineAlignment, Position, PositionAlignment, Vertical, VttBlock,
    VttComment, VttCue, VttTimestamp, VttTimings, WebVtt,
};

pub struct SubtpPlugin;

impl Plugin for SubtpPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![Box::new(FromVtt), Box::new(FromSrt)]
    }
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
        run(call, input)
    }
}

struct NuValue {
    value: Value,
}

impl NuValue {
    fn from_web_vtt(item: &WebVtt, span: Span) -> Self {
        NuValue {
            value: convert_webvtt_to_value(item, span),
        }
    }

    fn from_vtt_timestamp(timestamp: VttTimestamp, span: Span) -> Self {
        let start: Duration = timestamp.into();
        let seconds: i64 = start.as_secs() as i64 * 1_000_000_000;
        NuValue {
            value: Value::duration(seconds, span),
        }
    }
    fn from_vtt_timings(timings: VttTimings, span: Span) -> Self {
        let mut timing_record = record!();

        timing_record.push(
            "Start Time",
            NuValue::from_vtt_timestamp(timings.start, span).value,
            // convert_vtt_timestanp_to_duration(timings.start, span),
        );
        timing_record.push(
            "End Time",
            NuValue::from_vtt_timestamp(timings.end, span).value,
            // convert_vtt_timestanp_to_duration(timings.end, span),
        );
        NuValue {
            value: Value::record(timing_record, span),
        }
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
        run(call, input)
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
        example: "'provider \"aws\" {
  region = \"us-east-1\"
}
resource \"aws_instance\" \"web\" {
  ami           = \"ami-a1b2c3d4\"
  instance_type = \"t2.micro\"
}' | from hcl",
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

fn run(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let span = call.head;
    let input_string = input.as_str()?;

    let parse_result: WebVtt =
        subtp::vtt::WebVtt::parse(input_string).map_err(|e| LabeledError {
            labels: Box::new(vec![ErrorLabel {
                text: "Error parsuing hcl".into(),
                span,
            }]),
            msg: e.to_string(),
            code: None,
            url: None,
            help: None,
            inner: Box::new(Vec::default()),
        })?;

    Ok(NuValue::from_web_vtt(&parse_result, span).value)
}

fn convert_vtt_comment_to_value(vtt_comment: &VttComment, span: Span) -> (String, Value) {
    (
        "Comment".to_string(),
        match vtt_comment {
            VttComment::Side(side) => Value::string(side, span),
            VttComment::Below(below) => Value::string(below, span),
        },
    )
}
fn convert_vtt_cue_to_value(vtt_cue: &VttCue, span: Span) -> Vec<(String, Value)> {
    let cue_values: Vec<(String, Value)> = Vec::new();

    if let Some(settings) = &vtt_cue.settings {}

    cue_values
}

fn convert_vtt_line_alignment_to_column(
    line_alignment: &LineAlignment,
    span: Span,
) -> (String, Value) {
    (
        "Line alignment".to_string(),
        match line_alignment {
            LineAlignment::Start => Value::string("Start".to_string(), span),
            LineAlignment::Center => Value::string("Center".to_string(), span),
            LineAlignment::End => Value::string("End".to_string(), span),
        },
    )
}

fn convert_vtt_position_alignment_to_value(
    position_alignment: &PositionAlignment,
    span: Span,
) -> Value {
    match position_alignment {
        PositionAlignment::LineLeft => Value::string("Line Left".to_string(), span),
        PositionAlignment::Center => Value::string("Center".to_string(), span),
        PositionAlignment::LineRight => Value::string("Line Right".to_string(), span),
    }
}

fn convert_vtt_position_to_values(position: Position, span: Span) -> Vec<(String, Value)> {
    let mut position_values: Vec<(String, Value)> = Vec::new();
    position_values.push((
        "Position perccntage".to_string(),
        Value::float(position.value.value.into(), span),
    ));
    if let Some(alignment) = position.alignment {
        position_values.push((
            "Position Alignment".to_string(),
            convert_vtt_position_alignment_to_value(&alignment, span),
        ))
    }

    position_values
}
fn convert_vtt_line_to_values(line: &Line, span: Span) -> Vec<(String, Value)> {
    let mut line_values: Vec<(String, Value)> = Vec::new();

    match line {
        Line::LineNumber(val, alignment_option) => {
            line_values.push((
                "Line number".to_string(),
                Value::int(val.clone() as i64, span),
            ));
            if let Some(alignment) = alignment_option {
                let (col, val) = convert_vtt_line_alignment_to_column(&alignment, span);
                line_values.push((col, val));
            }
        }
        Line::Percentage(percentage, alignment_option) => {
            line_values.push((
                "Line percentage".to_string(),
                Value::float(percentage.value.clone() as f64, span),
            ));
            if let Some(alignment) = alignment_option {
                let (col, val) = convert_vtt_line_alignment_to_column(&alignment, span);
                line_values.push((col, val));
            }
        }
    }

    line_values
}

fn convert_vtt_anchor_to_value(anchor: Anchor, span: Span) -> Value {
    let mut anchor_record = record!();

    anchor_record.push("x".to_string(), Value::float(anchor.x.value.into(), span));
    anchor_record.push("y".to_string(), Value::float(anchor.y.value.into(), span));
    Value::record(anchor_record, span)
}

pub fn convert_webvtt_to_value(value: &WebVtt, span: Span) -> Value {
    let mut subtitles: Vec<Value> = vec![];

    for vtt_block in &value.blocks {
        let mut rec = record!();
        match vtt_block {
            VttBlock::Comment(vtt_comment) => {
                let (col, val) = convert_vtt_comment_to_value(&vtt_comment, span);
                rec.push(col, val)
            }
            VttBlock::Que(val) => {
                if let Some(setting) = &val.settings {
                    if let Some(vertical) = &setting.vertical {
                        let val = match vertical {
                            Vertical::Lr => Value::string("Lr".to_string(), span),
                            Vertical::Rl => Value::string("Rl".to_string(), span),
                        };
                        rec.push("Vertical", val);
                    }
                    if let Some(line) = setting.line {
                        for (col, val) in convert_vtt_line_to_values(&line, span) {
                            rec.push(col, val);
                        }
                    }
                    if let Some(position) = setting.position {
                        for (col, val) in convert_vtt_position_to_values(position, span) {
                            rec.push(col, val);
                        }
                    }
                    // todo!("implement settings");
                }
                if let Some(identifier) = &val.identifier {
                    rec.push("Identifier", Value::string(identifier.clone(), span))
                }
                rec.push(
                    "Timings",
                    NuValue::from_vtt_timings(val.timings, span).value,
                );

                let payload: Vec<Value> =
                    val.payload.iter().map(|v| Value::string(v, span)).collect();

                rec.push("Payload", Value::list(payload, span));
            }
            VttBlock::Style(val) => {
                rec.push("Style", nu_protocol::Value::string(val.style.clone(), span))
            }
            VttBlock::Region(val) => {
                if let Some(id) = &val.id {
                    rec.push("Id", Value::string(id.clone(), span))
                }

                if let Some(percentage) = val.width {
                    rec.push("Width", Value::float(percentage.value.into(), span))
                }

                if let Some(lines) = val.lines {
                    rec.push("Lines", Value::int(lines.into(), span))
                }

                if let Some(region_anchor) = val.region_anchor {
                    rec.push(
                        "Region Anchor",
                        convert_vtt_anchor_to_value(region_anchor, span),
                    );
                }

                if let Some(viewport_anchor) = val.viewport_anchor {
                    rec.push(
                        "Viewport Anchor",
                        convert_vtt_anchor_to_value(viewport_anchor, span),
                    );
                }
            }
        }
        let val = Value::record(rec, span);
        subtitles.push(val);
    }

    let l = Value::list(subtitles, span);
    l
}
