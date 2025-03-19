use nu_protocol::{Record, Span, Value, record};
use std::{i64, time::Duration};
use subtp::vtt::{
    Alignment, Anchor, CueSettings, Line, LineAlignment, Percentage, Position, PositionAlignment,
    Vertical, VttBlock, VttComment, VttCue, VttDescription, VttRegion, VttStyle, VttTimestamp,
    VttTimings, WebVtt,
};

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

use duplicate::duplicate_item;
#[duplicate_item(types; [String]; [str])]
impl ToValue for types {
    fn to_value(&self, span: Span) -> Value {
        Value::string(self, span)
    }
}

impl ToValue for Vec<VttBlock> {
    fn to_value(&self, span: Span) -> Value {
        Value::list(
            self.iter()
                .map(|vtt_block| vtt_block.to_value(span))
                .collect(),
            span,
        )
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
        record! {
            "Comment"=>
            match self {
                VttComment::Side(side) => side.to_value(span),
                VttComment::Below(below) => below.to_value(span),
            }
        }
        .to_value(span)
    }
}

impl ToValue for CueSettings {
    fn to_value(&self, span: Span) -> Value {
        let mut cue_settings = record!();
        if let Some(vertical) = &self.vertical {
            cue_settings.push("Vertical", vertical.to_value(span));
        }
        if let Some(line) = self.line {
            cue_settings.push("Line", line.to_value(span));
        }
        if let Some(position) = self.position {
            cue_settings.push("Position", position.to_value(span));
        }
        if let Some(size) = self.size {
            cue_settings.push("Size", size.to_value(span));
        }
        if let Some(alignment) = self.align {
            cue_settings.push("Alignment", alignment.to_value(span));
        }

        cue_settings.to_value(span)
    }
}

impl ToValue for Vertical {
    fn to_value(&self, span: Span) -> Value {
        match self {
            Vertical::Lr => "Lr".to_value(span),
            Vertical::Rl => "Rl".to_value(span),
        }
    }
}

impl ToValue for Alignment {
    fn to_value(&self, span: Span) -> Value {
        match self {
            Alignment::Center => "Center".to_value(span),
            Alignment::Start => "Start".to_value(span),
            Alignment::End => "End".to_value(span),
            Alignment::Left => "Left".to_value(span),
            Alignment::Right => "Right".to_value(span),
        }
    }
}

impl ToValue for VttCue {
    fn to_value(&self, span: Span) -> Value {
        let mut vtt_cue = record!();
        if let Some(cue_settings) = &self.settings {
            vtt_cue.push("Cue settings", cue_settings.to_value(span));
        }
        if let Some(identifier) = &self.identifier {
            vtt_cue.push("Identifier", identifier.to_value(span))
        }
        vtt_cue.push("Timings", self.timings.to_value(span));

        vtt_cue.push(
            "Payload",
            Value::list(
                self.payload.iter().map(|v| v.to_value(span)).collect(),
                span,
            ),
        );

        vtt_cue.to_value(span)
    }
}

impl ToValue for Line {
    fn to_value(&self, span: Span) -> Value {
        match self {
            Line::LineNumber(val, line_alignment_option) => {
                let mut line_number = record!();
                line_number.push("Number", val.to_value(span));
                if let Some(line_alignment) = line_alignment_option {
                    line_number.push("Alignment", line_alignment.to_value(span));
                }
                line_number.to_value(span)
            }
            Line::Percentage(percentage, line_alignment_option) => {
                let mut line_number = record!();
                line_number.push("Percentage", percentage.to_value(span));
                if let Some(line_alignment) = line_alignment_option {
                    line_number.push("Alignment", line_alignment.to_value(span));
                }
                line_number.to_value(span)
            }
        }
    }
}

impl ToValue for Percentage {
    fn to_value(&self, span: Span) -> Value {
        Value::float(self.value as f64, span)
    }
}

impl ToValue for Position {
    fn to_value(&self, span: Span) -> Value {
        let mut position_record = record!();
        position_record.push("Percentage", self.value.to_value(span));
        if let Some(position_alignment) = self.alignment {
            position_record.push("Alignment", position_alignment.to_value(span))
        }

        position_record.to_value(span)
    }
}
impl ToValue for VttTimings {
    fn to_value(&self, span: Span) -> Value {
        Value::record(
            record! {
                "Start"=>self.start.to_value(span),
                "End"=>self.end.to_value(span),
            },
            span,
        )
    }
}
impl ToValue for VttStyle {
    fn to_value(&self, span: Span) -> Value {
        let mut rec = record!();
        rec.push("Style", self.style.to_value(span));
        Value::record(rec, span)
    }
}

impl ToValue for VttRegion {
    fn to_value(&self, span: Span) -> Value {
        let mut vtt_style = record!();
        if let Some(id) = &self.id {
            vtt_style.push("Id", id.to_value(span))
        }

        if let Some(percentage) = self.width {
            vtt_style.push("Width", percentage.to_value(span))
        }

        if let Some(lines) = self.lines {
            vtt_style.push("Lines", lines.to_value(span))
        }

        if let Some(region_anchor) = self.region_anchor {
            vtt_style.push("Region Anchor", region_anchor.to_value(span));
        }

        if let Some(viewport_anchor) = self.viewport_anchor {
            vtt_style.push("Viewport Anchor", viewport_anchor.to_value(span));
        }

        vtt_style.to_value(span)
    }
}

impl ToValue for Anchor {
    fn to_value(&self, span: Span) -> Value {
        record! {
            "x"=>self.x.to_value(span),
            "y"=>self.y.to_value(span),
        }
        .to_value(span)
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

impl ToValue for LineAlignment {
    fn to_value(&self, span: Span) -> Value {
        match self {
            LineAlignment::Start => "Start".to_value(span),
            LineAlignment::Center => "Center".to_value(span),
            LineAlignment::End => "End".to_value(span),
        }
    }
}

impl ToValue for PositionAlignment {
    fn to_value(&self, span: Span) -> Value {
        match self {
            PositionAlignment::LineLeft => "Line Left".to_value(span),
            PositionAlignment::Center => "Center".to_value(span),
            PositionAlignment::LineRight => "Line Right".to_value(span),
        }
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
