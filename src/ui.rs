use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, List, Paragraph},
};

struct TextColors {
    foreground: Color,
    background: Color,
}

pub fn render(app: &App, frame: &mut Frame) {
    let constraints = [Constraint::Max(2), Constraint::Fill(1)];

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(constraints)
        .split(frame.area());

    let main_title = Line::from(" Quick Search ".bold());
    let instructions = get_instructions();

    let block = Block::bordered()
        .title(main_title)
        .title_bottom(instructions)
        .border_set(border::THICK);

    frame.render_widget(block, frame.area());

    render_search(app, frame, layout[0]);
    render_options(app, frame, layout[1]);
}

fn render_search(app: &App, frame: &mut Frame, area: Rect) {
    let content = format!("Search content: {}█", app.search_content);
    let search_paragraph = Paragraph::new(content);
    frame.render_widget(search_paragraph, area);
}

fn render_options(app: &App, frame: &mut Frame, area: Rect) {
    let mut search_items = Vec::<Line>::new();
    let options_block = Block::default().title("");

    app.pages.iter().enumerate().for_each(|(index, page)| {
        let colors = get_colors(app.selected, index);

        search_items.push(Line::from(Span::styled(
            &page.name,
            Style::default().fg(colors.foreground).bg(colors.background),
        )));
    });

    let options_list = List::new(search_items).block(options_block);
    frame.render_widget(options_list, area);
}

fn get_colors(selected: usize, index: usize) -> TextColors {
    if index == selected {
        TextColors {
            foreground: Color::Black,
            background: Color::LightYellow,
        }
    } else {
        TextColors {
            foreground: Color::LightYellow,
            background: Color::Reset,
        }
    }
}

fn get_instructions<'a>() -> Line<'a> {
    let instructions = Line::from(vec![
        " Previous ".into(),
        "<Up>".blue().bold(),
        " Next ".into(),
        "<Down>".blue().bold(),
        " Quit ".into(),
        "<Esc> ".blue().bold(),
    ]);

    Line::from(instructions)
}
