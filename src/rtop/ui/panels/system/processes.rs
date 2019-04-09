
use crate::rtop::app::App;
use crate::rtop::ui::panels::utils;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::{Block, Borders, Row, Table, Widget};
use tui::layout::Rect;
use tui::style::{Color, Style};


pub fn processes_panel(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    let mut processes_by_cpu = app.datastreams.process_info.processes.clone();
    processes_by_cpu.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    let (selected_proc, processes_to_display) = utils::scrolling(area, app.selected_proc, &processes_by_cpu[..]);

    let selected_style = Style::default().fg(Color::White).bg(Color::Green);
    let default_style = Style::default().fg(Color::Cyan);
    let proc_table = Table::new(
        ["PID", "Command", "%CPU▲", "Mem (KB)"].into_iter(),
        processes_to_display.iter().enumerate().map(|(i, s)| {
            let style = if i == selected_proc {
                &selected_style
            } else {
                &default_style
            };
            Row::StyledData(vec![s.0.to_string(), s.1.to_string(), format!("{:.2}", s.2), s.3.to_string()].into_iter(), style)
        }),
    ).block(Block::default().title("Processes").borders(Borders::ALL))
     .header_style(Style::default().fg(Color::Yellow))
     .widths(&[15, 15, 10, 10])
     .render(t, &area);
}