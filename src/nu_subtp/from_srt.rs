use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_protocol::{
    Category, ErrorLabel, Example, LabeledError, Signature, Span, Type, Value, record,
};

use super::SubtpPlugin;
use super::common::ToValue;

use subtp::srt::{LinePosition, SrtSubtitle, SubRip};
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
        example: "1 00:01:17,757 --> 00:01:18,757 Copy boy!

2 00:01:20,727 --> 00:01:23,662
Make it snappy.
Where's the rest of this story?

3 00:01:23,862 --> 00:01:25,091 Morning Post.

4 00:01:25,298 --> 00:01:27,960 City desk? Just a moment and I'll connect you.

5 00:01:28,268 --> 00:01:30,998 If anybody asks for me, I'm down at the courthouse.
",
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
        self.subtitles
            .iter()
            .map(|srt_subtitle| srt_subtitle.to_value(span))
            .collect::<Vec<Value>>()
            .to_value(span)
    }
}

impl ToValue for SrtSubtitle {
    fn to_value(&self, span: nu_protocol::Span) -> Value {
        let mut subtitle = record! {
            "Sequence"=> self.sequence.to_value(span),
            "Start"=> self.start.to_value(span),
            "End"=> self.end.to_value(span),
            "Text"=> self.text.to_value(span),
        };
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
