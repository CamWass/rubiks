use solver::{solve, Colour, Cube};

use Colour::Blue as B;
use Colour::Green as G;
use Colour::Orange as O;
use Colour::Red as R;
use Colour::White as W;
use Colour::Yellow as Y;

fn main() {
    // Start with solved cube, yellow front, white bottom.
    // Turn right side up.
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

    solve(cube);
}
