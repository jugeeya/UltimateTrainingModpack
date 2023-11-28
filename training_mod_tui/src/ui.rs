use crate::{App, AppPage, SliderState, NX_SUBMENU_COLUMNS};
use ratatui::{layout::Rect, prelude::*, widgets::*, Frame};

#[allow(unused_variables)]
pub fn render_ui(frame: &mut Frame, app: &mut App) {
    // Set up Layout
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(frame.size());

    // Define Areas
    // tab_area: list across the top
    // menu_area: menu entries
    let tab_area = layout[0];
    let menu_area = layout[1];
    let help_area = layout[2];

    render_tabs(frame, app, tab_area);
    match app.page {
        AppPage::SUBMENU => render_submenu_page(frame, app, menu_area),
        AppPage::TOGGLE => render_toggle_page(frame, app, menu_area),
        AppPage::SLIDER => render_slider_page(frame, app, menu_area),
        AppPage::CONFIRMATION => {
            frame.render_widget(Paragraph::new("Confirmation!"), menu_area);
        }
        AppPage::CLOSE => {}
    }
    render_help_text(frame, app, help_area);
}

fn render_submenu_page(frame: &mut Frame, app: &mut App, area: Rect) {
    let selected_tab = app.selected_tab();
    let submenus = &mut selected_tab.submenus;
    let tab_title = selected_tab.title;
    // Convert the currently selected tab's grid of Option<SubMenu>'s
    // into an Iter<Row<Cell>> so that we can pass it into Table::new()
    let rows = submenus
        .items
        .iter()
        .map(|row| {
            row.iter()
                .filter(|submenu| submenu.is_some())
                .map(|submenu| {
                    let s = submenu.clone().unwrap();
                    Cell::from(s.title.to_string())
                })
        })
        .map(|row| Row::new(row));

    let table = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title(tab_title))
        .cell_highlight_style(Style::default().bg(Color::Gray))
        .widths(&[Constraint::Ratio(1, NX_SUBMENU_COLUMNS as u32); NX_SUBMENU_COLUMNS]);

    frame.render_stateful_widget(table, area, &mut submenus.state);
}

fn render_toggle_page(frame: &mut Frame, app: &mut App, area: Rect) {
    let toggles = &mut app.selected_submenu().toggles;
    // Convert the currently selected submenu's grid of Option<Toggle>'s
    // into an Inter<Row<Cell>> so that we can pass it into Table::new()
    let rows = toggles
        .items
        .iter()
        .map(|row| {
            row.iter().filter(|x| x.is_some()).map(|toggle| {
                // Display both the title and the value
                // Don't need to clone() here because toggle is Copy
                let t = toggle.unwrap();
                Cell::from(t.title.to_string() + "  -  " + &t.value.to_string())
            })
        })
        .map(|row| Row::new(row));

    let table = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Submenus:"))
        .cell_highlight_style(Style::default().bg(Color::Gray))
        .widths(&[Constraint::Ratio(1, NX_SUBMENU_COLUMNS as u32); NX_SUBMENU_COLUMNS]);

    frame.render_stateful_widget(table, area, &mut toggles.state);
}

#[allow(dead_code, unused_variables)]
fn render_slider_page(frame: &mut Frame, app: &mut App, area: Rect) {
    let submenu = app.selected_submenu();
    let slider = submenu.slider.as_mut().expect("No slider selected!");

    // Double ended sliders are rendered as four distinct LineGauge widgets
    // 1. Minimum to Lower value
    // 2. Lower value to Upper value
    // 3. Upper value to maximum
    // 4. Maximum
    //
    // The LineGauge labels are left-aligned, so those four gauges have the following labels:
    // 1. Minimum
    // 2. Lower value
    // 3. Upper value
    // 4. Maximum
    //
    // Depending on the state, we style each gauge differently.
    let lbl_ratio = 0.95;
    let constraints = [
        Constraint::Ratio(
            (lbl_ratio * (slider.lower - slider.min) as f32) as u32,
            slider.max - slider.min,
        ),
        Constraint::Ratio(
            (lbl_ratio * (slider.upper - slider.lower) as f32) as u32,
            slider.max - slider.min,
        ),
        Constraint::Ratio(
            (lbl_ratio * (slider.max - slider.upper) as f32) as u32,
            slider.max - slider.min,
        ),
        Constraint::Length(3), // For upper limit label
    ];
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    let mut modified_line_set = symbols::line::NORMAL;
    modified_line_set.horizontal = " ";
    let hover_style = Style::default().fg(Color::Red);
    let selected_style = Style::default().fg(Color::Green);
    let deselected_style = Style::default().fg(Color::Yellow).bg(Color::Black);

    let base_gauge = LineGauge::default()
        .ratio(1.0)
        .style(Style::default().fg(Color::White))
        .gauge_style(Style::default().fg(Color::White).bg(Color::Black))
        // .block(Block::default().borders(Borders::ALL))
        .line_set(modified_line_set);

    // Min ---- Lower
    let gauge_min_to_lower = base_gauge.clone().label(slider.min.to_string());
    frame.render_widget(gauge_min_to_lower, layout[0]);

    // Lower ----- Upper
    let gauge_lower_to_upper = base_gauge
        .clone()
        .set_style(match slider.state {
            SliderState::LowerHover => hover_style,
            SliderState::LowerSelected => selected_style,
            _ => deselected_style,
        })
        .label(slider.lower.to_string())
        .line_set(symbols::line::NORMAL);
    frame.render_widget(gauge_lower_to_upper, layout[1]);

    // Upper ----- Max
    let gauge_upper_to_max = base_gauge
        .clone()
        .set_style(match slider.state {
            SliderState::UpperHover => hover_style,
            SliderState::UpperSelected => selected_style,
            _ => deselected_style,
        })
        .label(slider.upper.to_string());
    frame.render_widget(gauge_upper_to_max, layout[2]);

    // Max
    let mut gauge_max = base_gauge
        .clone()
        .line_set(modified_line_set)
        .label(slider.max.to_string());
    // This is displayed on top of the gauge_upper_to_max slider
    // So if the `upper` is close enough to the `max`
    // we need to change the gauge_max slider styling to match
    if (slider.upper as f32 / slider.max as f32) > lbl_ratio {
        gauge_max = gauge_max.set_style(match slider.state {
            SliderState::UpperHover => hover_style,
            SliderState::UpperSelected => selected_style,
            _ => Style::default(),
        });
    }
    frame.render_widget(gauge_max, layout[3]);
}

fn render_tabs(frame: &mut Frame, app: &mut App, area: Rect) {
    let titles = vec![
        "...",
        app.tabs
            .get_before_selected()
            .expect("No tab selected!")
            .title,
        app.tabs.get_selected().expect("No tab selected!").title,
        app.tabs
            .get_after_selected()
            .expect("No tab selected!")
            .title,
        "...",
    ];
    let tabs = Tabs::new(titles);
    frame.render_widget(tabs, area);
}

fn render_help_text(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_widget(Paragraph::new(app.selected_submenu().help_text), area);
}
