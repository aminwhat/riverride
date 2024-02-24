use rand::{thread_rng, Rng};
use std::{
    io::{stdout, Result, Stdout, Write},
    time::Duration,
};
use std::{thread, time};

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
    map: Vec<(u16, u16)>,
    maxc: u16,
    maxl: u16,
    died: bool, // TODO: create enum with died, playing, animation, paused
    next_right: u16,
    next_left: u16,
}

impl World {
    fn move_up(&mut self) {
        if self.player_l > 1 {
            self.player_l -= 1;
        }
    }

    fn move_down(&mut self, maxl: u16) {
        if self.player_l < maxl - 1 {
            self.player_l += 1;
        }
    }

    fn move_left(&mut self) {
        if self.player_c > 1 {
            self.player_c -= 1;
        }
    }

    fn move_right(&mut self, maxc: u16) {
        if self.player_c < maxc - 1 {
            self.player_c += 1;
        }
    }
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

fn physics(mut world: World) -> Result<World> {
    let mut rng = thread_rng();

    // check if player hit the ground
    if world.player_c < world.map[world.player_l as usize].0
        || world.player_c >= world.map[world.player_l as usize].1
    {
        world.died = true;
    }

    // move the map downward
    for l in (0..world.map.len() - 1).rev() {
        world.map[l + 1] = world.map[l];
    }
    if world.next_left > world.map[0].0 {
        world.map[0].0 += 1;
    }
    if world.next_left < world.map[0].0 {
        world.map[0].0 -= 1;
    }
    if world.next_right > world.map[0].1 {
        world.map[0].1 += 1;
    }
    if world.next_right < world.map[0].1 {
        world.map[0].1 -= 1;
    }
    // TODO: below randoms may 1) go outside of range
    if world.next_left == world.map[0].0 && rng.gen_range(0..10) >= 7 {
        world.next_left = rng.gen_range(world.next_left - 5..world.next_left + 5)
    }
    if world.next_right == world.map[0].1 && rng.gen_range(0..10) >= 7 {
        world.next_right = rng.gen_range(world.next_right - 5..world.next_right + 5)
    }
    if world.next_right - world.next_left < 3 {
        // todo: check abs
        world.next_right += 3;
    }
    Ok(world)
}

fn main() -> std::io::Result<()> {
    // init the screen
    let mut sc = stdout();
    let (maxc, maxl) = size().unwrap();
    sc.execute(Hide)?;
    enable_raw_mode()?;

    // init the world
    let slowness = 100;
    let mut world = World {
        player_c: maxc / 2,
        player_l: maxl - 1,
        map: vec![(maxc / 2 - 5, maxc / 2 + 5); maxl as usize],
        maxc: maxc,
        maxl: maxl,
        died: false,
        next_left: maxc / 2 - 7,
        next_right: maxc / 2 + 7,
    };

    while !world.died {
        if poll(Duration::from_millis(10))? {
            let key = read().unwrap();
            while poll(Duration::from_millis(0)).unwrap() {
                let _ = read();
            }
            match key {
                Event::Key(event) => {
                    // I'm reading from keyboard into event
                    match event.code {
                        KeyCode::Char('q') => {
                            break;
                        }
                        KeyCode::Char('w') => world.move_up(),
                        KeyCode::Up => world.move_up(),
                        KeyCode::Char('s') => world.move_down(maxl),
                        KeyCode::Down => world.move_down(maxl),
                        KeyCode::Char('a') => world.move_left(),
                        KeyCode::Left => world.move_left(),
                        KeyCode::Char('d') => world.move_right(maxc),
                        KeyCode::Right => world.move_right(maxc),
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            // Timeout expired and no `Event` is available
        }

        world = physics(world).unwrap();

        draw(&sc, &world)?;

        thread::sleep(time::Duration::from_millis(slowness));
    }

    // game is finished
    sc.queue(Clear(ClearType::All))?;
    sc.queue(MoveTo(0, 3))?;
    sc.queue(Print("Good game! Thanks.\n"))?;
    sc.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}
