pub mod vtt_val {
    use nu_protocol::{
        Category, ErrorLabel, Example, LabeledError, Signature, Span, Type, Value, record,
    };
    use std::{i64, time::Duration};
    use subtp::vtt::{
        Alignment, Anchor, Line, LineAlignment, Percentage, Position, PositionAlignment, Vertical,
        VttBlock, VttComment, VttCue, VttDescription, VttRegion, VttStyle, VttTimestamp,
        VttTimings, WebVtt,
    };
    pub struct NuValue {
        pub value: Value,
    }

    impl NuValue {
        pub fn from_web_vtt(web_vtt: &WebVtt, span: Span) -> Self {
            let mut vtt_rec = record!();
            if let Some(description) = &web_vtt.header.description {
                vtt_rec.push(
                    "Description",
                    NuValue::from_vtt_description(&description, span).value,
                );
            }
            vtt_rec.push(
                "Blocks",
                NuValue::from_vtt_blocks(&web_vtt.blocks, span).value,
            );

            NuValue {
                value: Value::record(vtt_rec, span),
            }
        }

        fn from_vtt_timestamp(vtt_timestamp: VttTimestamp, span: Span) -> Self {
            let start: Duration = vtt_timestamp.into();
            let seconds: i64 = start.as_secs() as i64 * 1_000_000_000;
            NuValue {
                value: Value::duration(seconds, span),
            }
        }
        fn from_vtt_timings(vtt_timings: VttTimings, span: Span) -> Self {
            NuValue {
                value: Value::record(
                    record! {
                        "Start".to_string()=>NuValue::from_vtt_timestamp(vtt_timings.start, span).value,
                        "End".to_string()=>NuValue::from_vtt_timestamp(vtt_timings.end, span).value,
                    },
                    span,
                ),
            }
        }
        fn from_vtt_comment(vtt_comment: &VttComment, span: Span) -> Self {
            NuValue {
                value: Value::record(
                    record! {
                        "Comment".to_string()=>
                        match vtt_comment {
                            VttComment::Side(side) => Value::string(side, span),
                            VttComment::Below(below) => Value::string(below, span),
                        }
                    },
                    span,
                ),
            }
        }
        fn from_vtt_anchor(anchor: Anchor, span: Span) -> Self {
            NuValue {
                value: Value::record(
                    record! {
                        "x".to_string()=>Value::float(anchor.x.value.into(), span),
                        "y".to_string()=>Value::float(anchor.y.value.into(), span),
                    },
                    span,
                ),
            }
        }
        fn from_vtt_line_alignment(line_alignment: &LineAlignment, span: Span) -> Self {
            NuValue {
                value: match line_alignment {
                    LineAlignment::Start => Value::string("Start".to_string(), span),
                    LineAlignment::Center => Value::string("Center".to_string(), span),
                    LineAlignment::End => Value::string("End".to_string(), span),
                },
            }
        }
        fn from_vtt_position_alignment(position_alignment: &PositionAlignment, span: Span) -> Self {
            NuValue {
                value: match position_alignment {
                    PositionAlignment::LineLeft => Value::string("Line Left".to_string(), span),
                    PositionAlignment::Center => Value::string("Center".to_string(), span),
                    PositionAlignment::LineRight => Value::string("Line Right".to_string(), span),
                },
            }
        }
        fn from_vtt_position(position: Position, span: Span) -> NuValue {
            let mut position_record = record!();
            position_record.push(
                "Position perccntage".to_string(),
                Value::float(position.value.value.into(), span),
            );
            if let Some(position_alignment) = position.alignment {
                position_record.push(
                    "Position Alignment".to_string(),
                    NuValue::from_vtt_position_alignment(&position_alignment, span).value,
                )
            }

            NuValue {
                value: Value::record(position_record, span),
            }
        }
        fn from_line_number(
            line_number_value: &i32,
            line_alignment_option: &Option<LineAlignment>,
            span: Span,
        ) -> Self {
            let mut line_number = record!();
            line_number.push(
                "Number".to_string(),
                Value::int(line_number_value.clone() as i64, span),
            );
            if let Some(line_alignment) = line_alignment_option {
                line_number.push(
                    "Alignment".to_string(),
                    NuValue::from_vtt_line_alignment(&line_alignment, span).value,
                );
            }
            NuValue {
                value: Value::record(line_number, span),
            }
        }
        fn from_line_percentage(
            percentage: &Percentage,
            line_alignment_option: &Option<LineAlignment>,
            span: Span,
        ) -> Self {
            let mut line_number = record!();
            line_number.push(
                "Percentage".to_string(),
                Value::float(percentage.value.clone() as f64, span),
            );
            if let Some(line_alignment) = line_alignment_option {
                line_number.push(
                    "Alignment".to_string(),
                    NuValue::from_vtt_line_alignment(&line_alignment, span).value,
                );
            }
            NuValue {
                value: Value::record(line_number, span),
            }
        }
        fn from_vtt_line(line: &Line, span: Span) -> Self {
            match line {
                Line::LineNumber(val, line_alignment_option) => {
                    NuValue::from_line_number(val, line_alignment_option, span)
                }
                Line::Percentage(percentage, line_alignment_option) => {
                    NuValue::from_line_percentage(percentage, line_alignment_option, span)
                }
            }
        }
        fn from_vtt_cue(vtt_cue: &VttCue, span: Span) -> Self {
            let mut rec = record!();
            if let Some(cue_setting) = &vtt_cue.settings {
                if let Some(vertical) = &cue_setting.vertical {
                    rec.push(
                        "Vertical",
                        match vertical {
                            Vertical::Lr => Value::string("Lr".to_string(), span),
                            Vertical::Rl => Value::string("Rl".to_string(), span),
                        },
                    );
                }
                if let Some(line) = cue_setting.line {
                    rec.push(
                        "Line".to_string(),
                        NuValue::from_vtt_line(&line, span).value,
                    );
                }
                if let Some(position) = cue_setting.position {
                    rec.push(
                        "Position".to_string(),
                        NuValue::from_vtt_position(position, span).value,
                    );
                }
                if let Some(size) = cue_setting.size {
                    rec.push("Size".to_string(), Value::float(size.value as f64, span));
                }
                if let Some(align) = cue_setting.align {
                    rec.push(
                        "Align".to_string(),
                        match align {
                            Alignment::Center => Value::string("Center", span),
                            Alignment::Start => Value::string("Start", span),
                            Alignment::End => Value::string("End", span),
                            Alignment::Left => Value::string("Left", span),
                            Alignment::Right => Value::string("Right", span),
                        },
                    );
                }

                // todo!("implement settings");
            }
            if let Some(identifier) = &vtt_cue.identifier {
                rec.push("Identifier", Value::string(identifier.clone(), span))
            }
            rec.push(
                "Timings",
                NuValue::from_vtt_timings(vtt_cue.timings, span).value,
            );

            let payload: Vec<Value> = vtt_cue
                .payload
                .iter()
                .map(|v| Value::string(v, span))
                .collect();

            rec.push("Payload", Value::list(payload, span));
            NuValue {
                value: Value::record(rec, span),
            }
        }
        fn from_vtt_style(vtt_style: &VttStyle, span: Span) -> Self {
            let mut rec = record!();
            rec.push(
                "Style",
                nu_protocol::Value::string(vtt_style.style.clone(), span),
            );
            NuValue {
                value: Value::record(rec, span),
            }
        }
        fn from_vtt_region(vtt_region: &VttRegion, span: Span) -> Self {
            let mut rec = record!();
            if let Some(id) = &vtt_region.id {
                rec.push("Id", Value::string(id.clone(), span))
            }

            if let Some(percentage) = vtt_region.width {
                rec.push("Width", Value::float(percentage.value.into(), span))
            }

            if let Some(lines) = vtt_region.lines {
                rec.push("Lines", Value::int(lines.into(), span))
            }

            if let Some(region_anchor) = vtt_region.region_anchor {
                rec.push(
                    "Region Anchor",
                    NuValue::from_vtt_anchor(region_anchor, span).value,
                );
            }

            if let Some(viewport_anchor) = vtt_region.viewport_anchor {
                rec.push(
                    "Viewport Anchor",
                    NuValue::from_vtt_anchor(viewport_anchor, span).value,
                );
            }
            NuValue {
                value: Value::record(rec, span),
            }
        }

        fn from_vtt_block(vtt_block: &VttBlock, span: Span) -> Self {
            match vtt_block {
                VttBlock::Comment(vtt_comment) => NuValue::from_vtt_comment(&vtt_comment, span),
                VttBlock::Que(vtt_cue) => NuValue::from_vtt_cue(&vtt_cue, span),
                VttBlock::Style(vtt_style) => NuValue::from_vtt_style(&vtt_style, span),
                VttBlock::Region(vtt_region) => NuValue::from_vtt_region(&vtt_region, span),
            }
        }
        fn from_vtt_blocks(vtt_blocks: &Vec<VttBlock>, span: Span) -> Self {
            let mut subtitles: Vec<Value> = vec![];

            for vtt_block in vtt_blocks {
                subtitles.push(NuValue::from_vtt_block(vtt_block, span).value);
            }

            NuValue {
                value: Value::list(subtitles, span),
            }
        }
        fn from_vtt_description(description: &VttDescription, span: Span) -> Self {
            NuValue {
                value: match description {
                    VttDescription::Side(side) => Value::string(side, span),
                    VttDescription::Below(below) => Value::string(below, span),
                },
            }
        }
    }
    fn convert_vtt_cue_to_value(vtt_cue: &VttCue, span: Span) -> Vec<(String, Value)> {
        let cue_values: Vec<(String, Value)> = Vec::new();

        if let Some(settings) = &vtt_cue.settings {}

        cue_values
    }
}
