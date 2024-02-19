use std::io::{stdout};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen}, ExecutableCommand};
use ratatui::{backend::{Backend, CrosstermBackend}, widgets::{Block, Borders}, Terminal}; 


// let the homepage show the user profile and the last played songs. 
// lands on the default page. 
// user should have the ability to search, play old songs or just play random stuff. 
// once the user has done that take it to the different stage. 

pub fn initialize() -> std::io::Result<()>
{
    // enable raw mode 
    //
    enable_raw_mode()?; 
    stdout().execute(EnterAlternateScreen)?;
    let back = CrosstermBackend::new(stdout());
    let b = back.size().unwrap();

    let mut term = Terminal::new(back)?;
    term.draw(|f| {
       let _ = f.render_widget(Block::new().borders(Borders::ALL), b);
    });

    Ok(())
}

// updates the app per eventloop. 
pub fn update() 
{

}
