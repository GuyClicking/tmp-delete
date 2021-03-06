extern crate cube;

use cube::Corner::*;
use cube::Edge::*;
use cube::*;

#[test]
fn solved_cube() {
  let cube = Cube::solved();
  let solved = Cube::new(
    [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
    [0; NUM_CORNERS],
    [UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR],
    [0; NUM_EDGES],
  );
  assert_eq!(solved, cube);
}

#[test]
fn invalid_cube() {
  let invalid_ep = Cube::new_unchecked(
    [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
    [0; NUM_CORNERS],
    [UF; 12],
    [0; NUM_EDGES],
  );
  assert_eq!(CubeStateErr::ErrEP, invalid_ep.verify().unwrap_err());

  let invalid_eo = Cube::new_unchecked(
    [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
    [0; NUM_CORNERS],
    [UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR],
    [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  );
  assert_eq!(CubeStateErr::ErrEO, invalid_eo.verify().unwrap_err());

  let invalid_eo = Cube::new_unchecked(
    [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
    [0; NUM_CORNERS],
    [UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  );
  assert_eq!(CubeStateErr::ErrEO, invalid_eo.verify().unwrap_err());

  let invalid_cp = Cube::new_unchecked(
    [URF; 8],
    [0; NUM_CORNERS],
    [UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR],
    [0; NUM_EDGES],
  );
  assert_eq!(CubeStateErr::ErrCP, invalid_cp.verify().unwrap_err());

  let invalid_co = Cube::new_unchecked(
    [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR],
    [0; NUM_EDGES],
  );
  assert_eq!(CubeStateErr::ErrCO, invalid_co.verify().unwrap_err());

  let invalid_co = Cube::new_unchecked(
    [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
    [3, 0, 0, 0, 0, 0, 0, 0],
    [UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR],
    [0; NUM_EDGES],
  );
  assert_eq!(CubeStateErr::ErrCO, invalid_co.verify().unwrap_err());

  let invalid_edge_parity = Cube::new_unchecked(
    [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
    [0; NUM_CORNERS],
    [UF, UR, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR],
    [0; NUM_EDGES],
  );
  assert_eq!(
    CubeStateErr::ErrParity,
    invalid_edge_parity.verify().unwrap_err()
  );

  let invalid_corner_parity = Cube::new_unchecked(
    [UFL, URF, ULB, UBR, DFR, DLF, DBL, DRB],
    [0; NUM_CORNERS],
    [UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR],
    [0; NUM_EDGES],
  );
  assert_eq!(
    CubeStateErr::ErrParity,
    invalid_corner_parity.verify().unwrap_err()
  );
}

#[test]
fn test_has_valid_parity() {
  let cube = Cube::new_unchecked(
    [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
    [0; NUM_CORNERS],
    [UF, UR, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR],
    [0; NUM_EDGES],
  );
  assert_eq!(CubeStateErr::ErrParity, cube.verify().unwrap_err());
  assert!(!cube.has_valid_parity());
}

#[test]
fn move_u() {
  let cube = Cube::solved();
  let cube = cube.apply_move(Move(Face::U, 1));
  let move_u = Cube::new(
    [UBR, URF, UFL, ULB, DFR, DLF, DBL, DRB],
    [0; NUM_CORNERS],
    [UB, UR, UF, UL, DR, DF, DL, DB, FR, FL, BL, BR],
    [0; NUM_EDGES],
  );
  assert_eq!(move_u, cube);
}

#[test]
fn move_r() {
  let cube = Cube::solved();
  let cube = cube.apply_move(Move(Face::R, 1));
  let move_r = Cube::new(
    [DFR, UFL, ULB, URF, DRB, DLF, DBL, UBR],
    [1, 0, 0, 2, 2, 0, 0, 1],
    [FR, UF, UL, UB, BR, DF, DL, DB, DR, FL, BL, UR],
    [0; NUM_EDGES],
  );
  assert_eq!(move_r, cube);
}

#[test]
fn move_f() {
  let cube = Cube::solved();
  let cube = cube.apply_move(Move(Face::F, 1));
  let move_f = Cube::new(
    [UFL, DLF, ULB, UBR, URF, DFR, DBL, DRB],
    [2, 1, 0, 0, 1, 2, 0, 0],
    [UR, FL, UL, UB, DR, FR, DL, DB, UF, DF, BL, BR],
    [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0],
  );
  assert_eq!(move_f, cube);
}

#[test]
fn move_d() {
  let cube = Cube::solved();
  let cube = cube.apply_move(Move(Face::D, 1));
  let move_d = Cube::new(
    [URF, UFL, ULB, UBR, DLF, DBL, DRB, DFR],
    [0; NUM_CORNERS],
    [UR, UF, UL, UB, DF, DL, DB, DR, FR, FL, BL, BR],
    [0; NUM_EDGES],
  );
  assert_eq!(move_d, cube);
}

#[test]
fn move_b() {
  let cube = Cube::solved();
  let cube = cube.apply_move(Move(Face::B, 1));
  let move_b = Cube::new(
    [URF, UFL, UBR, DRB, DFR, DLF, ULB, DBL],
    [0, 0, 2, 1, 0, 0, 1, 2],
    [UR, UF, UL, BR, DR, DF, DL, BL, FR, FL, UB, DB],
    [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1],
  );
  assert_eq!(move_b, cube);
}

#[test]
fn move_l() {
  let cube = Cube::solved();
  let cube = cube.apply_move(Move(Face::L, 1));
  let move_l = Cube::new(
    [URF, ULB, DBL, UBR, DFR, UFL, DLF, DRB],
    [0, 2, 1, 0, 0, 1, 2, 0],
    [UR, UF, BL, UB, DR, DF, FL, DB, FR, UL, DL, BR],
    [0; NUM_EDGES],
  );
  assert_eq!(move_l, cube);
}

#[test]
fn move_u2() {
  let cube = Cube::solved();
  let cube = cube.apply_move(Move(Face::U, 2));
  let move_u2 = Cube::new(
    [ULB, UBR, URF, UFL, DFR, DLF, DBL, DRB],
    [0; NUM_CORNERS],
    [UL, UB, UR, UF, DR, DF, DL, DB, FR, FL, BL, BR],
    [0; NUM_EDGES],
  );
  assert_eq!(move_u2, cube);
}

#[test]
fn move_u_prime() {
  let cube = Cube::solved();
  let cube = cube.apply_move(Move(Face::U, 3));
  let move_u_prime = Cube::new(
    [UFL, ULB, UBR, URF, DFR, DLF, DBL, DRB],
    [0; NUM_CORNERS],
    [UF, UL, UB, UR, DR, DF, DL, DB, FR, FL, BL, BR],
    [0; NUM_EDGES],
  );
  assert_eq!(move_u_prime, cube);
}

#[test]
fn alg_u2r2() {
  let mut cube = Cube::solved();
  for _ in 0..3 {
    cube = cube.apply_move(Move(Face::U, 2));
    cube = cube.apply_move(Move(Face::R, 2));
  }
  let cube_u2r2 = Cube::new(
    [URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB],
    [0; NUM_CORNERS],
    [UR, UB, UL, UF, DR, DF, DL, DB, BR, FL, BL, FR],
    [0; NUM_EDGES],
  );
  assert_eq!(cube_u2r2, cube);
}

#[test]
fn alg_tperm() {
  let tperm = [
    Move(Face::R, 1),
    Move(Face::U, 1),
    Move(Face::R, 3),
    Move(Face::U, 3),
    Move(Face::R, 3),
    Move(Face::F, 1),
    Move(Face::R, 2),
    Move(Face::U, 3),
    Move(Face::R, 3),
    Move(Face::U, 3),
    Move(Face::R, 1),
    Move(Face::U, 1),
    Move(Face::R, 3),
    Move(Face::F, 3),
  ];
  let mut cube = Cube::solved();
  for m in &tperm {
    cube = cube.apply_move(*m);
  }
  let cube_tperm = Cube::new(
    [UBR, UFL, ULB, URF, DFR, DLF, DBL, DRB],
    [0; NUM_CORNERS],
    [UL, UF, UR, UB, DR, DF, DL, DB, FR, FL, BL, BR],
    [0; NUM_EDGES],
  );
  assert_eq!(cube_tperm, cube);
}

#[test]
fn opposite_face() {
  let f = Face::F;
  assert!(f.is_opposite(Face::B));
  assert!(!f.is_opposite(Face::U));

  let u = Face::U;
  assert!(u.is_opposite(Face::D));
  assert!(!u.is_opposite(Face::R));

  let r = Face::R;
  assert!(r.is_opposite(Face::L));
  assert!(!r.is_opposite(Face::F));
}
