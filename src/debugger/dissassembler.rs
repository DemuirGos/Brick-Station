use crate::hardware::interfaces::Device;
use crate::hardware::bus::*;
use crate::hardware::cpu::*;
use crate::hardware::ram::*;

use std::{io, io::Error};
use tui::symbols::DOT;
use tui::{backend::CrosstermBackend, Terminal};
use tui::{
    style::*,
    text::*,
    widgets::*,
    layout::*,
};
use crossterm::{
    event::*,
    execute,
    terminal::*,
};
pub struct State {
    pub ram: Ram,
    pub bus: Bus,
    pub cpu: Cpu,
}

pub struct App {
    pub index: usize,
    pub state: State,
}

impl State {
    pub fn start() -> Result<(), Error> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = match Terminal::new(backend) {
            Ok(it) => it,
            Err(err) => return Err(err),
        };
        Ok(())
    }


    pub fn memory_viewer(ram: &Ram,  app: &App) {
        let build_page_table = |ram: &Ram, page: u16| {
            let header_cells = (0..16)
                .map(|i| format!("{:02X}", i))
                .map(|s| Cell::from(s).style(Style::default().fg(Color::Red)));
            let header_row = Row::new(header_cells)
                .style(Style::default().bg(Color::Blue))
                .height(1)
                .bottom_margin(1);
            
            let mut rows = vec![];
            for i in 0..16 {
                let mut row_data = vec![];
                for j in 0..16 {
                    let address = (page << 8) + (i * 16 + j);
                    let value = ram.read(address);
                    let cell = Cell::from(format!("{:02X}", value));
                    row_data.push(cell);
                }
                let row = Row::new(row_data);
                rows.push(row);
            }

            let page_title = format!("Page {:02X}", page);
            let table = Table::new(rows)
                .header(header_row)
                .block(Block::default().borders(Borders::ALL).title(page_title))
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">> ")
                .widths(&[
                    Constraint::Percentage(50),
                    Constraint::Length(30),
                    Constraint::Min(10),
                ]);
            table
        };

        let titles_cells = Spans::from((0..16)
            .map(|i| format!("{:02X}", i))
            .map(|s| Span::styled(s, Style::default().fg(Color::Red)))
            .collect::<Vec<_>>());
        

        let tabs = Tabs::new(vec![titles_cells])
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(app.index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black),
            );

    }

    pub fn dissassemble(bus: &Bus, range_start: u16, range_end: u16) {
        let mut string_builder = vec![];
        string_builder.push("test");
        ()
    }
}

