#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Colour {
    White,
    Red,
    Blue,
    Orange,
    Green,
    Yellow,
}

use std::ops::Index;

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
}

type Pos = u8;

#[derive(Debug)]
pub struct Cube {
    // top, left, front, right, back, bottom
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
    faces: [Face; 6],
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
}

impl Index<FaceName> for Cube {
    type Output = Face;

    fn index(&self, index: FaceName) -> &Self::Output {
        &self.faces[index as usize]
    }
}

struct Solver {
    cube: Cube,
}

impl Solver {
    fn solve_bottom_cross(&mut self) {
        for target_edge in self.cube.bottom_cross_edges() {
            let cur_pos = self.cube.find_edge(target_edge);
            let target_pos = Cube::SOLVED.find_edge(target_edge);
            dbg!(cur_pos, target_pos);
        }
    }
}

pub fn solve(cube: Cube) {
    let mut solver = Solver { cube };
    solver.solve_bottom_cross();
}
