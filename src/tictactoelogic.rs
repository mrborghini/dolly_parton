use tic_tac_toe_rs::*;

pub struct TicTacToeLogic {
    pub players: Vec<Player>,
    game: DisplayGrid,
}

impl TicTacToeLogic {
    pub fn new() -> TicTacToeLogic {
        let players: Vec<Player> = Vec::new();
        let game = DisplayGrid::new();
        TicTacToeLogic { players, game }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
    pub fn start_tic_tac_toe_game(&self) {
        println!("Game started");
    }
    pub fn show_board(&self) -> String {
        self.game._show_grid_string()
    }
}
