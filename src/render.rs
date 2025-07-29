use crate::coord::Coord;

use crate::world::World;

use crossbeam::channel::{Receiver, Sender};

use crossterm::cursor::{Hide , MoveTo, Show};
use crossterm::style::{style, Color, Stylize};

use crossterm::ExecutableCommand;
use std::io::{self, Stdout};

fn goto_coord(stdout: &mut Stdout, coord:Coord) {
    stdout
        .execute(MoveTo(coord.col as u16, coord.row as u16))
        .unwrap();
}

pub fn render_loop(world_rx: Receiver<World>, main_tx: Sender<World>) {
    let stdout = &mut io::stdout();
    stdout.execute(Hide).unwrap();



    let mut world = world_rx.recv().unwrap();
    let game_title_coord = Coord::new(world.floor.rows, 0);
    goto_coord(stdout, Coord::new(0, 0));

    {
        let tiles =&world.floor.tiles;
        for row in tiles {
            for tile in row {
                print!("{}", tile);
            }
            println!("\r\n");
        }
    }

    main_tx.send(world).unwrap();



    loop {
        world = match world_rx.recv() {
            Ok(w) => w,
            Err(_) => {
                break;
            }
        };

        for coord in world.dirty_coords.drain(..) {
            goto_coord(stdout, coord);
            println!("{}", world.floor.get_symbol(coord))
        }

        let player = &mut world.player;
        if player.dirty {
            player.dirty = false;

            goto_coord(stdout, player.sword_coord);
            print!("{}", style(player.sword_symbol()).with(Color::Red));

            goto_coord(stdout, player.coord);
            print!("{}", style(&player.symbol).with(Color::Blue));
        }

        let score_string = format!("Score: {}", player.score);
        goto_coord(stdout, Coord::new(world.floor.rows, world.floor.cols - score_string.len()),
        );

        print!("{}", style(score_string).with(Color::Blue));


        let monster = &mut world.monsters;
        for monster in monster.iter() {
            goto_coord(stdout, monster.coord);
            print!("{}", style(&monster.symbol).with(Color::Green));
        }

        goto_coord(stdout, game_title_coord);
        print!(
            "{}",
            style("Rusty Sword - Game if Infamy!").with(Color::White)
        );

        if main_tx.send(world).is_err() {
            break;
        }
    }

    stdout.execute(Show).unwrap();

}