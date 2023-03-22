use crate::hardware::interfaces::Device;
use crate::hardware::bus::*;
use crate::hardware::cpu::*;
use crate::hardware::ram::*;

use std::{io, io::Error};
use tui::Frame;
use tui::backend::Backend;
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
    pub program: Vec<String>,
}

pub struct App {
    pub memory_page_index: u16,
    pub program_counter: u16,
    pub inner_machine_state: State,
}

impl State {
    pub fn start() -> Result<(), Error> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = match Terminal::new(backend) {
            Ok(it) => it,
            Err(err) => return Err(err),
        };

        let mut app = App {
            memory_page_index: 0,
            program_counter: 0,
            inner_machine_state: State {
                ram: Ram::new(),
                bus: Bus::new(),
                cpu: Cpu::new(),
                program: vec![],
            },
        };

        terminal.clear()?;
        terminal.hide_cursor()?;

        loop {
            terminal.draw(|f| {
                State::build_view(f, &app);
            })?;

            if let Ok(Event::Key(key)) = read() {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('w') => app.memory_page_index = app.memory_page_index.wrapping_add(1),
                    KeyCode::Char('s') => app.memory_page_index = app.memory_page_index.wrapping_sub(1),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn build_view<B: Backend>(f: &mut Frame<B>, app: &App)  {
        let size = Rect::new(0, 0, f.size().width, f.size().height);
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(5)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(size);

        let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
        f.render_widget(block, size);

        State::memory_viewer(f, app);
        State::program_viewer(f, app);
        State::processor_viewer(f, app);
    }

    pub fn memory_viewer<B: Backend>(f: &mut Frame<B>, app: &App)  {
        let size = Rect::new(0, 0, (f.size().width as f32 * 0.70) as u16, f.size().height);
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(5)
            .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref())
            .split(size);

        let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
        f.render_widget(block, size);

        let build_page_table = |ram: &Ram, page: u16| {
            let header_cells = (0..=16)
                .map(|i| format!("{:02X}", i))
                .map(|s| Cell::from(s).style(Style::default().fg(Color::Red)));
            let header_row = Row::new(header_cells)
                .style(Style::default().bg(Color::Blue))
                .height(1)
                .bottom_margin(1);
            
            let mut rows = vec![];
            for i in 0..=16 {
                let mut row_data = vec![
                    Cell::from(format!("{:02X}", i + 1)).style(Style::default().fg(Color::Black).bg(Color::Blue)),
                ];
                for j in 0..16 {
                    let address = (page << 8) + (i * 16 + j);
                    let value = ram.read(address);
                    let cell = Cell::from(format!("{:04X}", value)).style(Style::default().fg(Color::Black));
                    row_data.push(cell);
                }
                dbg!(row_data.len());
                let row = Row::new(row_data).height(2);
                rows.push(row);
            }


            let page_title = format!("Page {:02X}", page);
            let table = Table::new(rows)
                .header(header_row)
                .block(Block::default().borders(Borders::ALL).title(page_title))
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">> ")
                .widths(&[
                    Constraint::Percentage(5), Constraint::Percentage(5), Constraint::Percentage(5), Constraint::Percentage(5),
                    Constraint::Percentage(5), Constraint::Percentage(5), Constraint::Percentage(5), Constraint::Percentage(5),
                    Constraint::Percentage(5), Constraint::Percentage(5), Constraint::Percentage(5), Constraint::Percentage(5),
                    Constraint::Percentage(5), Constraint::Percentage(5), Constraint::Percentage(5), Constraint::Percentage(5),
                    Constraint::Percentage(5), Constraint::Percentage(5), Constraint::Percentage(5), Constraint::Percentage(5),
                ]);
            table
        };

        let page_cells = (0..16)
            .map(|i| format!("[ {:02X} ]", i))
            .map(|s| Span::styled(s, Style::default().fg(Color::LightBlue)))
            .map(|s| Row::new([s]).height(2))
            .collect::<Vec<_>>();

        let page_selection_table = Table::new(page_cells)
            .block(Block::default().borders(Borders::ALL).title("Pages"))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">> ")
            .widths(&[Constraint::Percentage(100)]);
        f.render_widget(page_selection_table, chunks[0]);
        
        let table = build_page_table(&app.inner_machine_state.ram, app.memory_page_index);
        f.render_widget(table, chunks[1]);

    }

    pub fn program_viewer<B: Backend>(f: &mut Frame<B>, app: &App)  {
        let size = Rect::new((f.size().width as f32 * 0.70) as u16, (f.size().height as f32 * 0.31) as u16, (f.size().width as f32 * 0.30) as u16, (f.size().height as f32 * 0.69) as u16);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(size);

        let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
        f.render_widget(block, size);

        
        let build_program_list = |program: Vec<String>| {
            let list_elements = program
                .into_iter()
                .map(|s| ListItem::new(Spans::from(vec![Span::raw(s)])))
                .collect::<Vec<ListItem>>();
            let list = List::new(list_elements)
                .block(Block::default().borders(Borders::ALL).title("Program"))
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">> ");
            list
        };

        let program = app.inner_machine_state.program.clone();
        let list = build_program_list(program);
        f.render_widget(list, chunks[1]);
    }

    pub fn processor_viewer<B: Backend>(f: &mut Frame<B>, app: &App)  {
        let size = Rect::new((f.size().width as f32 * 0.70) as u16, 0, (f.size().width as f32 * 0.30) as u16, (f.size().height as f32 * 0.60) as u16);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref())
            .split(size);

        let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
        f.render_widget(block, size);

        
        let build_registers_list = |cpu: &Cpu| {
            let list_elements = vec![
                ListItem::new(Spans::from(vec![Span::raw(format!("A: {:02X}", cpu.registers.a))])), 
                ListItem::new(Spans::from(vec![Span::raw(format!("X: {:02X}", cpu.registers.x))])), 
                ListItem::new(Spans::from(vec![Span::raw(format!("Y: {:02X}", cpu.registers.y))])), 
                ListItem::new(Spans::from(vec![Span::raw(format!("PC: {:04X}", cpu.registers.pc))])), 
                ListItem::new(Spans::from(vec![Span::raw(format!("SP: {:02X}", cpu.registers.sp))])), 
                ListItem::new(Spans::from(vec![Span::raw(format!("P: {:02X}", cpu.registers.status))])), 
            ];
            let list = List::new(list_elements)
                .block(Block::default().borders(Borders::ALL).title("Registers"))
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">> ");
            list
        };

        let build_status_view = |cpu: &Cpu| {
            let list_elements = vec![
                ListItem::new(Spans::from(vec![Span::raw(format!("N: {}", cpu.registers.get_flag(crate::hardware::registers::Flag::N)))])), 
                ListItem::new(Spans::from(vec![Span::raw(format!("V: {}", cpu.registers.get_flag(crate::hardware::registers::Flag::O)))])), 
                ListItem::new(Spans::from(vec![Span::raw(format!("B: {}", cpu.registers.get_flag(crate::hardware::registers::Flag::B)))])), 
                ListItem::new(Spans::from(vec![Span::raw(format!("D: {}", cpu.registers.get_flag(crate::hardware::registers::Flag::D)))])), 
                ListItem::new(Spans::from(vec![Span::raw(format!("I: {}", cpu.registers.get_flag(crate::hardware::registers::Flag::I)))])), 
                ListItem::new(Spans::from(vec![Span::raw(format!("Z: {}", cpu.registers.get_flag(crate::hardware::registers::Flag::Z)))])), 
                ListItem::new(Spans::from(vec![Span::raw(format!("C: {}", cpu.registers.get_flag(crate::hardware::registers::Flag::C)))])), 
                ListItem::new(Spans::from(vec![Span::raw(format!("U: {}", cpu.registers.get_flag(crate::hardware::registers::Flag::U)))])), 

            ];
            let list = List::new(list_elements)
                .block(Block::default().borders(Borders::ALL).title("Status"))
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">> ");
            list
        };

        let registers_list = build_registers_list(&app.inner_machine_state.cpu);
        f.render_widget(registers_list, chunks[1]);
        let status_list = build_status_view(&app.inner_machine_state.cpu);
        f.render_widget(status_list, chunks[0]);

    }

    pub fn disassemble(program: &Vec<u8>) -> Vec<String> {
        let mut string_builder = vec![];
        string_builder.push("test".to_string());
        string_builder
    }
}

