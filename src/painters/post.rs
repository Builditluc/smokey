use crate::application::App;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
    Terminal,
};

pub fn draw_post<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
    terminal
        .draw(|frame| {
            let test = &mut app.test;
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
                .vertical_margin(app.margin)
                .horizontal_margin(app.margin)
                .split(frame.size());

            let final_wpm = format!("{}", test.hoarder.final_wpm.round());
            let final_acc = format!("{}", test.hoarder.final_acc.round());

            let up_txt = vec![
                Spans::from(vec![
                    Span::raw("wpm: "),
                    Span::styled(final_wpm, Style::default().fg(Color::Blue)),
                ]),
                Spans::from(vec![
                    Span::raw("acc: "),
                    Span::styled(final_acc, Style::default().fg(Color::Green)),
                ]),
                Spans::from(vec![
                    Span::raw("mis: "),
                    Span::styled(
                        format!("{}", test.mistakes),
                        Style::default().fg(Color::Red),
                    ),
                ]),
            ];

            let block =
                Paragraph::new(up_txt).block(Block::default().title("WPM").borders(Borders::ALL));

            frame.render_widget(block, chunks[0]);

            let secs: f64 = test.hoarder.seconds as f64;
            let length: f64 = test.hoarder.wpms.len() as f64;
            let max_wpm: f64 = test.hoarder.get_max_wpm();

            let data = test
                .hoarder
                .wpms
                .iter()
                .enumerate()
                .map(|(i, val)| (((i + 1) as f64 * secs as f64), *val))
                .collect::<Vec<(f64, f64)>>();

            let wpm_datasets = vec![Dataset::default()
                .name("wpm")
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Blue))
                .graph_type(GraphType::Line)
                .data(&data)];

            let x_labels: Vec<Span> = data
                .iter()
                .map(|&(i, _)| Span::styled(format!("{}", i), Style::default().fg(Color::Blue)))
                .collect();

            let margin: f64 = 20.;
            let y_upper_bound: f64 = max_wpm.div_euclid(10.) * 10. + margin;

            let y_labels: Vec<Span> = (0..=y_upper_bound.div_euclid(10.) as i32)
                .map(|i| Span::styled(format!("{}", i * 10), Style::default().fg(Color::Blue)))
                .collect();

            let chart = Chart::new(wpm_datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "wpm chart",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("time (s)")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([secs, length * secs])
                        .labels(x_labels),
                )
                .y_axis(
                    Axis::default()
                        .title("wpm")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0., y_upper_bound])
                        .labels(y_labels),
                );

            frame.render_widget(chart, chunks[1]);
        })
        .expect("drawing post went fine");
}
