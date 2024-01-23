use std::fmt::{Display, Write};
use std::ops::{Index, IndexMut};

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Colour {
    White,
    Red,
    Blue,
    Orange,
    Green,
    Yellow,
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            White => f.write_str("W"),
            Red => f.write_str("R"),
            Blue => f.write_str("B"),
            Orange => f.write_str("O"),
            Green => f.write_str("G"),
            Yellow => f.write_str("Y"),
        }
    }
}

use Colour::*;

use Colour::Blue as B;
use Colour::Green as G;
use Colour::Orange as O;
use Colour::Red as R;
use Colour::White as W;
use Colour::Yellow as Y;

// // Old cube
// #[rustfmt::skip]
// static EDGES: [[Edge; 4]; 6] = [
//     [Edge(White, Red), Edge(White, Green), Edge(White, Orange), Edge(White, Yellow)],
//     [Edge(Red, Blue), Edge(Red, Yellow), Edge(Red, Orange), Edge(Red, White)],
//     [Edge(Blue, Red), Edge(Blue, Green), Edge(Blue, Orange), Edge(Blue, Yellow)],
//     [Edge(Orange, White), Edge(Orange, Green), Edge(Orange, Red), Edge(Orange, Blue)],
//     [Edge(Green, White), Edge(Green, Blue), Edge(Green, Yellow), Edge(Green, Orange)],
//     [Edge(Yellow, White), Edge(Yellow, Green), Edge(Yellow, Red), Edge(Yellow, Blue)],
// ];
// New cube
#[rustfmt::skip]
static EDGES: [[Edge; 4]; 6] = [
    [Edge(White, Red), Edge(White, Blue), Edge(White, Orange), Edge(White, Green)],
    [Edge(Red, White), Edge(Red, Blue), Edge(Red, Green), Edge(Red, Yellow)],
    [Edge(Blue, White), Edge(Blue, Red), Edge(Blue, Orange), Edge(Blue, Yellow)],
    [Edge(Orange, White), Edge(Orange, Blue), Edge(Orange, Green), Edge(Orange, Yellow)],
    [Edge(Green, White), Edge(Green, Red), Edge(Green, Orange), Edge(Green, Yellow)],
    [Edge(Yellow, Red), Edge(Yellow, Blue), Edge(Yellow, Orange), Edge(Yellow, Green)],
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum FaceName {
    Top,
    Left,
    Front,
    Right,
    Back,
    Bottom,
}

use FaceName::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Edge(Colour, Colour);

#[derive(Debug, PartialEq, Eq)]
struct Face([Colour; 9]);

impl Face {
    fn colour(&self) -> Colour {
        self.0[4]
    }

    fn rotate(&mut self, clockwise: bool) {
        let top = [self.0[0], self.0[1]];
        let right = [self.0[2], self.0[5]];
        let bottom = [self.0[7], self.0[8]];
        let left = [self.0[3], self.0[6]];

        if clockwise {
            // top->right
            self.0[2] = top[0];
            self.0[5] = top[1];
            // right->bottom
            self.0[8] = right[0];
            self.0[7] = right[1];
            // bottom->left
            self.0[3] = bottom[0];
            self.0[6] = bottom[1];
            // left->top
            self.0[0] = left[1];
            self.0[1] = left[0];
        } else {
            // top->left
            self.0[6] = top[0];
            self.0[3] = top[1];
            // left->bottom
            self.0[7] = left[0];
            self.0[8] = left[1];
            // bottom->right
            self.0[5] = bottom[0];
            self.0[2] = bottom[1];
            // right->top
            self.0[0] = right[0];
            self.0[1] = right[1];
        }
    }
}

type Pos = u8;

/*
       Top
       X X X
       X X X
       X X X
Left   Front  Right  Back
X X X  X X X  X X X  X X X
X X X  X X X  X X X  X X X
X X X  X X X  X X X  X X X
       Bottom
       X X X
       X X X
       X X X
*/
#[derive(Debug)]
pub struct Cube {
    // top, left, front, right, back, bottom
    faces: [Face; 6],
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("       Top\n")?;
        f.write_fmt(format_args!(
            "       {} {} {}\n",
            self[Top].0[0], self[Top].0[1], self[Top].0[2]
        ))?;
        f.write_fmt(format_args!(
            "       {} {} {}\n",
            self[Top].0[3], self[Top].0[4], self[Top].0[5]
        ))?;
        f.write_fmt(format_args!(
            "       {} {} {}\n",
            self[Top].0[6], self[Top].0[7], self[Top].0[8]
        ))?;
        f.write_str("Left   Front  Right  Back\n")?;
        for row in 0..3 {
            for (i, face) in self.faces[1..5].iter().enumerate() {
                for col in 0..3 {
                    let idx = row * 3 + col;
                    f.write_fmt(format_args!("{}", face.0[idx]))?;
                    if col != 3 && i != 4 {
                        f.write_char(' ')?;
                    }
                }
                if i != 4 {
                    f.write_char(' ')?;
                }
            }
            f.write_char('\n')?;
        }
        f.write_str("       Bottom\n")?;
        f.write_fmt(format_args!(
            "       {} {} {}\n",
            self[Bottom].0[0], self[Bottom].0[1], self[Bottom].0[2]
        ))?;
        f.write_fmt(format_args!(
            "       {} {} {}\n",
            self[Bottom].0[3], self[Bottom].0[4], self[Bottom].0[5]
        ))?;
        f.write_fmt(format_args!(
            "       {} {} {}\n",
            self[Bottom].0[6], self[Bottom].0[7], self[Bottom].0[8]
        ))?;
        Ok(())
    }
}

impl Cube {
    // Old cube
    /*
    #[rustfmt::skip]
    const SOLVED: Cube = Cube::make_cube([
              /*Top*/
              B, B, B,
              B, B, B,
              B, B, B,
    /*Left    Front     Right     Back*/
    R, R, R,  Y, Y, Y,  G, G, G,  O, O, O,
    R, R, R,  Y, Y, Y,  G, G, G,  O, O, O,
    R, R, R,  Y, Y, Y,  G, G, G,  O, O, O,
              /*Bottom*/
              W, W, W,
              W, W, W,
              W, W, W,
    ]);
    */
    // New cube
    #[rustfmt::skip]
    const SOLVED: Cube = Cube::make_cube([
              /*Top*/
              Y, Y, Y,
              Y, Y, Y,
              Y, Y, Y,
    /*Left    Front     Right     Back*/
    O, O, O,  B, B, B,  R, R, R,  G, G, G,
    O, O, O,  B, B, B,  R, R, R,  G, G, G,
    O, O, O,  B, B, B,  R, R, R,  G, G, G,
              /*Bottom*/
              W, W, W,
              W, W, W,
              W, W, W,
    ]);

    #[rustfmt::skip]
    pub const fn make_cube(colours: [Colour; 54]) -> Cube {
        Cube {
            faces: [
                Face([
                    colours[0], colours[1], colours[2],
                    colours[3], colours[4], colours[5],
                    colours[6], colours[7], colours[8],
                ]),
                Face([
                    colours[9], colours[10], colours[11],
                    colours[21], colours[22], colours[23],
                    colours[33], colours[34], colours[35],
                ]),
                Face([
                    colours[12], colours[13], colours[14],
                    colours[24], colours[25], colours[26],
                    colours[36], colours[37], colours[38],
                ]),
                Face([
                    colours[15], colours[16], colours[17],
                    colours[27], colours[28], colours[29],
                    colours[39], colours[40], colours[41],
                ]),
                Face([
                    colours[18], colours[19], colours[20],
                    colours[30], colours[31], colours[32],
                    colours[42], colours[43], colours[44],
                ]),
                Face([
                    colours[45], colours[46], colours[47],
                    colours[48], colours[49], colours[50],
                    colours[51], colours[52], colours[53],
                ]),
            ],
        }
    }

    fn face_colour(&self, face: FaceName) -> Colour {
        self[face].colour()
    }

    fn bottom_cross_edges(&self) -> [Edge; 4] {
        EDGES[self.face_colour(Bottom) as usize]
    }

    fn edges(&self) -> impl Iterator<Item = (Edge, Pos)> {
        let mut result = [(Edge(White, White), 0); 12];

        // Top
        {
            result[0] = (Edge(self[Top].0[1], self[Back].0[1]), 1);
            result[1] = (Edge(self[Left].0[1], self[Top].0[3]), 3);
            result[2] = (Edge(self[Top].0[5], self[Right].0[1]), 5);
            result[3] = (Edge(self[Top].0[7], self[Front].0[1]), 7);
        }
        // Middle
        {
            result[4] = (Edge(self[Left].0[3], self[Back].0[5]), 9);
            result[5] = (Edge(self[Right].0[5], self[Back].0[3]), 11);
            result[6] = (Edge(self[Left].0[5], self[Front].0[3]), 15);
            result[7] = (Edge(self[Front].0[5], self[Right].0[3]), 17);
        }
        // Bottom
        {
            result[8] = (Edge(self[Back].0[7], self[Bottom].0[7]), 19);
            result[9] = (Edge(self[Left].0[7], self[Bottom].0[3]), 21);
            result[10] = (Edge(self[Right].0[7], self[Bottom].0[5]), 23);
            result[11] = (Edge(self[Front].0[7], self[Bottom].0[1]), 25);
        }

        // TODO: extract to e.g. Edge constructor so all edges are canonical.
        for (Edge(a, b), _) in result.iter_mut() {
            if *a > *b {
                std::mem::swap(a, b);
            }
        }

        result.into_iter()
    }

    fn find_edge(&self, edge: Edge) -> Pos {
        self.edges().find(|e| e.0 == edge).unwrap().1
    }

    fn make_move(&mut self, action: Move) {
        /// Side slices are specified: ["Top", "Right", "Bottom", "Left"], relative to the face-to-be-moved.
        /// For clockwise, slices are rotated: top->right->bottom->left->top
        /// For anti-clockwise, slices are rotated: top->left->bottom->right->top
        macro_rules! rotate_sides {
            (
                $([
                    $clock:ident, $anti_clock:ident, $face:ident,
                    [$top:ident, $t1:literal, $t2:literal, $t3:literal],
                    [$right:ident, $r1:literal, $r2:literal, $r3:literal],
                    [$bottom:ident, $b1:literal, $b2:literal, $b3:literal],
                    [$left:ident, $l1:literal, $l2:literal, $l3:literal],
                ],)+
            ) => {
                match action {
                    $(
                        Move::$clock => {
                            let top = [self[$top].0[$t1], self[$top].0[$t2], self[$top].0[$t3]];
                            let right = [self[$right].0[$r1], self[$right].0[$r2], self[$right].0[$r3]];
                            let bottom = [self[$bottom].0[$b1], self[$bottom].0[$b2], self[$bottom].0[$b3]];
                            let left = [self[$left].0[$l1], self[$left].0[$l2], self[$left].0[$l3]];

                            // top->right
                            self[$right].0[$r1] = top[0];
                            self[$right].0[$r2] = top[1];
                            self[$right].0[$r3] = top[2];
                            // right->bottom
                            self[$bottom].0[$b1] = right[0];
                            self[$bottom].0[$b2] = right[1];
                            self[$bottom].0[$b3] = right[2];
                            // bottom->left
                            self[$left].0[$l1] = bottom[0];
                            self[$left].0[$l2] = bottom[1];
                            self[$left].0[$l3] = bottom[2];
                            // left->top
                            self[$top].0[$t1] = left[0];
                            self[$top].0[$t2] = left[1];
                            self[$top].0[$t3] = left[2];

                            self[$face].rotate(true);
                        }
                        Move::$anti_clock => {
                            let top = [self[$top].0[$t1], self[$top].0[$t2], self[$top].0[$t3]];
                            let right = [self[$right].0[$r1], self[$right].0[$r2], self[$right].0[$r3]];
                            let bottom = [self[$bottom].0[$b1], self[$bottom].0[$b2], self[$bottom].0[$b3]];
                            let left = [self[$left].0[$l1], self[$left].0[$l2], self[$left].0[$l3]];

                            // top->left
                            self[$left].0[$l1] = top[2];
                            self[$left].0[$l2] = top[1];
                            self[$left].0[$l3] = top[0];
                            // left->bottom
                            self[$bottom].0[$b1] = left[0];
                            self[$bottom].0[$b2] = left[1];
                            self[$bottom].0[$b3] = left[2];
                            // bottom->right
                            self[$right].0[$r1] = bottom[2];
                            self[$right].0[$r2] = bottom[1];
                            self[$right].0[$r3] = bottom[0];
                            // right->top
                            self[$top].0[$t1] = right[0];
                            self[$top].0[$t2] = right[1];
                            self[$top].0[$t3] = right[2];

                            self[$face].rotate(false);
                        }
                    )+
                }
            };
        }

        #[rustfmt::skip]
        rotate_sides!(
            [
                F, FP, Front,
                [Top, 6, 7, 8],
                [Right, 0, 3, 6],
                [Bottom, 0, 1, 2],
                [Left, 2, 5, 8],
            ],
            [
                B, BP, Back,
                [Top, 0, 1, 2],
                [Left, 0, 3, 6],
                [Bottom, 6, 7, 8],
                [Right, 2, 5, 8],
            ],
            [
                U, UP, Top,
                [Back, 0, 1, 2],
                [Right, 0, 1, 2],
                [Front, 0, 1, 2],
                [Left, 0, 1, 2],
            ],
            [
                D, DP, Bottom,
                [Front, 6, 7, 8],
                [Right, 6, 7, 8],
                [Back, 6, 7, 8],
                [Left, 6, 7, 8],
            ],
            [
                L, LP, Left,
                [Top, 0, 3, 6],
                [Front, 0, 3, 6],
                [Bottom, 0, 3, 6],
                [Back, 2, 5, 8],
            ],
            [
                R, RP, Right,
                [Top, 8, 5, 2],
                [Back, 0, 3, 6],
                [Bottom, 2, 5, 8],
                [Front, 2, 5, 8],
            ],
        );
    }
}

impl Index<FaceName> for Cube {
    type Output = Face;

    fn index(&self, index: FaceName) -> &Self::Output {
        &self.faces[index as usize]
    }
}

impl IndexMut<FaceName> for Cube {
    fn index_mut(&mut self, index: FaceName) -> &mut Self::Output {
        &mut self.faces[index as usize]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
    F,
    B,
    U,
    D,
    L,
    R,

    FP,
    BP,
    UP,
    DP,
    LP,
    RP,
}

impl Move {
    fn reverse(&self) -> Self {
        match self {
            Self::F => Self::FP,
            Self::B => Self::BP,
            Self::U => Self::UP,
            Self::D => Self::DP,
            Self::L => Self::LP,
            Self::R => Self::RP,

            Self::FP => Self::F,
            Self::BP => Self::B,
            Self::UP => Self::U,
            Self::DP => Self::D,
            Self::LP => Self::L,
            Self::RP => Self::R,
        }
    }
}

const fn make_identity_map() -> [u8; 54] {
    let mut result = [0; 54];
    let mut i = 0;
    while i < 54 {
        result[i as usize] = i;
        i += 1;
    }
    result
}
const POS_IDENTITY_MAP: [u8; 54] = make_identity_map();

fn update_pos_after_move(pos: &mut Pos, action: Move) {
    macro_rules! make_maps {
        (
            $(
                [
                    $move:ident, $anti_move:ident,
                    [$($s:literal=>$t:literal,)+]
                ],
            )+
        ) => {
            match action {
                $(
                    Move::$move => {
                        static MAP: [u8; 54] = {
                            const fn mk() -> [u8; 54]{
                                let mut map = POS_IDENTITY_MAP;
                                $(map[$s] = $t;)+
                                map
                            }
                            mk()
                        };
                        *pos = MAP[*pos as usize];
                    }
                    Move::$anti_move => {
                        static MAP: [u8; 54] = {
                            const fn mk() -> [u8; 54]{
                                let mut map = POS_IDENTITY_MAP;
                                $(map[$s] = $t;)+
                                map
                            }
                            mk()
                        };
                        *pos = MAP[*pos as usize];
                    }
                )+
            }


        };
    }
    #[rustfmt::skip]
    make_maps!(
        [
            F, FP,
            [
                6 => 8,
                7 => 16,
                8 => 26,
                15 => 7,
                16 => 25,
                24 => 6,
                25 => 15,
                26 => 24,
            ]
        ],
        [
            B, BP,
            [
                0 => 2,
                1 => 11,
                2 => 20,
                9 => 1,
                11 => 19,
                18 => 0,
                19 => 9,
                20 => 18,
            ]
        ],
        [
            U, UP,
            [
                0 => 2,
                1 => 5,
                2 => 8,
                3 => 1,
                5 => 7,
                6 => 0,
                7 => 3,
                8 => 6,
            ]
        ],
        [
            D, DP,
            [
                18 => 20,
                19 => 23,
                20 => 26,
                21 => 19,
                23 => 25,
                24 => 18,
                25 => 21,
                26 => 24,
            ]
        ],
        [
            L, LP,
            [
                0 => 6,
                3 => 15,
                6 => 24,
                9 => 3,
                15 => 21,
                18 => 0,
                21 => 9,
                24 => 18,
            ]
        ],
        [
            R, RP,
            [
                2 => 20,
                5 => 11,
                8 => 2,
                11 => 23,
                17 => 5,
                20 => 26,
                23 => 17,
                26 => 8,
            ]
        ],
    );
}

struct Solver {
    cube: Cube,
    move_stack: Vec<Move>,
}

impl Solver {
    fn make_move(&mut self, action: Move) {
        self.cube.make_move(action);
        self.move_stack.push(action);
    }

    fn move_bottom_cross_edge(&mut self, mut source: Pos, edge: Edge) {
        // Move to top layer
        {
            let cur_layer = source / 9;
            match cur_layer {
                0 => {
                    // Already in top layer
                }
                1 => {
                    // Middle layer
                    match source {
                        9 => {
                            self.make_move(Move::L);
                            update_pos_after_move(&mut source, Move::L);
                        }
                        11 => {
                            self.make_move(Move::RP);
                            update_pos_after_move(&mut source, Move::RP);
                        }
                        15 => {
                            self.make_move(Move::L);
                            update_pos_after_move(&mut source, Move::L);
                        }
                        17 => {
                            self.make_move(Move::R);
                            update_pos_after_move(&mut source, Move::R);
                        }
                        _ => unreachable!(),
                    }
                }
                2 => {
                    // Bottom layer
                    match source {
                        19 => {
                            self.make_move(Move::B);
                            update_pos_after_move(&mut source, Move::B);
                            self.make_move(Move::B);
                            update_pos_after_move(&mut source, Move::B);
                        }
                        21 => {
                            self.make_move(Move::L);
                            update_pos_after_move(&mut source, Move::L);
                            self.make_move(Move::L);
                            update_pos_after_move(&mut source, Move::L);
                        }
                        23 => {
                            self.make_move(Move::R);
                            update_pos_after_move(&mut source, Move::R);
                            self.make_move(Move::R);
                            update_pos_after_move(&mut source, Move::R);
                        }
                        25 => {
                            self.make_move(Move::F);
                            update_pos_after_move(&mut source, Move::F);
                            self.make_move(Move::F);
                            update_pos_after_move(&mut source, Move::F);
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
            // Move target piece out of the way.
            match cur_layer {
                0 => {}
                1 | 2 => {
                    self.make_move(Move::U);
                    update_pos_after_move(&mut source, Move::U);
                }
                _ => unreachable!(),
            }
            // Restore sides.
            match cur_layer {
                0 => {}
                1 => {
                    let m = self.move_stack[self.move_stack.len() - 2];
                    self.make_move(m.reverse());
                    update_pos_after_move(&mut source, m.reverse());
                }
                2 => {
                    let m = self.move_stack[self.move_stack.len() - 2];
                    self.make_move(m.reverse());
                    update_pos_after_move(&mut source, m.reverse());
                    self.make_move(m.reverse());
                    update_pos_after_move(&mut source, m.reverse());
                }
                _ => unreachable!(),
            }
        }

        let top_face_pos = source % 9;
        if self.cube[Top].0[top_face_pos as usize] == self.cube.face_colour(Bottom) {
            // Bottom colour is facing up on the edge.

            let non_bottom_colour = if edge.0 == self.cube.face_colour(Bottom) {
                edge.1
            } else {
                edge.0
            };

            let target = if non_bottom_colour == self.cube.face_colour(Back) {
                0
            } else if non_bottom_colour == self.cube.face_colour(Right) {
                1
            } else if non_bottom_colour == self.cube.face_colour(Front) {
                2
            } else if non_bottom_colour == self.cube.face_colour(Left) {
                3
            } else {
                unreachable!();
            };

            let cur = match top_face_pos {
                1 => 0,
                3 => 3,
                5 => 1,
                7 => 2,
                _ => unreachable!(),
            };

            let mut num_moves = target - cur;

            if num_moves == 3 {
                num_moves = -1;
            } else if num_moves == -3 {
                num_moves = 1;
            }

            let action = if num_moves < 0 { Move::UP } else { Move::U };

            for _ in 0..i32::abs(num_moves) {
                self.make_move(action);
                update_pos_after_move(&mut source, action);
            }

            let target_face_move = if non_bottom_colour == self.cube.face_colour(Back) {
                Move::B
            } else if non_bottom_colour == self.cube.face_colour(Right) {
                Move::R
            } else if non_bottom_colour == self.cube.face_colour(Front) {
                Move::F
            } else if non_bottom_colour == self.cube.face_colour(Left) {
                Move::L
            } else {
                unreachable!();
            };

            self.make_move(target_face_move);
            update_pos_after_move(&mut source, target_face_move);
            self.make_move(target_face_move);
            update_pos_after_move(&mut source, target_face_move);
        } else {
            // Bottom colour is facing out on the edge.

            // TODO: expose insertion slot
            // TODO: insert into slot
            // TODO: restore insertion slot
            todo!();
        }
    }

    fn solve_bottom_cross(&mut self) {
        for target_edge in self.cube.bottom_cross_edges() {
            let cur_pos = self.cube.find_edge(target_edge);
            let target_pos = Cube::SOLVED.find_edge(target_edge);
            if cur_pos == target_pos {
                continue;
            }
            self.move_bottom_cross_edge(cur_pos, target_edge);
        }
    }
}

pub fn solve(cube: Cube) -> Vec<Move> {
    let mut solver = Solver {
        cube,
        move_stack: Vec::new(),
    };
    println!("{}", solver.cube);
    solver.solve_bottom_cross();
    println!("{}", solver.cube);

    solver.move_stack
}
