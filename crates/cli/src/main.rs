use solver::*;

use Colour::Blue as B;
use Colour::Green as G;
use Colour::Orange as O;
use Colour::Red as R;
use Colour::White as W;
use Colour::Yellow as Y;

fn main() {
    // Old cube
    // Start with solved cube, yellow front, white bottom.
    // Moves: R
    /*
    #[rustfmt::skip]
    let cube = Cube::make_cube([
                  /*Top*/
                  B, B, Y,
                  B, B, Y,
                  B, B, Y,
        /*Left    Front     Right     Back*/
        R, R, R,  Y, Y, W,  G, G, G,  B, O, O,
        R, R, R,  Y, Y, W,  G, G, G,  B, O, O,
        R, R, R,  Y, Y, W,  G, G, G,  B, O, O,
                  /*Bottom*/
                  W, W, O,
                  W, W, O,
                  W, W, O,
    ]);
    */
    // New cube
    // Start with solved cube, blue front, white bottom.
    // Moves: R
    #[rustfmt::skip]
    let cube1 = Cube::make_cube([
                  /*Top*/
                  Y, Y, B,
                  Y, Y, B,
                  Y, Y, B,
        /*Left    Front     Right     Back*/
        O, O, O,  B, B, W,  R, R, R,  Y, G, G,
        O, O, O,  B, B, W,  R, R, R,  Y, G, G,
        O, O, O,  B, B, W,  R, R, R,  Y, G, G,
                  /*Bottom*/
                  W, W, G,
                  W, W, G,
                  W, W, G,
    ]);
    // New cube
    // Start with solved cube, blue front, white bottom.
    // Moves: R, R, L, L, F, F, B, B
    #[rustfmt::skip]
    let cube2 = Cube::make_cube([
                  /*Top*/
                  Y, W, Y,
                  W, Y, W,
                  Y, W, Y,
        /*Left    Front     Right     Back*/
        R, O, R,  G, B, G,  O, R, O,  B, G, B,
        R, O, R,  G, B, G,  O, R, O,  B, G, B,
        R, O, R,  G, B, G,  O, R, O,  B, G, B,
                  /*Bottom*/
                  W, Y, W,
                  Y, W, Y,
                  W, Y, W,
    ]);

    // New cube
    // Start with solved cube, blue front, white bottom.
    #[rustfmt::skip]
    let cube3 = {
        let mut c = Cube::SOLVED;
        c.make_move(Move::R);
        c.make_move(Move::FP);
        c.make_move(Move::U);
        c.make_move(Move::F);
        c.make_move(Move::RP);
        c
    };

    let moves = solve(cube3);
    println!("Moves: {:#?}", moves);
    println!("Number of moves: {}", moves.len());
}
