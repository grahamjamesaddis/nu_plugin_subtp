use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand, SimplePluginCommand};
use nu_protocol::{
    Category, ErrorLabel, Example, LabeledError, Signature, Span, Type, Value, record,
};

use super::SubtpPlugin;
use super::common::ToValue;

use subtp::vtt::{
    Alignment, Anchor, CueSettings, Line, LineAlignment, Percentage, Position, PositionAlignment,
    Vertical, VttBlock, VttComment, VttCue, VttDescription, VttRegion, VttStyle, VttTimings,
    WebVtt,
};
pub struct FromVtt;

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
fn signature(name: &str) -> nu_protocol::Signature {
    Signature::build(name)
        .input_output_type(Type::String, Type::Record(Box::new([])))
        .category(Category::Formats)
}
const EX_TXT: &str = "WEBVTT

00:01.000 --> 00:04.000
Never drink liquid nitrogen.

00:05.000 --> 00:09.000
— It will perforate your stomach.
— You could die.

00:10.000 --> 00:14.000
The Organisation for Sample Public Service Announcements accepts no liability for the content of this advertisement, or for the consequences of any actions taken on the basis of the information provided.
";

fn examples(description: &str) -> Vec<Example> {
    let span = Span::test_data();
    let vec = vec![Example {
        description,
        example: EX_TXT,
        result: Some(Value::record(
            record! {
            "Blocks".to_string()=>Value::list(vec![
                Value::record(record!{
                    "Cue".to_string() => Value::record(record!{
                        "Timings".to_string()=>Value::record(record!{
                                "Start".to_string()=>Value::duration(((0*60+1)*1000+000)*1_000_000, span),
                                "End".to_string()=>Value::duration(((0*60+4)*1000+000)*1_000_000, span),
                            }, span),
                        "Payload".to_string()=>Value::list(vec![
                            Value::string("Never drink liquid nitrogen.", span),
                            ], span),
                    }, span),
                }, span),
                Value::record(record!{
                    "Cue".to_string() => Value::record(record!{
                        "Timings".to_string()=>Value::record(record!{
                            "Start".to_string()=>Value::duration(((0*60+5)*1000+000)*1_000_000, span),
                            "End".to_string()=>Value::duration(((0*60+9)*1000+000)*1_000_000, span),
                        }, span),

                        "Payload".to_string()=>Value::list(vec![
                            Value::string("— It will perforate your stomach.", span),
                            Value::string("— You could die.", span),
                            ], span),
                        }, span),
                    }, span),
                Value::record(record!{
                    "Cue".to_string() => Value::record(record!{
                        "Timings".to_string()=>Value::record(record!{
                            "Start".to_string()=>Value::duration(((0*60+10)*1000+000)*1_000_000, span),
                            "End".to_string()=>Value::duration(((0*60+14)*1000+000)*1_000_000, span),
                        }, span),
                        "Payload".to_string()=>Value::list(vec![
                            Value::string("The Organisation for Sample Public Service Announcements accepts no liability for the content of this advertisement, or for the consequences of any actions taken on the basis of the information provided.", span),
                            ], span)
                        }, span),
                    }, span),
                ], span)
            },
            span,
        )),
    }];
    vec
}

pub fn run(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let span = call.head;
    let input_string = input.as_str()?;

    let parse_result: WebVtt =
        subtp::vtt::WebVtt::parse(input_string).map_err(|e| LabeledError {
            labels: Box::new(vec![ErrorLabel {
                text: "Error parsing vtt".into(),
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

impl ToValue for VttDescription {
    fn to_value(&self, span: Span) -> Value {
        match self {
            VttDescription::Side(side) => side.to_value(span),
            VttDescription::Below(below) => below.to_value(span),
        }
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
            VttBlock::Comment(vtt_comment) => {
                record! {"Commmet"=> vtt_comment.to_value(span)}.to_value(span)
            }
            VttBlock::Que(vtt_cue) => record! {"Cue"=>vtt_cue.to_value(span)}.to_value(span),
            VttBlock::Style(vtt_style) => {
                record! {"Style"=>vtt_style.to_value(span)}.to_value(span)
            }

            VttBlock::Region(vtt_region) => {
                record! {"Region"=>vtt_region.to_value(span)}.to_value(span)
            }
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
        self.value.to_value(span)
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
        record! {
            "Start"=>self.start.to_value(span),
            "End"=>self.end.to_value(span),
        }
        .to_value(span)
    }
}
impl ToValue for VttStyle {
    fn to_value(&self, span: Span) -> Value {
        record! {
            "Style"=>self.style.to_value(span),
        }
        .to_value(span)
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
