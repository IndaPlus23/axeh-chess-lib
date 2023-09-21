use std::fmt;
use crate::Pieces::Rook;
use crate::Pieces::Bishop;
use crate::Pieces::Knight;
use crate::Pieces::King;
use crate::Pieces::Queen;
use crate::Pieces::Pawn;
use crate::Colour::Black;
use crate::Colour::White;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState { //Olika faser som spelet kan befinna sig i
    InProgress,
    Check,
    GameOver,
    Checkmate
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colour{ //De olika färger som spelaren kan köra som samt de färger som en pjäs kan vara
    Black,
    White
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Pieces{ //Olika roller som en pjäs kan ha
    King,
    Queen,
    Knight,
    Bishop,
    Rook,
    Pawn
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece{ //en pjäs har en roll och en färg
    role: Pieces,
    colour: Colour
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
                array[0] = Some(Piece {role: Rook, colour: Black});
                array[1] = Some(Piece {role: Knight, colour: Black});
                array[2] = Some(Piece {role: Bishop, colour: Black});
                array[3] = Some(Piece {role: Queen, colour: Black});
                array[4] = Some(Piece {role: King, colour: Black});
                array[5] = Some(Piece {role: Bishop, colour: Black});
                array[6] = Some(Piece {role: Knight, colour: Black});
                array[7] = Some(Piece {role: Rook, colour: Black});
                for i in 8..16{
                    array[i] = Some(Piece {role: Pawn, colour: Black});
                }
                for i in 48..56{
                    array[i] = Some(Piece {role: Pawn, colour: White});
                }
                array[56] = Some(Piece {role: Rook, colour: White});
                array[57] = Some(Piece {role: Knight, colour: White});
                array[58] = Some(Piece {role: Bishop, colour: White});
                array[59] = Some(Piece {role: Queen, colour: White});
                array[60] = Some(Piece {role: King, colour: White});
                array[61] = Some(Piece {role: Bishop, colour: White});
                array[62] = Some(Piece {role: Knight, colour: White});
                array[63] = Some(Piece {role: Rook, colour: White});
                array
            }
            //...
        }
    }

    /// If the current game state is `InProgress` and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        None
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
            match self.board[i] {
                Some(Piece {role: Rook, colour: Black}) => result.push("\u{2656}"),
                Some(Piece {role: Knight, colour: Black}) => result.push("\u{2658}"),
                Some(Piece {role: Bishop, colour: Black}) => result.push("\u{2657}"),
                Some(Piece {role: King, colour: Black}) => result.push("\u{2654}"),
                Some(Piece {role: Queen, colour: Black}) => result.push("\u{2655}"),
                Some(Piece {role: Pawn, colour: Black}) => result.push("\u{2659}"),
                Some(Piece {role: Rook, colour: White}) => result.push("\u{265C}"),
                Some(Piece {role: Knight, colour: White}) => result.push("\u{265E}"),
                Some(Piece {role: Bishop, colour: White}) => result.push("\u{265D}"),
                Some(Piece {role: King, colour: White}) => result.push("\u{265A}"),
                Some(Piece {role: Queen, colour: White}) => result.push("\u{265B}"),
                Some(Piece {role: Pawn, colour: White}) => result.push("\u{265F}"),
                None => result.push("*"),
            };
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
        
        //write!(f, "{:?}", self.board)
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

        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}