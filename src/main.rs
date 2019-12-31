#[macro_use] extern crate log;
use std::io;
use std::thread;
use std::time;
use std::sync::mpsc;
use std::process::exit;

use termion::event;
use termion::input::TermRead;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use tui::Terminal;
use tui::backend::TermionBackend;

mod rtop;

use crate::rtop::app::App;
use crate::rtop::cmd::Cmd;
use crate::rtop::event::Event;
use crate::rtop::error::Error;
use crate::rtop::ui::renderer::render;

fn main() {
    exit(match _main() {
       Ok(_) => 0,
       Err(err) => {
           eprintln!("error: {:?}", err);
           1
       }
    });
}

fn _main() -> Result<(), Error> {
    stderrlog::new().module(module_path!())
                    .verbosity(4)
                    .init()
                    .unwrap();

    info!("Start");
    #[cfg(feature = "gpu-monitor")]
    info!("GPU Monitoring Enabled");
    //Program
    let mut app = App::new(5000, 50)?;
    #[cfg(feature = "gpu-monitor")]
    app.init()?;
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    thread::spawn(move || {
        let tx = tx.clone();
        loop {
            tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(1000));
        }
    });

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut term_size = terminal.size()?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    let mut clk_split = 0;
    
    loop {
        let size = terminal.size().unwrap();
        if size != term_size {
            terminal.resize(size).unwrap();
            term_size = size;
        }
        let evt = rx.recv().unwrap();
        {
            match evt {
                Event::Input(input) => {
                    match app.input_handler(input) {
                        Some(command) => {
                            match command {
                                Cmd::Quit => {break},
                                _ => (),
                            }
                        },
                        None => (),
                    }
                },
                Event::Tick => {
                    if clk_split % 1 == 0 {
                        app.update()?;
                    }
                } 
            }
        }

        render(&mut terminal, &app, term_size)?;
    }
    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();
    Ok(())
}
