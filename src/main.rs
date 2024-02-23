use std::{
    io::{stdout, Stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};

struct World {
    player_c: u16,
    player_l: u16,
    maxc: u16,
    maxl: u16,
    map: Vec<(u16, u16)>,
}

fn draw(mut sc: &Stdout, world: &World) -> std::io::Result<()> {
    sc.queue(Clear(ClearType::All))?;

    // draw the map
    for l in 0..world.map.len() {
        sc.queue(MoveTo(0, l as u16))?;
        sc.queue(Print("+".repeat(world.map[l].0 as usize)))?;
        sc.queue(MoveTo(world.map[l].1, l as u16))?;
        sc.queue(Print("+".repeat((world.maxc - world.map[l].1) as usize)))?;
    }

    // draw the player
    sc.queue(MoveTo(world.player_c, world.player_l))?;
    sc.queue(Print("P"))?;

    sc.flush()?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    // init the screen
    let mut sc = stdout();
    let (maxc, maxl) = size().unwrap();
    sc.execute(Hide)?;
    enable_raw_mode()?;

    // init the game
    let mut world: World = World {
        player_c: maxc / 2,
        player_l: maxl - 1,
        maxc: maxc,
        maxl: maxl,
        map: vec![((maxc / 2) - 5, (maxc / 2) + 5); maxl as usize],
    };

    loop {
        if poll(Duration::from_millis(10))? {
            let key = read().unwrap();
            match key {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('w') => {
                        if world.player_l > 1 {
                            world.player_l -= 1
                        };
                    }
                    KeyCode::Char('s') => {
                        if world.player_l < maxl - 1 {
                            world.player_l += 1
                        };
                    }
                    KeyCode::Char('a') => {
                        if world.player_c > 1 {
                            world.player_c -= 1
                        };
                    }
                    KeyCode::Char('d') => {
                        if world.player_c < maxc - 1 {
                            world.player_c += 1
                        };
                    }
                    _ => {}
                },
                _ => {}
            }
        } else {
            // Timeout expired and no `Event` is available
        }

        // physics()

        draw(&sc, &world)?;
    }
    sc.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}
