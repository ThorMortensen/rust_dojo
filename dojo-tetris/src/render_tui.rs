use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io, thread,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Widget},
    Frame, Terminal,
};

struct Menu<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> Menu<T> {
    fn set_items(items: Vec<T>) -> Menu<T> {
        Menu {
            state: ListState::default(),
            items,
        }
    }

    fn navigate(&mut self, direction: i32) {
        self.state.select(
            match *self.state.selected().get_or_insert(0) as i32 + direction {
                i if i < 0 => Some(self.items.len()),
                i if i > self.items.len() as i32 => Some(0),
                i => Some(i as usize),
            },
        );
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

struct Context<'a> {
    main_menu: Menu<&'a str>,
}

struct App<'a> {
    ctx: Vec<Context<'a>>,
}

pub fn run_tui() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut ctx = Context {
        main_menu: Menu::set_items(vec!["Item 1", "Item 2", "Item 3", "Item 4"]),
    };
    ctx.main_menu.state.select(Some(0));

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);
    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            draw_menu1(f, &mut ctx);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Left => ctx.main_menu.unselect(),
                    KeyCode::Down => ctx.main_menu.navigate(1),
                    KeyCode::Up => ctx.main_menu.navigate(-1),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            // app.on_tick();
            last_tick = Instant::now();
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn drawText<B: Backend>(f: &mut Frame<B>, ctx: &mut Context) {
    
}

fn draw_menu1<B: Backend>(f: &mut Frame<B>, ctx: &mut Context) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default().title("Block 2").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
    let block = Block::default().title("Block 3").borders(Borders::ALL);

    f.render_widget(block, chunks[3]);

    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = ctx
        .main_menu
        .items
        .iter()
        .map(|i| ListItem::new(*i).style(Style::default().fg(Color::White)))
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut ctx.main_menu.state);
}
