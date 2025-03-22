use std::time::Duration;

use nu_plugin::EvaluatedCall;
use nu_protocol::{ErrorLabel, LabeledError, Value};
use nu_protocol::{Span, record};
use subtp::srt::{LinePosition, SubRip};
use subtp::srt::{SrtSubtitle, SrtTimestamp};

use super::ToValue;
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
