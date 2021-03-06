/// The faces on a 3x3x3 cube.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Face {
  U,
  R,
  F,
  D,
  B,
  L,
}

#[derive(PartialEq)]
enum Slice {
  E,
  M,
  S,
}

impl Face {
  fn slice(&self) -> Slice {
    match self {
      Face::U => Slice::E,
      Face::D => Slice::E,

      Face::R => Slice::M,
      Face::L => Slice::M,

      Face::F => Slice::S,
      Face::B => Slice::S,
    }
  }
  /// Test if `face` is an opposite `Face`.
  pub fn is_opposite(&self, face: Face) -> bool {
    self.slice() == face.slice()
  }
}

impl From<Face> for usize {
  fn from(val: Face) -> usize {
    match val {
      Face::U => 0,
      Face::R => 1,
      Face::F => 2,
      Face::D => 3,
      Face::B => 4,
      Face::L => 5,
    }
  }
}

/// A move on a 3x3x3 cube.
#[derive(Clone, Copy, Debug)]
pub struct Move(pub Face, pub u8);

/// The permutations and orientations representing a move.
struct MovePerm {
  cp: &'static [usize; NUM_CORNERS],
  co: &'static [u8; NUM_CORNERS],
  ep: &'static [usize; NUM_EDGES],
  eo: &'static [u8; NUM_EDGES],
}

/// An array containing the 6 basic moves on a 3x3x3.
const MOVE_PERMS: [MovePerm; 6] = [
  MOVE_PERM_U,
  MOVE_PERM_R,
  MOVE_PERM_F,
  MOVE_PERM_D,
  MOVE_PERM_B,
  MOVE_PERM_L,
];

const MOVE_PERM_U: MovePerm = MovePerm {
  cp: &[3, 0, 1, 2, 4, 5, 6, 7],
  co: &[0; 8],
  ep: &[3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11],
  eo: &[0; 12],
};

const MOVE_PERM_R: MovePerm = MovePerm {
  cp: &[4, 1, 2, 0, 7, 5, 6, 3],
  co: &[2, 0, 0, 1, 1, 0, 0, 2],
  ep: &[8, 1, 2, 3, 11, 5, 6, 7, 4, 9, 10, 0],
  eo: &[0; 12],
};

const MOVE_PERM_F: MovePerm = MovePerm {
  cp: &[1, 5, 2, 3, 0, 4, 6, 7],
  co: &[1, 2, 0, 0, 2, 1, 0, 0],
  ep: &[0, 9, 2, 3, 4, 8, 6, 7, 1, 5, 10, 11],
  eo: &[0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0],
};

const MOVE_PERM_D: MovePerm = MovePerm {
  cp: &[0, 1, 2, 3, 5, 6, 7, 4],
  co: &[0; 8],
  ep: &[0, 1, 2, 3, 5, 6, 7, 4, 8, 9, 10, 11],
  eo: &[0; 12],
};

const MOVE_PERM_B: MovePerm = MovePerm {
  cp: &[0, 1, 3, 7, 4, 5, 2, 6],
  co: &[0, 0, 1, 2, 0, 0, 2, 1],
  ep: &[0, 1, 2, 11, 4, 5, 6, 10, 8, 9, 3, 7],
  eo: &[0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1],
};

const MOVE_PERM_L: MovePerm = MovePerm {
  cp: &[0, 2, 6, 3, 4, 1, 5, 7],
  co: &[0, 1, 2, 0, 0, 2, 1, 0],
  ep: &[0, 1, 10, 3, 4, 5, 9, 7, 8, 2, 6, 11],
  eo: &[0; 12],
};

/// The corners on a 3x3x3 cube.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Corner {
  URF,
  UFL,
  ULB,
  UBR,
  DFR,
  DLF,
  DBL,
  DRB,
}

impl From<usize> for Corner {
  fn from(val: usize) -> Corner {
    match val {
      0 => Corner::URF,
      1 => Corner::UFL,
      2 => Corner::ULB,
      3 => Corner::UBR,
      4 => Corner::DFR,
      5 => Corner::DLF,
      6 => Corner::DBL,
      7 => Corner::DRB,
      _ => panic!("Invalid corner!"),
    }
  }
}

/// The edges on a 3x3x3 cube.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Edge {
  UR,
  UF,
  UL,
  UB,
  DR,
  DF,
  DL,
  DB,
  FR,
  FL,
  BL,
  BR,
}

impl From<usize> for Edge {
  fn from(val: usize) -> Edge {
    match val {
      0 => Edge::UR,
      1 => Edge::UF,
      2 => Edge::UL,
      3 => Edge::UB,
      4 => Edge::DR,
      5 => Edge::DF,
      6 => Edge::DL,
      7 => Edge::DB,
      8 => Edge::FR,
      9 => Edge::FL,
      10 => Edge::BL,
      11 => Edge::BR,
      _ => panic!("Invalid edge!"),
    }
  }
}

/// An error for the different invalid cube states.
#[derive(Debug, PartialEq)]
pub enum CubeStateErr {
  ErrEO,
  ErrCO,
  ErrEP,
  ErrCP,
  ErrParity,
}

/// Number of corners on a 3x3x3 cube.
pub const NUM_CORNERS: usize = 8;
/// Number of edges on a 3x3x3 cube.
pub const NUM_EDGES: usize = 12;

/// Models a 3x3x3 cube, separating permutation and orientation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cube {
  pub cp: [Corner; NUM_CORNERS],
  pub co: [u8; NUM_CORNERS],
  pub ep: [Edge; NUM_EDGES],
  pub eo: [u8; NUM_EDGES],
}

impl Cube {
  /// Creates a new `Cube` with the specified permutations and orientations.
  pub fn new(
    cp: [Corner; NUM_CORNERS],
    co: [u8; NUM_CORNERS],
    ep: [Edge; NUM_EDGES],
    eo: [u8; NUM_EDGES],
  ) -> Cube {
    let cube = Cube { cp, co, ep, eo };
    cube.verify().unwrap();
    cube
  }

  /// Creates a new `Cube` with the specified permutations and orientations.
  /// This function does not check that the `Cube` is in a solvable state.
  pub fn new_unchecked(
    cp: [Corner; NUM_CORNERS],
    co: [u8; NUM_CORNERS],
    ep: [Edge; NUM_EDGES],
    eo: [u8; NUM_EDGES],
  ) -> Cube {
    Cube { cp, co, ep, eo }
  }

  /// Creates a new `Cube` in the solved state.
  pub fn solved() -> Cube {
    let cp = [
      Corner::URF,
      Corner::UFL,
      Corner::ULB,
      Corner::UBR,
      Corner::DFR,
      Corner::DLF,
      Corner::DBL,
      Corner::DRB,
    ];
    let co = [0; NUM_CORNERS];
    let ep = [
      Edge::UR,
      Edge::UF,
      Edge::UL,
      Edge::UB,
      Edge::DR,
      Edge::DF,
      Edge::DL,
      Edge::DB,
      Edge::FR,
      Edge::FL,
      Edge::BL,
      Edge::BR,
    ];
    let eo = [0; NUM_EDGES];
    Cube::new(cp, co, ep, eo)
  }

  /// Return a new `Cube` after applying `Move` to the current `Cube`.
  pub fn apply_move(&self, move_: Move) -> Cube {
    assert!(move_.1 > 0 && move_.1 < 4);
    let mp = &MOVE_PERMS[usize::from(move_.0)];
    let new = (0..move_.1).fold(*self, |acc, _| acc.apply_move_perm(mp));
    debug_assert!(new.verify().is_ok());
    new
  }

  /// Return a new `Cube` after applying `MovePerm` to the current `Cube`.
  fn apply_move_perm(&self, move_perm: &MovePerm) -> Cube {
    let mut cp = [Corner::URF; NUM_CORNERS];
    let mut co = [0; NUM_CORNERS];
    let mut ep = [Edge::UR; NUM_EDGES];
    let mut eo = [0; NUM_EDGES];

    for (i, &j) in move_perm.cp.iter().enumerate() {
      cp[i] = self.cp[j];
      co[i] = (self.co[j] + move_perm.co[j]) % 3;
    }

    for (i, &j) in move_perm.ep.iter().enumerate() {
      ep[i] = self.ep[j];
      eo[i] = self.eo[j] ^ move_perm.eo[i];
    }

    Cube { cp, co, ep, eo }
  }

  /// Verify that a `Cube` is in a solvable state.
  pub fn verify(&self) -> Result<(), CubeStateErr> {
    // Check that each edge is used only once.
    let mut edges = 0u16;
    for i in &self.ep {
      edges |= 1 << (*i as u16);
    }
    if edges != 0b111111111111 {
      return Err(CubeStateErr::ErrEP);
    }

    // Check that each edge orientation is 0 or 1.
    if !self.eo.iter().all(|&eo| eo <= 1) {
      return Err(CubeStateErr::ErrEO);
    }

    // Check that the total edge orientation is a multiple of 2.
    let eo: u8 = self.eo.iter().sum();
    if eo % 2 != 0 {
      return Err(CubeStateErr::ErrEO);
    }

    // Check that each corner is used only once.
    let mut corners = 0u8;
    for i in &self.cp {
      corners |= 1 << (*i as u8);
    }
    if corners != 0b11111111 {
      return Err(CubeStateErr::ErrCP);
    }

    // Check that each edge orientation is 0, 1 or 2.
    if !self.co.iter().all(|&co| co <= 2) {
      return Err(CubeStateErr::ErrCO);
    }

    // Check that the total corner orientation is a multiple of 3.
    let co: u8 = self.co.iter().sum();
    if co % 3 != 0 {
      return Err(CubeStateErr::ErrCO);
    }

    // Check that corner parity and edge parity are equal.
    if !self.has_valid_parity() {
      return Err(CubeStateErr::ErrParity);
    }
    Ok(())
  }

  fn corner_parity(&self) -> bool {
    num_inversions(&self.cp) % 2 != 0
  }

  fn edge_parity(&self) -> bool {
    num_inversions(&self.ep) % 2 != 0
  }

  /// Check if a `Cube` has valid parity.
  pub fn has_valid_parity(&self) -> bool {
    self.edge_parity() == self.corner_parity()
  }
}

/// Count the number of inversions in a permutation.
fn num_inversions<P: PartialOrd>(perm: &[P]) -> usize {
  let mut num = 0;
  for i in 0..perm.len() - 1 {
    for j in i + 1..perm.len() {
      assert!(i < j);
      if perm[i] > perm[j] {
        num += 1;
      }
    }
  }
  num
}
