use rand::seq::SliceRandom;
use rand::thread_rng;

struct Game {
    players: [u32; 2],
    turn: u8,
    size: u8,
    wins: Vec<u32>,
    total_evaluations: u32,
}

impl Game {
    fn new() -> Self {
        let mut g = Game {
            players: [0, 0],
            turn: 0,
            size: 3,
            wins: vec![],
            total_evaluations: 0,
        };
        g.init_win_mask();
        g
    }
    // Generate masks for win conditions
    fn init_win_mask(&mut self) {
        // Horizontals
        let mut mask: u32 = (1 << self.size) - 1;
        for _ in 0..self.size {
            self.wins.push(mask);
            mask <<= self.size;
        }
        // Verticals
        let mut mask: u32 = 0;
        for _ in 0..self.size {
            mask = (mask << self.size) | 1
        }
        for _ in 0..self.size {
            self.wins.push(mask);
            mask <<= 1
        }
        // Diagonals
        let mut mask: u32 = 0;
        for _ in 0..self.size {
            mask = (mask << (self.size + 1)) | 1
        }
        self.wins.push(mask);
        let mut mask: u32 = 0;
        for _ in 0..self.size {
            mask = (mask << (self.size - 1)) | 1;
        }
        self.wins.push(mask << (self.size - 1))
    }

    // Make a move and changes player
    fn make_move(&mut self, square: u32) {
        let mask = 1 << square;
        self.players[self.turn as usize] ^= mask;
        self.turn = 1 - self.turn;
    }

    // Reverse a move and changes player
    fn undo_move(&mut self, square: u32) {
        let mask = 1 << square;
        self.turn = 1 - self.turn;
        self.players[self.turn as usize] ^= mask
    }

    // Compute possible next moves
    fn moves(&self) -> Vec<u32> {
        let mut moves = vec![];
        let board = self.players[0] | self.players[1];
        for square in 0..self.size.pow(2) {
            if board & (1 << square) == 0 {
                moves.push(square.into())
            }
        }
        moves
    }

    // Check if game was won by any of the players
    fn is_won(&self) -> bool {
        let x = self.players[(1 - self.turn) as usize];
        self.wins.iter().any(|mask| x & mask == *mask)
    }

    // Check if no more move is possible
    fn is_full(&self) -> bool {
        let full = (1 << (self.size * self.size)) - 1;
        self.players[0] | self.players[1] == full
    }

    // Check game over, either by full, win or both
    fn is_over(&self) -> bool {
        self.is_full() | self.is_won()
    }

    // Compute number of free lines and occupancy for player
    fn threats(&self, turn: u8) -> f32 {
        let player = self.players[turn as usize];
        let opponent = self.players[(1 - turn) as usize];
        let threats: u32 = self
            .wins
            .iter()
            .filter(|mask| opponent & **mask == 0)
            .map(|mask| (player & mask).count_ones().pow(2))
            .sum();
        threats as f32
    }

    // Score heuristic based on both sides threats
    fn heuristic(&self) -> f32 {
        self.threats(self.turn) - self.threats(1 - self.turn)
    }

    // Return best move according to minimax
    fn best_move(&mut self, alpha: f32, beta: f32, depth: u8) -> u32 {
        self.negamax(alpha, beta, depth).0
    }

    // Play randomly
    fn random_move(&mut self) -> u32 {
        let mut rng = thread_rng();
        self.moves()
            .choose(&mut rng)
            .expect("Can't chose from 0 moves")
            .clone()
    }

    // Evaluate positions according to the negamax algorithm
    fn negamax(&mut self, mut alpha: f32, beta: f32, depth: u8) -> (u32, f32) {
        if self.is_won() {
            return (0u32, -f32::INFINITY);
        } else if self.is_full() {
            return (0u32, 0.0f32);
        } else if depth == 0 {
            return (0u32, self.heuristic());
        }
        let mut best_moves = vec![];

        let mut value = -f32::INFINITY;
        let mut best_value = -f32::INFINITY;
        for square in self.moves() {
            self.total_evaluations += 1;
            self.make_move(square);
            let score = -self.negamax(-beta, -alpha, depth - 1).1;
            value = value.max(score);
            self.undo_move(square);
            if score == best_value {
                best_moves.push(square);
            } else if score > best_value {
                best_value = score;
                best_moves = vec![square];
                if score > beta {
                    break;
                }
            }
            alpha = alpha.max(score);
        }
        let mut rng = thread_rng();
        (
            best_moves
                .choose(&mut rng)
                .expect("Can't chose from 0 moves")
                .clone(),
            value,
        )
    }
}

fn main() {
    let mut results = [0, 0, 0];
    let mut eval_total = 0;
    let n_games = 100;
    for _ in 0..n_games {
        let mut game = Game::new();
        while !game.is_over() {
            let next_move = if game.turn == 0 {
                game.best_move(-f32::INFINITY, f32::INFINITY, 6)
            } else {
                game.best_move(-f32::INFINITY, f32::INFINITY, 6)
            };
            game.make_move(next_move);
        }
        eval_total += game.total_evaluations;
        if game.is_won() {
            if game.turn == 0 {
                results[1] += 1;
            } else {
                results[0] += 1;
            }
        } else {
            results[2] += 1
        }
    }
    println!("{:?}", results);
    println!("Total evaluations per game: {:?}", eval_total / n_games);
}
