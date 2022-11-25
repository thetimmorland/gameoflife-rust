use std::{
    io,
    time::{Duration, Instant},
};

use clap::Parser;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use gameoflife::Grid;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

/// A simple Conway's Game of Life implementation.
#[derive(Parser, Debug)]
struct Args {
    /// The delay between new generations in seconds.
    #[arg(default_value_t = 0.1)]
    tick_rate: f64,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create simulation and run it
    let (width, height) = terminal::size()?;
    let grid = Grid::random(width as usize, height as usize);
    let tick_rate = Duration::from_secs_f64(args.tick_rate);
    let res = run_simulation(&mut terminal, grid, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // print errors
    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_simulation<B: Backend>(
    terminal: &mut Terminal<B>,
    mut grid: Grid,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| f.render_widget(&grid, f.size()))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_secs(0));

        if event::poll(timeout)? {
            // quit on any key
            if let Event::Key(_) = event::read()? {
                return Ok(());
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
            grid.update();
        }
    }
}
