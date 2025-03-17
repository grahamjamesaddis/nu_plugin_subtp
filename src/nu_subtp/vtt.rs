use nu_protocol::{Record, Span, Value, record};
use std::{i64, time::Duration};
use subtp::vtt::{
    Alignment, Anchor, CueSettings, Line, LineAlignment, Percentage, Position, PositionAlignment,
    Vertical, VttBlock, VttComment, VttCue, VttDescription, VttRegion, VttStyle, VttTimestamp,
    VttTimings, WebVtt,
};
pub struct NuValue;

pub trait ToValue {
    fn to_value(&self, span: Span) -> Value;
}

impl ToValue for WebVtt {
    fn to_value(&self, span: Span) -> Value {
        let mut vtt_rec = record!();
        if let Some(description) = &self.header.description {
            vtt_rec.push("Description", description.to_value(span));
        }
        vtt_rec.push("Blocks", self.blocks.to_value(span));

        vtt_rec.to_value(span)
    }
}

impl ToValue for Record {
    fn to_value(&self, span: Span) -> Value {
        Value::record(self.clone(), span)
    }
}

impl ToValue for VttDescription {
    fn to_value(&self, span: Span) -> Value {
        match self {
            VttDescription::Side(side) => side.to_value(span),
            VttDescription::Below(below) => below.to_value(span),
        }
    }
}

impl ToValue for String {
    fn to_value(&self, span: Span) -> Value {
        Value::string(self, span)
    }
}

impl ToValue for Vec<VttBlock> {
    fn to_value(&self, span: Span) -> Value {
        let mut subtitles: Vec<Value> = vec![];

        for vtt_block in self {
            subtitles.push(vtt_block.to_value(span));
        }
        Value::list(subtitles, span)
    }
}

impl ToValue for VttBlock {
    fn to_value(&self, span: Span) -> Value {
        match self {
            VttBlock::Comment(vtt_comment) => vtt_comment.to_value(span),
            VttBlock::Que(vtt_cue) => vtt_cue.to_value(span),
            VttBlock::Style(vtt_style) => vtt_style.to_value(span),
            VttBlock::Region(vtt_region) => vtt_region.to_value(span),
        }
    }
}
impl ToValue for VttComment {
    fn to_value(&self, span: Span) -> Value {
        Value::record(
            record! {
                "Comment".to_string()=>
                match self {
                    VttComment::Side(side) => side.to_value(span),
                    VttComment::Below(below) => below.to_value(span),
                }
            },
            span,
        )
    }
}

impl ToValue for CueSettings {
    fn to_value(&self, span: Span) -> Value {
        let mut rec = record!();
        if let Some(vertical) = &self.vertical {
            rec.push(
                "Vertical",
                match vertical {
                    Vertical::Lr => "Lr".to_string().to_value(span),
                    Vertical::Rl => "Rl".to_string().to_value(span),
                },
            );
        }
        if let Some(line) = self.line {
            rec.push("Line".to_string(), line.to_value(span));
        }
        if let Some(position) = self.position {
            rec.push("Position".to_string(), position.to_value(span));
        }
        if let Some(size) = self.size {
            rec.push("Size".to_string(), Value::float(size.value as f64, span));
        }
        if let Some(align) = self.align {
            rec.push(
                "Align".to_string(),
                match align {
                    Alignment::Center => "Center".to_string().to_value(span),
                    Alignment::Start => "Start".to_string().to_value(span),
                    Alignment::End => "End".to_string().to_value(span),
                    Alignment::Left => "Left".to_string().to_value(span),
                    Alignment::Right => "Right".to_string().to_value(span),
                },
            );
        }

        Value::record(rec, span)
    }
}
impl ToValue for VttCue {
    fn to_value(&self, span: Span) -> Value {
        let mut rec = record!();
        if let Some(cue_setting) = &self.settings {
            rec.push("Settings", cue_setting.to_value(span));
        }
        if let Some(identifier) = &self.identifier {
            rec.push("Identifier", identifier.to_value(span))
        }
        rec.push("Timings", self.timings.to_value(span));

        rec.push(
            "Payload",
            Value::list(
                self.payload.iter().map(|v| v.to_value(span)).collect(),
                span,
            ),
        );

        Value::record(rec, span)
    }
}

impl ToValue for Line {
    fn to_value(&self, span: Span) -> Value {
        match self {
            Line::LineNumber(val, line_alignment_option) => {
                NuValue::from_line_number(val, line_alignment_option, span)
            }
            Line::Percentage(percentage, line_alignment_option) => {
                NuValue::from_line_percentage(percentage, line_alignment_option, span)
            }
        }
    }
}

impl ToValue for Position {
    fn to_value(&self, span: Span) -> Value {
        let mut position_record = record!();
        position_record.push(
            "Position perccntage".to_string(),
            Value::float(self.value.value.into(), span),
        );
        if let Some(position_alignment) = self.alignment {
            position_record.push(
                "Position Alignment".to_string(),
                NuValue::from_vtt_position_alignment(&position_alignment, span),
            )
        }

        Value::record(position_record, span)
    }
}
impl ToValue for VttTimings {
    fn to_value(&self, span: Span) -> Value {
        Value::record(
            record! {
                "Start".to_string()=>self.start.to_value(span),
                "End".to_string()=>self.end.to_value(span),
            },
            span,
        )
    }
}
impl ToValue for VttStyle {
    fn to_value(&self, span: Span) -> Value {
        let mut rec = record!();
        rec.push("Style", Value::string(self.style.clone(), span));
        Value::record(rec, span)
    }
}

impl ToValue for VttRegion {
    fn to_value(&self, span: Span) -> Value {
        let mut rec = record!();
        if let Some(id) = &self.id {
            rec.push("Id", Value::string(id.clone(), span))
        }

        if let Some(percentage) = self.width {
            rec.push("Width", Value::float(percentage.value.into(), span))
        }

        if let Some(lines) = self.lines {
            rec.push("Lines", Value::int(lines.into(), span))
        }

        if let Some(region_anchor) = self.region_anchor {
            rec.push("Region Anchor", region_anchor.to_value(span));
        }

        if let Some(viewport_anchor) = self.viewport_anchor {
            rec.push("Viewport Anchor", viewport_anchor.to_value(span));
        }

        Value::record(rec, span)
    }
}

impl ToValue for Anchor {
    fn to_value(&self, span: Span) -> Value {
        Value::record(
            record! {
                "x".to_string()=>Value::float(self.x.value.into(), span),
                "y".to_string()=>Value::float(self.y.value.into(), span),
            },
            span,
        )
    }
}

impl ToValue for VttTimestamp {
    fn to_value(&self, span: Span) -> Value {
        let duration = Duration::new(
            ((self.hours * 60 + self.minutes) * 60 + self.seconds) as u64,
            self.milliseconds as u32 * 1_000_000,
        );
        let nanoseconds: i64 = (duration.as_secs() * 1_000_000_000) as i64;
        Value::duration(nanoseconds, span)
    }
}

impl NuValue {
    fn from_vtt_line_alignment(line_alignment: &LineAlignment, span: Span) -> Value {
        match line_alignment {
            LineAlignment::Start => Value::string("Start".to_string(), span),
            LineAlignment::Center => Value::string("Center".to_string(), span),
            LineAlignment::End => Value::string("End".to_string(), span),
        }
    }
    fn from_vtt_position_alignment(position_alignment: &PositionAlignment, span: Span) -> Value {
        match position_alignment {
            PositionAlignment::LineLeft => Value::string("Line Left".to_string(), span),
            PositionAlignment::Center => Value::string("Center".to_string(), span),
            PositionAlignment::LineRight => Value::string("Line Right".to_string(), span),
        }
    }

    fn from_line_number(
        line_number_value: &i32,
        line_alignment_option: &Option<LineAlignment>,
        span: Span,
    ) -> Value {
        let mut line_number = record!();
        line_number.push(
            "Number".to_string(),
            Value::int(line_number_value.clone() as i64, span),
        );
        if let Some(line_alignment) = line_alignment_option {
            line_number.push(
                "Alignment".to_string(),
                NuValue::from_vtt_line_alignment(&line_alignment, span),
            );
        }
        Value::record(line_number, span)
    }
    fn from_line_percentage(
        percentage: &Percentage,
        line_alignment_option: &Option<LineAlignment>,
        span: Span,
    ) -> Value {
        let mut line_number = record!();
        line_number.push(
            "Percentage".to_string(),
            Value::float(percentage.value.clone() as f64, span),
        );
        if let Some(line_alignment) = line_alignment_option {
            line_number.push(
                "Alignment".to_string(),
                NuValue::from_vtt_line_alignment(&line_alignment, span),
            );
        }
        Value::record(line_number, span)
    }
}
