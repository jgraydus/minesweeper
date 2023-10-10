use rand;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::collections::HashMap;

const NUMBER_BOMBS: usize = 50;

#[derive(Debug, Eq, PartialEq)]
pub enum Outcome {
  InProgress,
  Win,
  Lose
}

#[derive(Debug)]
pub struct GameState {
  pub outcome: Outcome,
  pub height: usize,
  pub width: usize,
  bombs: HashSet<(usize,usize)>,
  pub visible_bomb: Option<(usize,usize)>, // when the user clicks a bomb, we'll draw it
  pub revealed_squares: HashSet<(usize,usize)>,
  pub neighboring_bombs: HashMap<(usize,usize),u32>,
}

fn choose_bomb_locations(height: usize, width: usize) -> HashSet<(usize,usize)> {
  let mut rng = rand::thread_rng();
  let mut all_locations = Vec::new();
  for x in 0..width {
    for y in 0..height {
      all_locations.push((x,y));
    }
  }
  all_locations.shuffle(&mut rng);
  all_locations.into_iter().take(NUMBER_BOMBS).collect() 
}

const DELTAS: [(i32,i32); 8] = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];

fn neighbors(
  game_state: &GameState,
  (x, y): (usize, usize)
) -> Vec<(usize,usize)> {
  let mut result = Vec::new();
  for (dx,dy) in DELTAS {
    let x0 = x as i32 + dx;
    let y0 = y as i32 + dy;
    if x0 >= 0 && x0 < game_state.width as i32 && y0 >= 0 && y0 < game_state.height as i32 {
        result.push((x0 as usize, y0 as usize));
    }
  }
  result
}

impl GameState {
  pub fn new(height: usize, width: usize) -> Self {
    Self {
      outcome: Outcome::InProgress,
      height,
      width,
      bombs: choose_bomb_locations(height, width),
      visible_bomb: None,
      revealed_squares: HashSet::new(),
      neighboring_bombs: HashMap::new(),
    }
  }

  pub fn reset(&mut self) {
      self.outcome = Outcome::InProgress;
      self.bombs = choose_bomb_locations(self.height, self.width);
      self.visible_bomb = None;
      self.revealed_squares = HashSet::new();
      self.neighboring_bombs = HashMap::new();
  }

  pub fn click(&mut self, loc: (usize, usize)) {
    if self.bombs.contains(&loc) {
      self.visible_bomb = Some(loc);
      self.outcome = Outcome::Lose;
      return;
    }

    let mut todo = Vec::new();
    let mut visited = HashSet::new();

    todo.push(loc);

    while !todo.is_empty() {
      let next = todo.pop().unwrap();
      self.revealed_squares.insert(next);
      visited.insert(next);

      let ns = neighbors(self, next);

      let mut neighboring_bombs: u32 = 0;
      for n in &ns {
        if self.bombs.contains(n) {
          neighboring_bombs = neighboring_bombs + 1;
        }
      }

      self.neighboring_bombs.insert(next, neighboring_bombs);

      if neighboring_bombs == 0 {
        for n in ns {
          if !visited.contains(&n) {
            todo.push(n)
          } 
        }
      }
    }

    // check for win condition`
    if self.revealed_squares.len() == self.height * self.width - NUMBER_BOMBS {
      self.outcome = Outcome::Win;
    }
  }
}

