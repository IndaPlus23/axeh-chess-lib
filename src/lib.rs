use core::num;
use std::fmt;



#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState { //Olika faser som spelet kan befinna sig i
    InProgress,
    Check,
    GameOver,
    Checkmate
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colour{ //De olika färger som spelaren kan köra som samt de färger som en pjäs kan vara
    Black, White
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Roles{ //Olika roller som en pjäs kan ha
    King, Queen, Knight, Bishop, Rook, Pawn
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece{ //en pjäs har en roll och en färg
    role: Roles,
    colour: Colour,
}

impl Piece {
    fn new(role: Roles, colour: Colour) -> Piece{
        Piece{
            role,
            colour
        }
    }
    
}
/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 * - Read the Rust documentation, ask questions if you get stuck!
 */
pub struct Game { //Spelet befinner sig i olika faser där det alltid är en färgs tur. Spelet representeras av ett bräde
    /* save board, active colour, ... */
    state: GameState,
    activeColour: Colour,
    board: [Option<Piece>; 64],
    //...
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            activeColour: Colour::White,
            board: {let mut array = [None; 64]; //skapar brädet med pjäser på rätt plats
                array[0] = Some(Piece::new(Roles::Rook, Colour::Black));
                array[1] = Some(Piece::new(Roles::Knight, Colour::Black));
                array[2] = Some(Piece::new(Roles::Bishop, Colour::Black)); 
                array[3] = Some(Piece::new(Roles::Queen, Colour::Black));
                array[4] = Some(Piece::new(Roles::King, Colour::Black));
                array[5] = Some(Piece::new(Roles::Bishop, Colour::Black));
                array[6] = Some(Piece::new(Roles::Knight, Colour::Black));
                array[7] = Some(Piece::new(Roles::Rook, Colour::Black));
                for i in 8..16{
                    array[i] = Some(Piece::new(Roles::Pawn, Colour::Black));
                }
                for i in 48..56{
                    array[i] =Some(Piece::new(Roles::Pawn, Colour::White));
                }
                array[56] = Some(Piece::new(Roles::Rook, Colour::White));
                array[57] = Some(Piece::new(Roles::Knight, Colour::White));
                array[58] = Some(Piece::new(Roles::Bishop, Colour::White));
                array[59] = Some(Piece::new(Roles::Queen, Colour::White));
                array[60] = Some(Piece::new(Roles::King, Colour::White));
                array[61] = Some(Piece::new(Roles::Bishop, Colour::White));
                array[62] = Some(Piece::new(Roles::Knight, Colour::White));
                array[63] = Some(Piece::new(Roles::Rook, Colour::White));
                array
            }
        }
    }

    pub fn chess_position_to_number(chess_position: &str) -> usize {
        let filePlaceholder = chess_position.chars().nth(0);
        let rankPlaceholder = chess_position.chars().nth(1);
    
        let file = match filePlaceholder {
            Some(c) => c as char,
            None => 'a'
        };
        let rank = match rankPlaceholder {
            Some(c) => c as char,
            None => '1'
        };
        let file_num = match file {
            'a'..='h' => file as u8 - 'a' as u8,
            _ => 0
        };
    
        let rank_num = match rank {
            '1'..='8' => '8' as u8 - rank as u8,
            _ => 99
        };
    
        let numerical_position:usize = (rank_num * 8 + file_num) as usize;
    
        numerical_position
    }

    /// If the current game state is `InProgress` and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        let from = Self::chess_position_to_number(_from);
        let to = Self::chess_position_to_number(_to);

        let movedPiece = self.board[from];

        self.board[from] = None;

        self.board[to] = movedPiece;
        
        Some(GameState::InProgress)
    }

    /// (Optional but recommended) Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _piece: &str) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Implement en passant and castling.
    pub fn get_possible_moves(&self, _postion: &str) -> Option<Vec<String>> {
        None
    }
}

/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|h
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        let mut result: Vec<&str> = Vec::with_capacity(64);
        result.push("\n /-----------------\\ \n |");
        for i in 0..64{
            if let Some(Piece) = &self.board[i]{
                match Piece.colour {
                    Colour::Black => match Piece.role {
                        Roles::Rook => result.push("\u{2656}"),
                        Roles::Knight => result.push("\u{2658}"),
                        Roles::Bishop => result.push("\u{2657}"),
                        Roles::King => result.push("\u{2654}"),
                        Roles::Queen => result.push("\u{2655}"),
                        Roles::Pawn => result.push("\u{2659}"),
                    }
                    Colour::White => match Piece.role {
                        Roles::Rook => result.push("\u{265C}"),
                        Roles::Knight => result.push("\u{265E}"),
                        Roles::Bishop => result.push("\u{265D}"),
                        Roles::King => result.push("\u{265A}"),
                        Roles::Queen => result.push("\u{265B}"),
                        Roles::Pawn => result.push("\u{265F}"),
                        
                    }
                }
            }
            else{
                result.push("*");
            }
            if i == 7 || i == 15 || i == 23 || i == 31 || i== 39 || i == 47 || i == 55{
                result.push("| \n |");
            }
        }
        result.push("|");
        result.push("\n \\-----------------/");
        let resultString = result
            .iter()
            .map(|&x| x.to_string()) 
            .collect::<Vec<String>>() 
            .join(" ");


        write!(f, "{}", resultString)
        
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {

        let mut game = Game::new();

        println!("{:?}", game);

        game.make_move("a7", "h1");

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}