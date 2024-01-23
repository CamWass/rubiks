use super::*;

// Using new cube

fn test_move(m: Move, output: [Colour; 54]) {
    let mut input = Cube::SOLVED;
    input.make_move(m);
    let output = Cube::make_cube(output).faces;
    assert!(input.faces == output);
}

#[test]
fn test_front() {
    #[rustfmt::skip]
    let output = [
              /*Top*/
              Y, Y, Y,
              Y, Y, Y,
              O, O, O,
    /*Left    Front     Right     Back*/
    O, O, W,  B, B, B,  Y, R, R,  G, G, G,
    O, O, W,  B, B, B,  Y, R, R,  G, G, G,
    O, O, W,  B, B, B,  Y, R, R,  G, G, G,
              /*Bottom*/
              R, R, R,
              W, W, W,
              W, W, W,
    ];
    test_move(Move::F, output);
}
#[test]
fn test_back() {
    #[rustfmt::skip]
    let output = [
              /*Top*/
              R, R, R,
              Y, Y, Y,
              Y, Y, Y,
    /*Left    Front     Right     Back*/
    Y, O, O,  B, B, B,  R, R, W,  G, G, G,
    Y, O, O,  B, B, B,  R, R, W,  G, G, G,
    Y, O, O,  B, B, B,  R, R, W,  G, G, G,
              /*Bottom*/
              W, W, W,
              W, W, W,
              O, O, O,
    ];
    test_move(Move::B, output);
}
#[test]
fn test_up() {
    #[rustfmt::skip]
    let output = [
              /*Top*/
              Y, Y, Y,
              Y, Y, Y,
              Y, Y, Y,
    /*Left    Front     Right     Back*/
    B, B, B,  R, R, R,  G, G, G,  O, O, O,
    O, O, O,  B, B, B,  R, R, R,  G, G, G,
    O, O, O,  B, B, B,  R, R, R,  G, G, G,
              /*Bottom*/
              W, W, W,
              W, W, W,
              W, W, W,
    ];
    test_move(Move::U, output);
}
#[test]
fn test_down() {
    #[rustfmt::skip]
    let output = [
              /*Top*/
              Y, Y, Y,
              Y, Y, Y,
              Y, Y, Y,
    /*Left    Front     Right     Back*/
    O, O, O,  B, B, B,  R, R, R,  G, G, G,
    O, O, O,  B, B, B,  R, R, R,  G, G, G,
    G, G, G,  O, O, O,  B, B, B,  R, R, R,
              /*Bottom*/
              W, W, W,
              W, W, W,
              W, W, W,
    ];
    test_move(Move::D, output);
}
#[test]
fn test_left() {
    #[rustfmt::skip]
    let output = [
              /*Top*/
              G, Y, Y,
              G, Y, Y,
              G, Y, Y,
    /*Left    Front     Right     Back*/
    O, O, O,  Y, B, B,  R, R, R,  G, G, W,
    O, O, O,  Y, B, B,  R, R, R,  G, G, W,
    O, O, O,  Y, B, B,  R, R, R,  G, G, W,
              /*Bottom*/
              B, W, W,
              B, W, W,
              B, W, W,
    ];
    test_move(Move::L, output);
}
#[test]
fn test_right() {
    #[rustfmt::skip]
    let output = [
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
    ];
    test_move(Move::R, output);
}
#[test]
fn test_front_prime() {
    #[rustfmt::skip]
    let output = [
              /*Top*/
              Y, Y, Y,
              Y, Y, Y,
              R, R, R,
    /*Left    Front     Right     Back*/
    O, O, Y,  B, B, B,  W, R, R,  G, G, G,
    O, O, Y,  B, B, B,  W, R, R,  G, G, G,
    O, O, Y,  B, B, B,  W, R, R,  G, G, G,
              /*Bottom*/
              O, O, O,
              W, W, W,
              W, W, W,
    ];
    test_move(Move::FP, output);
}
#[test]
fn test_back_prime() {
    #[rustfmt::skip]
    let output = [
              /*Top*/
              O, O, O,
              Y, Y, Y,
              Y, Y, Y,
    /*Left    Front     Right     Back*/
    W, O, O,  B, B, B,  R, R, Y,  G, G, G,
    W, O, O,  B, B, B,  R, R, Y,  G, G, G,
    W, O, O,  B, B, B,  R, R, Y,  G, G, G,
              /*Bottom*/
              W, W, W,
              W, W, W,
              R, R, R,
    ];
    test_move(Move::BP, output);
}
#[test]
fn test_up_prime() {
    #[rustfmt::skip]
    let output = [
              /*Top*/
              Y, Y, Y,
              Y, Y, Y,
              Y, Y, Y,
    /*Left    Front     Right     Back*/
    G, G, G,  O, O, O,  B, B, B,  R, R, R,
    O, O, O,  B, B, B,  R, R, R,  G, G, G,
    O, O, O,  B, B, B,  R, R, R,  G, G, G,
              /*Bottom*/
              W, W, W,
              W, W, W,
              W, W, W,
    ];
    test_move(Move::UP, output);
}
#[test]
fn test_down_prime() {
    #[rustfmt::skip]
    let output = [
              /*Top*/
              Y, Y, Y,
              Y, Y, Y,
              Y, Y, Y,
    /*Left    Front     Right     Back*/
    O, O, O,  B, B, B,  R, R, R,  G, G, G,
    O, O, O,  B, B, B,  R, R, R,  G, G, G,
    B, B, B,  R, R, R,  G, G, G,  O, O, O,
              /*Bottom*/
              W, W, W,
              W, W, W,
              W, W, W,
    ];
    test_move(Move::DP, output);
}
#[test]
fn test_left_prime() {
    #[rustfmt::skip]
    let output = [
              /*Top*/
              B, Y, Y,
              B, Y, Y,
              B, Y, Y,
    /*Left    Front     Right     Back*/
    O, O, O,  W, B, B,  R, R, R,  G, G, Y,
    O, O, O,  W, B, B,  R, R, R,  G, G, Y,
    O, O, O,  W, B, B,  R, R, R,  G, G, Y,
              /*Bottom*/
              G, W, W,
              G, W, W,
              G, W, W,
    ];
    test_move(Move::LP, output);
}
#[test]
fn test_right_prime() {
    #[rustfmt::skip]
    let output = [
              /*Top*/
              Y, Y, G,
              Y, Y, G,
              Y, Y, G,
    /*Left    Front     Right     Back*/
    O, O, O,  B, B, Y,  R, R, R,  W, G, G,
    O, O, O,  B, B, Y,  R, R, R,  W, G, G,
    O, O, O,  B, B, Y,  R, R, R,  W, G, G,
              /*Bottom*/
              W, W, B,
              W, W, B,
              W, W, B,
    ];
    test_move(Move::RP, output);
}
