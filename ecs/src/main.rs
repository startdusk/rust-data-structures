mod data;
mod gen;
mod store;
mod system;

use core::time;
use std::io::Write;
use store::EcsStore;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    // get keyboard input in a thready maner;
    let (ch_s, ch_r) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        for k in stdin.keys() {
            // keys depends on TermRead Trait
            ch_s.send(k).ok();
        }
    });
    let (w, h) = termion::terminal_size().unwrap();
    let (w, h) = (w as i32, h as i32);
    let mut screen = std::io::stdout().into_raw_mode().unwrap();
    let mut gen = gen::GenManager::new();
    let mut strengths = store::VecStore::new();
    let mut dirs = store::VecStore::new();
    let mut poss = store::VecStore::new();
    let mut pass = 0;

    loop {
        // create one element per loop (chioce not requirement);
        let g = gen.next();
        strengths.add(g, data::Strength { s: 1, h: 5 });
        dirs.add(g, data::Dir { vx: 9, vy: 0 });
        poss.add(
            g,
            data::Pos {
                x: rand::random::<i32>() % w,
                y: rand::random::<i32>() % h,
            },
        );

        system::dir_sys(&mut dirs, &poss);
        system::move_sys(&dirs, &mut poss);
        system::collision_sys(&poss, &mut strengths);
        system::death_sys(&mut gen, &mut strengths, &mut poss, &mut dirs);
        system::render_sys(&mut screen, &poss, &strengths);

        // print pass too
        write!(&mut screen, "{}pass={}", termion::cursor::Goto(1, 1), pass).ok();
        pass += 1;
        screen.flush().ok();

        while let Ok(Ok(k)) = ch_r.try_recv() {
            match k {
                Key::Char('q') => return,
                // Here handle any key presses to make the game do stuff
                _ => {}
            }
        }

        std::thread::sleep(time::Duration::from_millis(300));
    }
}
