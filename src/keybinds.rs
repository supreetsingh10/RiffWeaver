use futures::{future::FutureExt, select, StreamExt};
use crossterm::event::{Event, EventStream, KeyCode};

/// This module will be solely responsible for delegating the keys pressed, their possible function
/// this module will not be changing the state of the UI, nethier it will be making any request to
/// the Spotify api. 

// check how the keybindings work here. 
pub async fn check_event() -> std::io::Result<()> {
    let mut reader = EventStream::new(); 

        // let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let mut event = reader.next().fuse();

        select! {
            // _ = delay => { println!(".\r"); },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        if event == Event::Key(KeyCode::Char('w').into()) 
                        {
                            println!("Event::{:?}\r", event);
                        }
                        if event == Event::Key(KeyCode::Char('a').into()) {
                            println!("Event::{:?}\r", event);
                        }
                        if event == Event::Key(KeyCode::Char('s').into()) {
                            println!("Event::{:?}\r", event);
                        }
                        if event == Event::Key(KeyCode::Char('d').into()) {
                            println!("Event::{:?}\r", event);
                        }

                        if event == Event::Key(KeyCode::Esc.into()) {
                            println!("Event::{:?}\r", event);
                            // break;
                        }
                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => println!("Nope"),
                }
            }
        };

    Ok(())
}
