use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_protocol::{
    Category, ErrorLabel, Example, LabeledError, Record, Signature, Span, Type, Value, record,
};

use super::SubtpPlugin;
use super::common::ToValue;

use subtp::srt::{SrtSubtitle, SrtTimestamp, SubRip};
pub struct ToSrt;

trait ToSubRip {
    fn to_sub_rip(&self, span: Span) -> Result<SubRip, LabeledError>;
}

trait ToSrtSubtitle {
    fn to_sub_rip(&self, span: Span) -> Result<SrtSubtitle, LabeledError>;
}

impl SimplePluginCommand for ToSrt {
    type Plugin = SubtpPlugin;

    fn name(&self) -> &str {
        "to srt"
    }

    fn signature(&self) -> nu_protocol::Signature {
        signature(PluginCommand::name(self))
    }

    fn description(&self) -> &str {
        "Create .srt file from nushell records"
    }

    // fn examples(&self) -> Vec<Example> {
    //     examples("Convert .srt data into records")
    // }

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
        .input_output_type(Type::Table(Box::new([])), Type::String)
        .category(Category::Formats)
}

const EX_TXT: &str = "
1 00:01:17,757 --> 00:01:18,757 Copy boy!

2 00:01:20,727 --> 00:01:23,662
Make it snappy.
Where's the rest of this story?

3 00:01:23,862 --> 00:01:25,091 Morning Post.

4 00:01:25,298 --> 00:01:27,960 City desk? Just a moment and I'll connect you.

5 00:01:28,268 --> 00:01:30,998 If anybody asks for me, I'm down at the courthouse.
";

fn examples(description: &str) -> Vec<Example> {
    let span = Span::test_data();
    let vec = vec![Example {
        description,
        example: EX_TXT,
        result: Some(Value::list(
            vec![
                Value::record(
                    record! {
                        "Sequence"=>Value::int(1, span),
                        "Start"=>Value::duration(((1*60+17)*1000+757)*1_000_000, span),
                        "End"=>Value::duration(((1*60+18)*1000+757)*1_000_000, span),
                        "Text"=>Value::List { vals: vec![
                            Value::String { val:"Make it snappy.".to_string(), internal_span: span },
                            Value::String { val:"Where's the rest of this story?".to_string(), internal_span: span },
                            ], internal_span: span }
                    },
                    span,
                ),
                Value::record(
                    record! {
                        "Sequence"=>Value::int(2, span),
                        "Start"=>Value::duration(((1*60+20)*1000+727)*1_000_000, span),
                        "End"=>Value::duration(((1*60+23)*1000+662)*1_000_000, span),
                        "Text"=>Value::List { vals: vec![
                            Value::String { val:"Copy boy!".to_string(), internal_span: Span { start: 1, end: 2 } }
                            ], internal_span: span }
                    },
                    span,
                ),
                Value::record(
                    record! {
                        "Sequence"=>Value::int(3, span),
                        "Start"=>Value::duration(((1*60+23)*1000+862)*1000_000, span),
                        "End"=>Value::duration(((1*60+25)*1000+91)*1_000_000, span),
                        "Text"=>Value::List { vals: vec![
                            Value::String { val:"Morning Post.".to_string(), internal_span: Span { start: 1, end: 2 } }
                            ], internal_span: span }
                    },
                    span,
                ),
                Value::record(
                    record! {
                        "Sequence"=>Value::int(4, span),
                        "Start"=>Value::duration(((1*60+25)*1000+298)*1000_000, span),
                        "End"=>Value::duration(((1*60+27)*1000+960)*1_000_000, span),
                        "Text"=>Value::List { vals: vec![
                            Value::String { val:"City desk? Just a moment and I'll connect you.".to_string(), internal_span: Span { start: 1, end: 2 } }
                            ], internal_span: span }
                    },
                    span,
                ),
                Value::record(
                    record! {
                        "Sequence"=>Value::int(5, span),
                        "Start"=>Value::duration(((1*60+28)*1000+263)*1000_000, span),
                        "End"=>Value::duration(((1*60+30)*1000+998)*1_000_000, span),
                        "Text"=>Value::List { vals: vec![
                            Value::String { val:"If anybody asks for me, I'm down at the courthouse.".to_string(), internal_span: Span { start: 1, end: 2 } }
                            ], internal_span: span }
                    },
                    span,
                ),
            ],
            span,
        )),
    }];
    vec
}

fn run(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let span = call.head;

    match input {
        Value::List {
            vals,
            internal_span,
        } => Ok(vals
            .to_sub_rip(*internal_span)?
            .render()
            .to_value(*internal_span)),
        _ => Err(srt_construction_error(
            "Input inconsistent with srt format",
            span,
        )),
    }
}

fn timestamp_from_duration(val: i64) -> SrtTimestamp {
    let hours = (val / (60 * 60 * 1_000_000_000)) as u8;
    let rest = val % (60 * 60 * 1_000_000_000);
    let minutes = (rest / (60 * 1_000_000_000)) as u8;
    let rest = rest % (60 * 1_000_000_000);
    let seconds = (rest / (1_000_000_000)) as u8;
    let rest = rest % (1_000_000_000);
    let milliseconds = (rest / (1_000_000)) as u16;
    SrtTimestamp {
        hours: hours,
        minutes: minutes,
        seconds: seconds,
        milliseconds: milliseconds,
    }
}
fn srt_construction_error(msg: &str, span: Span) -> LabeledError {
    LabeledError {
        labels: Box::new(vec![ErrorLabel {
            text: msg.into(),
            span,
        }]),
        msg: "Error constructing srt".to_string(),
        code: None,
        url: None,
        help: None,
        inner: Box::new(Vec::default()),
    }
}

trait FromValue {
    fn get_subtitle(&self, span: Span) -> Result<SrtSubtitle, LabeledError>;
    fn get_sequence(&self) -> Option<u32>;
    fn get_start(&self) -> Option<SrtTimestamp>;
    fn get_end(&self) -> Option<SrtTimestamp>;
    fn get_text(&self) -> Option<Vec<String>>;
}

impl FromValue for Value {
    fn get_subtitle(&self, span: Span) -> Result<SrtSubtitle, LabeledError> {
        match self {
            Value::Record { val, internal_span } => val.get_subtitle(*internal_span),
            _ => Err(srt_construction_error(
                "Input inconsistent with srt format",
                span,
            )),
        }
    }
    fn get_sequence(&self) -> Option<u32> {
        match self {
            Value::Int {
                val,
                internal_span: _,
            } => Some(*val as u32),
            _ => None,
        }
    }
    fn get_start(&self) -> Option<SrtTimestamp> {
        match self {
            Value::Duration {
                val,
                internal_span: _,
            } => Some(timestamp_from_duration(*val)),
            _ => None,
        }
    }
    fn get_end(&self) -> Option<SrtTimestamp> {
        match self {
            Value::Duration {
                val,
                internal_span: _,
            } => Some(timestamp_from_duration(*val)),
            _ => None,
        }
    }
    fn get_text(&self) -> Option<Vec<String>> {
        match self {
            Value::List {
                vals,
                internal_span: _,
            } => text_from_list(vals),
            _ => None,
        }
    }
}
trait FromRecord {
    fn get_subtitle(&self, span: Span) -> Result<SrtSubtitle, LabeledError>;
    fn get_sequence(&self) -> Option<u32>;
    fn get_start(&self) -> Option<SrtTimestamp>;
    fn get_end(&self) -> Option<SrtTimestamp>;
    fn get_text(&self) -> Option<Vec<String>>;
}

impl FromRecord for Record {
    fn get_subtitle(&self, span: Span) -> Result<SrtSubtitle, LabeledError> {
        let Some(sequence) = self.get_sequence() else {
            return Err(srt_construction_error(
                "Sequence number not in record",
                span,
            ));
        };
        let Some(start_time) = self.get_start() else {
            return Err(srt_construction_error("Start time not in record", span));
        };
        let Some(end_time) = self.get_end() else {
            return Err(srt_construction_error("End time not in record", span));
        };
        let Some(text) = self.get_text() else {
            return Err(srt_construction_error("Text not in record", span));
        };
        Ok(SrtSubtitle {
            sequence: sequence,
            start: start_time,
            end: end_time,
            text: text,
            line_position: None,
        })
    }
    fn get_sequence(&self) -> Option<u32> {
        self.iter()
            .find(|(s, _)| *s == "Sequence")
            .map(|(_, v)| v.get_sequence())
            .unwrap_or_default()
    }
    fn get_start(&self) -> Option<SrtTimestamp> {
        self.iter()
            .find(|(s, _)| *s == "Start")
            .map(|(_, v)| v.get_start())
            .unwrap_or_default()
    }
    fn get_end(&self) -> Option<SrtTimestamp> {
        self.iter()
            .find(|(s, _)| *s == "End")
            .map(|(_, v)| v.get_end())
            .unwrap_or_default()
    }
    fn get_text(&self) -> Option<Vec<String>> {
        self.iter()
            .find(|(s, _)| *s == "Text")
            .map(|(_, v)| v.get_text())
            .unwrap_or_default()
    }
}

fn text_from_list(list: &Vec<Value>) -> Option<Vec<String>> {
    list.iter()
        .map(|val| match val {
            Value::String {
                val,
                internal_span: _,
            } => Some(val.to_string()),
            _ => None,
        })
        .collect()
}

impl ToSubRip for Vec<Value> {
    fn to_sub_rip(&self, span: Span) -> Result<SubRip, nu_protocol::LabeledError> {
        let subtitle_list = self.iter().to_owned().map(|val| val.get_subtitle(span));
        Ok(SubRip {
            subtitles: subtitle_list.collect::<Result<Vec<SrtSubtitle>, LabeledError>>()?,
        })
    }
}
