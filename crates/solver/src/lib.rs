use std::fmt::{Display, Write};
use std::ops::{Index, IndexMut};

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

#[rustfmt::skip]
static EDGES: [[Edge; 4]; 6] = [
    [Edge(White, Red), Edge(White, Green), Edge(White, Orange), Edge(White, Yellow)],
    [Edge(Red, Blue), Edge(Red, Yellow), Edge(Red, Orange), Edge(Red, White)],
    [Edge(Blue, Red), Edge(Blue, Green), Edge(Blue, Orange), Edge(Blue, Yellow)],
    [Edge(Orange, White), Edge(Orange, Green), Edge(Orange, Red), Edge(Orange, Blue)],
    [Edge(Green, White), Edge(Green, Blue), Edge(Green, Yellow), Edge(Green, Orange)],
    [Edge(Yellow, White), Edge(Yellow, Green), Edge(Yellow, Red), Edge(Yellow, Blue)],
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

#[derive(Debug)]
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

        if !clockwise {
            todo!();
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

    fn bottom_colour(&self) -> Colour {
        self.faces[5].colour()
    }

    fn bottom_cross_edges(&self) -> [Edge; 4] {
        EDGES[self.bottom_colour() as usize]
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
        match action {
            Move::F => {
                let top = [self[Top].0[6], self[Top].0[7], self[Top].0[8]];
                let right = [self[Right].0[0], self[Right].0[3], self[Right].0[6]];
                let bottom = [self[Bottom].0[0], self[Bottom].0[1], self[Bottom].0[2]];
                let left = [self[Left].0[2], self[Left].0[5], self[Left].0[8]];

                // top->right
                self[Right].0[0] = top[0];
                self[Right].0[3] = top[1];
                self[Right].0[6] = top[2];
                // right->bottom
                self[Bottom].0[0] = right[0];
                self[Bottom].0[1] = right[1];
                self[Bottom].0[2] = right[2];
                // bottom->left
                self[Left].0[2] = bottom[0];
                self[Left].0[5] = bottom[1];
                self[Left].0[8] = bottom[2];
                // left->top
                self[Top].0[6] = left[0];
                self[Top].0[7] = left[1];
                self[Top].0[8] = left[2];

                self[Front].rotate(true);
            }
            Move::B => todo!(),
            Move::U => todo!(),
            Move::D => todo!(),
            Move::L => todo!(),
            Move::R => todo!(),
            Move::FP => todo!(),
            Move::BP => todo!(),
            Move::UP => todo!(),
            Move::DP => todo!(),
            Move::LP => todo!(),
            Move::RP => todo!(),
        }
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
        todo!()
    }
}

fn update_pos_after_move(pos: &mut Pos, action: Move) {
    todo!();
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

    fn move_bottom_cross_edge(&mut self, mut source: Pos, target: Pos) {
        if source == target {
            return;
        }

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
        // TODO: expose insertion slot
        // TODO: insert into slot
        // TODO: restore insertion slot

        todo!();
    }

    fn solve_bottom_cross(&mut self) {
        for target_edge in self.cube.bottom_cross_edges() {
            let cur_pos = self.cube.find_edge(target_edge);
            let target_pos = Cube::SOLVED.find_edge(target_edge);
            self.move_bottom_cross_edge(cur_pos, target_pos);
        }

        todo!();
    }
}

pub fn solve(cube: Cube) -> Vec<Move> {
    let mut solver = Solver {
        cube,
        move_stack: Vec::new(),
    };
    // solver.solve_bottom_cross();
    println!("{}", solver.cube);
    solver.make_move(Move::F);
    println!("{}", solver.cube);

    solver.move_stack
}
