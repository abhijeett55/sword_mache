use crossbeam::channel::bounded;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use rand::distributions::{Distribution, Uniform};
use rusty_audio::Audio;
use sword_mache::coord::{key_to_direction, Coord};
use sword_mache::monster::Monster;
use sword_mache::render::render_loop;
use sword_mache::timer::Timer;
use sword_mache::world::World;
use std::io;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut audio = Audio::new();
    audio.add("monster_dies", "clips/player_dies.wav");
    audio.add("monster_spawns", "clips/player_dies.wav");
    audio.add("player_dies", "clips/player_dies.wav");

    let mut world = World::new(30, 60);


    let (render_tx, render_rx) = bounded::<World>(0);
    let (main_tx, main_rx) = bounded::<World>(0);

    // Render Thread
    let render_thread = { thread::spawn(move || render_loop(render_rx, main_tx)) };

    // Game Loop
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(EnterAlternateScreen).unwrap();
    let mut rng = rand::thread_rng();
    let mut spawn_timer = Timer::from_millis(1000);
    let mut last_instant = Instant::now();
    'gameloop: loop {
        let delta = last_instant.elapsed();
        last_instant = Instant::now();
        let player = &mut world.player;

        // Player moves?
        let mut player_moved = false;
        while event::poll(Duration::default()).unwrap() {
            let an_event = event::read().unwrap();
            if let Event::Key(key_event) = an_event {
                if (key_event.code == KeyCode::Char('q')) | (key_event.code == KeyCode::Esc) {
                    break 'gameloop;
                }
                if let Some(direction) = key_to_direction(key_event) {
                    player_moved = player.travel(direction, &world.floor, &mut world.dirty_coords);
                }
            }
        }

        // Update monster timers
        for monster in world.monsters.iter_mut() {
            monster.move_timer.update(delta);
        }

        // Monsters move?
        if !player_moved {
            for monster in world.monsters.iter_mut() {
                monster.try_travel(player.coord, &mut world.dirty_coords);
            }
        }

        // Did a monster die?
        let num_monsters = world.monsters.len();
        world
            .monsters
            .retain(|monster| monster.coord != player.sword_coord);
        let num_killed = num_monsters - world.monsters.len();
        if num_killed > 0 {
            player.score += num_killed as u64;
            audio.play("monster_dies");
        }

        // Spawn a new monster!
        spawn_timer.update(delta);
        if spawn_timer.ready {
            spawn_timer = Timer::from_millis(Uniform::new(1000, 5000).sample(&mut rng));
            let spawn_coord = Coord::new(
                Uniform::new(1, world.floor.rows).sample(&mut rng),
                Uniform::new(1, world.floor.cols).sample(&mut rng),
            );
            if spawn_coord != player.coord {
                world.monsters.push(Monster::new(spawn_coord, &mut rng));
                audio.play("monster_spawns");
            }
        }

        // Did the player die?
        if world
            .monsters
            .iter()
            .any(|monster| monster.coord == player.coord)
        {
            audio.play("player_dies");
            audio.wait(); // Wait until the sound finishes, so we can hear it before quitting.
            break 'gameloop;
        }

        // Give the whole world to the renderer
        render_tx.send(world).unwrap();
        // Get the whole world back
        world = main_rx.recv().unwrap();
        // Don't exceed ~60/fps
        if let Some(t) = Duration::from_secs_f64(1. / 60.).checked_sub(last_instant.elapsed()) {
            thread::sleep(t);
        }
    }

    // Close the render_tx channel, which will trigger the render thread to exit
    drop(render_tx);
    // Wait for the render thread to actually exit
    render_thread.join().unwrap();

    stdout.execute(LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
    println!("Thanks for playing!");
}