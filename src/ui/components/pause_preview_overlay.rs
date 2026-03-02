use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::agent::PauseContext;

pub struct PausePreviewOverlay;

impl PausePreviewOverlay {
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        agent_name: &str,
        pause_context: &PauseContext,
        resume_key: &str,
    ) {
        frame.render_widget(Clear, area);

        let popup_width = area.width.min(72);
        let label = "Resume error: ";
        // Estimate available text width inside bordered popup.
        let available_width = popup_width.saturating_sub(4).max(8) as usize;
        let error_text_opt = pause_context
            .last_resume_error
            .as_ref()
            .map(|err| normalize_error_text(err));
        let wrapped_error_lines = error_text_opt.as_ref().map(|error_text| {
            if label.chars().count() + error_text.chars().count() <= available_width {
                vec![error_text.clone()]
            } else {
                wrap_text_to_width(error_text, available_width)
            }
        });

        let error_lines = if let Some(lines) = &wrapped_error_lines {
            if lines.is_empty() {
                2
            } else if lines.len() == 1
                && label.chars().count() + lines[0].chars().count() <= available_width
            {
                2 // spacer + inline label+text
            } else {
                2 + lines.len() // spacer + label + wrapped lines
            }
        } else {
            0
        };
        let base_lines = 10usize;
        // +2 for top/bottom border padding in Block.
        let needed_height = (base_lines + error_lines + 2) as u16;
        let popup_height = area.height.min(needed_height).max(11);
        let popup = centered_rect(popup_width, popup_height, area);

        frame.render_widget(Clear, popup);

        let mut lines = vec![
            Line::from(Span::styled(
                format!("Agent '{}' is paused", agent_name),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(format!("Mode: {}", pause_context.mode.label())),
            Line::from(""),
            Line::from("Checkout command:"),
            Line::from(Span::styled(
                pause_context.checkout_command.clone(),
                Style::default().fg(Color::Yellow),
            )),
            Line::from(""),
            Line::from(pause_context.instruction_message.clone()),
            Line::from(""),
            Line::from(Span::styled(
                format!("Press {} to resume", resume_key),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
        ];

        if let Some(error_lines) = wrapped_error_lines {
            lines.push(Line::from(""));
            if error_lines.len() == 1
                && label.chars().count() + error_lines[0].chars().count() <= available_width
            {
                lines.push(Line::from(vec![
                    Span::styled(
                        label,
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(error_lines[0].clone(), Style::default().fg(Color::Red)),
                ]));
            } else {
                lines.push(Line::from(Span::styled(
                    label,
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )));
                for chunk in error_lines {
                    lines.push(Line::from(Span::styled(
                        chunk,
                        Style::default().fg(Color::Red),
                    )));
                }
            }
        }

        let paragraph = Paragraph::new(lines).alignment(Alignment::Left).block(
            Block::default()
                .title(" Checkout Pause ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        );

        frame.render_widget(paragraph, popup);
    }
}

fn normalize_error_text(err: &str) -> String {
    let normalized = err
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if normalized.is_empty() {
        "(no details)".to_string()
    } else {
        normalized
    }
}

fn wrap_text_to_width(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }

    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();

    for token in text.split_whitespace() {
        let token_len = token.chars().count();
        if token_len > width {
            if !current.is_empty() {
                lines.push(std::mem::take(&mut current));
            }
            lines.extend(split_long_token(token, width));
            continue;
        }

        let candidate_len = if current.is_empty() {
            token_len
        } else {
            current.chars().count() + 1 + token_len
        };

        if candidate_len <= width {
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(token);
        } else {
            lines.push(std::mem::take(&mut current));
            current.push_str(token);
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        vec![String::new()]
    } else {
        lines
    }
}

fn split_long_token(token: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![token.to_string()];
    }
    let mut chunks = Vec::new();
    let mut current = String::new();
    for ch in token.chars() {
        if current.chars().count() >= width {
            chunks.push(std::mem::take(&mut current));
        }
        current.push(ch);
    }
    if !current.is_empty() {
        chunks.push(current);
    }
    chunks
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((area.height.saturating_sub(height)) / 2),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((area.width.saturating_sub(width)) / 2),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(vertical[1])[1]
}
