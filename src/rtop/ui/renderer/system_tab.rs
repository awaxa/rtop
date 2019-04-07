use crate::rtop::app::App;
use crate::rtop::ui::panels::system::*;

use tui::Terminal;
use tui::backend::MouseBackend;
use tui::widgets::Widget;
use tui::layout::{Direction, Group, Rect, Size};

pub fn render_system_tab(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
      Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, area, |t, chunks| {
            render_charts(t, app, &chunks[0]);
            render_bottom_thrid(t, app, &chunks[1]);
        });
}

pub fn render_charts(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    let sizes = vec![Size::Percent(35), Size::Percent(65)];
    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&sizes)
        .render(t, area, |t, chunks| {
                render_sidebar(t, app, &chunks[0]);
                cpu_usage_history_panel(t, app, &chunks[1]);
        });
}

fn render_sidebar(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, area, |t, chunks| {
            processes_panel(t, app, &chunks[0]);
            disk_usage_panel(t, app, &chunks[1]);
        });
}

fn render_bottom_thrid(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, area, |t, chunks| {
            render_bottom_left_corner(t, app, &chunks[0]);
            mem_and_swap_history_panel(t, app, &chunks[1]);
            //panels::text::render_text(t, &chunks[2]);
        });
}

fn render_bottom_left_corner(t: &mut Terminal<MouseBackend>, app: &App, area: &Rect) {
    if cfg!(feature = "battery-monitor") {    
        Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(25), Size::Percent(75)])
        .render(t, area, |t, chunks| {
            #[cfg(feature = "battery-monitor")]
            battery_panel(t, app, &chunks[0]);
            network_info_panel(t, app, &chunks[1]);
        });
    } else {    
        network_info_panel(t, app, area);
    }

}