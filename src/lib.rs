#![allow(non_snake_case)]
use core::num;
use std::{fmt, iter::FlatMap, f32::consts::PI};



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
pub struct Piece{ //en pjäs har en roll och en färg och har antingen rört sig eller inte
    role: Roles,
    colour: Colour, 
    hasMoved: bool
}

impl Piece {
    fn new(role: Roles, colour: Colour, hasMoved: bool) -> Piece{
        Piece{
            role,
            colour,
            hasMoved
        }
    }
    fn changeMoved(mut self){
        self.hasMoved = true;
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
                array[0] = Some(Piece::new(Roles::Rook, Colour::Black, false));
                array[1] = Some(Piece::new(Roles::Knight, Colour::Black, false));
                array[2] = Some(Piece::new(Roles::Bishop, Colour::Black, false)); 
                array[3] = Some(Piece::new(Roles::Queen, Colour::Black, false));
                array[4] = Some(Piece::new(Roles::King, Colour::Black, false));
                array[5] = Some(Piece::new(Roles::Bishop, Colour::Black, false));
                array[6] = Some(Piece::new(Roles::Knight, Colour::Black, false));
                array[7] = Some(Piece::new(Roles::Rook, Colour::Black, false));
                for i in 8..16{
                    array[i] = Some(Piece::new(Roles::Pawn, Colour::Black, false));
                }
                for i in 48..56{
                    array[i] =Some(Piece::new(Roles::Pawn, Colour::White, false));
                }
                array[56] = Some(Piece::new(Roles::Rook, Colour::White, false));
                array[57] = Some(Piece::new(Roles::Knight, Colour::White, false));
                array[58] = Some(Piece::new(Roles::Bishop, Colour::White, false));
                array[59] = Some(Piece::new(Roles::Queen, Colour::White, false));
                array[60] = Some(Piece::new(Roles::King, Colour::White, false));
                array[61] = Some(Piece::new(Roles::Bishop, Colour::White, false));
                array[62] = Some(Piece::new(Roles::Knight, Colour::White, false));
                array[63] = Some(Piece::new(Roles::Rook, Colour::White, false));
                array
            }
        }
    }

    pub fn switch_colour(&mut self){
        match self.activeColour {
            Colour::White => self.activeColour = Colour::Black,
            Colour::Black => self.activeColour = Colour::White
        }
    }

    pub fn chessPosToNum(&self, chess_position: &str) -> usize {
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
    pub fn numToChessPos(&self, _postion: usize) -> String{
        let rank = 8 - (_postion / 8);
        let field = match _postion % 8 {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => 'a'
        };
        let chessPos = format!("{}{}", field, rank);
        chessPos   
    }

    pub fn get_legal_moves(&mut self, _postion: &str) -> Option<Vec<String>>{
        let mut moves = self.get_possible_moves(_postion).unwrap();
        let pos = self.chessPosToNum(_postion);

        for i in 0..moves.len(){
            let newPos = self.chessPosToNum(&moves[i]);
            let newS = moves[i].to_string();
            let mut fakeBoard = self.board;
            if let Some(piece) = fakeBoard[pos]{
                fakeBoard[newPos] = Some(piece);
                fakeBoard[pos] = None;
                if self.is_king_in_check(&fakeBoard, self.activeColour){
                    moves.retain(|s| s != &newS)
                }
            }
        }
        Some(moves)
    }

    /// If the current game state is `InProgress` and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        let from = self.chessPosToNum(_from);
        let moves = self.get_legal_moves(_from);
        if let Some(mut piece) = self.board[from]{
            if piece.colour == self.activeColour{
                if moves.as_ref().map(|vec| vec.iter().any(|s| s == _to)).unwrap_or(false){
                    let to = self.chessPosToNum(_to);
                    self.board[to] = Some(piece);
                    self.board[from] = None;
                    piece.changeMoved(); //detta ändrar inte värdet för fuck u
                    self.switch_colour();
                    if self.is_king_in_check(&self.board, self.activeColour){
                        return Some(GameState::Check);
                    }else {
                        return Some(GameState::InProgress);
                    }
                }else {
                    return Some(GameState::InProgress);
                }
            }else {
                return Some(GameState::InProgress);
            }     
        }else {
            return Some(GameState::InProgress);
        }
        
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
    
    pub fn is_king_in_check(&self, board: &[Option<Piece>], king_colour: Colour) -> bool {
        // Find the king's position on the board
        let king_position = board.iter().position(|square| {
            if let Some(piece) = square {
                piece.role == Roles::King && piece.colour == king_colour
            } else {
                false
            }
        });
    
        if let Some(king_position) = king_position {
            let king_x = king_position % 8;
            let king_y = king_position / 8;
    
            let directions = [
                (-1, -1), (-1, 0), (-1, 1),
                (0, -1),           (0, 1),
                (1, -1), (1, 0), (1, 1),
            ];
    
            for (dx, dy) in &directions {
                let mut x = king_x as i32 + dx;
                let mut y = king_y as i32 + dy;
    
                while x >= 0 && x < 8 && y >= 0 && y < 8 {
                    let position = (y * 8 + x) as usize;
                    if let Some(piece) = board[position] {
                        if piece.colour != king_colour {
                            match piece.role {
                                Roles::Queen | Roles::Bishop => return true,
                                _ => break,
                            }
                        } else {
                            break;
                        }
                    }
                    x += dx;
                    y += dy;
                }
            }
    
            // Check for attacks by rooks and queens along horizontal and vertical lines
            let rook_directions = [
                (0, -1), (0, 1), (-1, 0), (1, 0),
            ];
    
            for (dx, dy) in &rook_directions {
                let mut x = king_x as i32 + dx;
                let mut y = king_y as i32 + dy;
    
                while x >= 0 && x < 8 && y >= 0 && y < 8 {
                    let position = (y * 8 + x) as usize;
                    if let Some(piece) = board[position] {
                        if piece.colour != king_colour {
                            match piece.role {
                                Roles::Queen | Roles::Rook => return true,
                                _ => break,
                            }
                        } else {
                            break;
                        }
                    }
                    x += dx;
                    y += dy;
                }
            }
    
            // Check for attacks by knights
            let knight_moves = [
                (-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1),
            ];
    
            for &(dx, dy) in &knight_moves {
                let x = king_x as i32 + dx;
                let y = king_y as i32 + dy;
    
                if x >= 0 && x < 8 && y >= 0 && y < 8 {
                    let position = (y * 8 + x) as usize;
                    if let Some(piece) = board[position] {
                        if piece.colour != king_colour && piece.role == Roles::Knight {
                            return true;
                        }
                    }
                }
            }
    
            // Check for attacks by pawns
            let pawn_moves = if king_colour == Colour::White {
                [(1, -1), (1, 1)]
            } else {
                [(-1, -1), (-1, 1)]
            };
    
            for &(dx, dy) in &pawn_moves {
                let x = king_x as i32 + dx;
                let y = king_y as i32 + dy;
    
                if x >= 0 && x < 8 && y >= 0 && y < 8 {
                    let position = (y * 8 + x) as usize;
                    if let Some(piece) = board[position] {
                        if piece.colour != king_colour && piece.role == Roles::Pawn {
                            return true;
                        }
                    }
                }
            }
        }
    
        false
    }

    pub fn get_rook_moves(&self, piece: Piece, position: usize) -> Vec<String>{
        let mut moves: Vec<String> = Vec::new();
        for i in 1..8{
            let pIndex = position + 8*i;
            if pIndex <= 63 && self.board[pIndex].is_none(){
                moves.push(self.numToChessPos(pIndex))
            } else if  pIndex <= 63 && self.board[pIndex].unwrap().colour != piece.colour{
                moves.push(self.numToChessPos(pIndex));
                break;
            }else if  pIndex <= 63 && self.board[pIndex].unwrap().colour == piece.colour{
                break;
            }
        }
        for i in 1..8{
            let nIndex:i32 = position as i32 - 8*i;
            if nIndex >= 0 && self.board[nIndex as usize].is_none(){
                moves.push(self.numToChessPos(nIndex as usize))
            } else if  nIndex >= 0 && self.board[nIndex as usize].unwrap().colour != piece.colour{
                moves.push(self.numToChessPos(nIndex as usize));
                break;
            }else if  nIndex >= 0 && self.board[nIndex as usize].unwrap().colour == piece.colour{
                break;
            }
        }
        for i in 1..8{
            let rIndex = position + i;
            if rIndex <= 63 && self.board[rIndex].is_none(){
                moves.push(self.numToChessPos(rIndex));
                if (rIndex+1) % 8 == 0{
                    break;
                }
            } else if self.board[rIndex].unwrap().colour != piece.colour{
                moves.push(self.numToChessPos(rIndex));
                break;
            }else if self.board[rIndex].unwrap().colour == piece.colour{
                break;
            }
        }
        for i in 1..8{
            let lIndex:isize = position as isize - i;
            if lIndex >= 0 && lIndex <= 63{
                if self.board[lIndex as usize].is_none(){
                    moves.push(self.numToChessPos(lIndex as usize));
                }
                else if self.board[lIndex as usize].unwrap().colour != piece.colour{
                    moves.push(self.numToChessPos(lIndex as usize));
                    break;
                }else if self.board[lIndex as usize].unwrap().colour == piece.colour{
                    break;
                }
                if lIndex % 8 == 0{
                    break;
                }
            } 
        }
        moves
    }
    
    pub fn get_bishop_moves(&self, piece: Piece, position: usize) -> Vec<String>{
        let mut moves: Vec<String> = Vec::new();
        for i in 1..8{
            let posNeg9:isize = position as isize -9*i;
            if posNeg9 >= 0 {
                let ref newPos = self.board[posNeg9 as usize];
                if newPos.is_none(){
                    moves.push(self.numToChessPos(posNeg9 as usize));
                }
                else if newPos.unwrap().colour != piece.colour {
                    moves.push(self.numToChessPos(posNeg9 as usize));
                    break;
                }
                else if newPos.unwrap().colour == piece.colour {
                    break;
                }
                if posNeg9 % 8 == 0 {
                    break;
                }
            }
        }
        for i in 1..8{
            let posNeg7:isize = position as isize -7*i;
            if posNeg7 >= 0 {
                let ref newPos = self.board[posNeg7 as usize];
                if newPos.is_none(){
                    moves.push(self.numToChessPos(posNeg7 as usize));
                    if (posNeg7+1) % 8 == 0 {
                        break;
                    }
                }
                else if newPos.unwrap().colour != piece.colour {
                    moves.push(self.numToChessPos(posNeg7 as usize));
                    break;
                }
                else if newPos.unwrap().colour == piece.colour {
                    break;
                }
            }
        }
        for i in 1..8{
            let pos9 = position+9*i;
            if pos9 <= 63 {
                let ref newPos = self.board[pos9];
                if newPos.is_none(){
                    moves.push(self.numToChessPos(pos9));
                    if (pos9+1) % 8 == 0 {
                        break;
                    }
                }
                else if self.board[pos9].unwrap().colour != piece.colour {
                    moves.push(self.numToChessPos(pos9));
                    break;
                }
                else if self.board[pos9].unwrap().colour == piece.colour {
                    break;
                }  
            }
        }
        for i in 1..8{
            let pos7 = position+7*i;
            if pos7 >= 0 {
                let ref newPos = self.board[pos7];

                if newPos.is_none(){
                    moves.push(self.numToChessPos(pos7));
                    if pos7 % 8 == 0 {
                        break;
                    }
                }
                else if newPos.unwrap().colour != piece.colour {
                    moves.push(self.numToChessPos(pos7));
                    break;
                }
                else if newPos.unwrap().colour == piece.colour {
                    break;
                }
            }
        } 
        moves
    }

    pub fn get_king_moves(&self, piece: Piece, position: usize) -> Vec<String>{
        let mut moves: Vec<String> = Vec::new();
        if position > 8 && position < 55 && position % 8 != 0 && (position+1) % 8 != 0{ //fall där kung inte nuddar kant
            let list: [i16; 8] = [-9, -8, -7, -1, 1, 7, 8, 9];
            for i in 0..list.len(){
                let index:i16 = position as i16 + list[i];
                if self.board[index as usize].is_none() || self.board[index as usize].unwrap().colour != piece.colour{
                    moves.push(self.numToChessPos(index as usize));
                }
            }
        }else if (position >= 8 || position == 0) && position % 8 == 0{//fall där kung nuddar vänster kant
            let list: [i16; 5] = [-8, -7, 1, 8, 9];
            for i in 0..list.len(){
                let index:i16 = position as i16 + list[i];
                if index >= 0 && index <= 63 && (self.board[index as usize].is_none() || self.board[index as usize].unwrap().colour != piece.colour){
                    moves.push(self.numToChessPos(index as usize));
                }
            }
        }else if (position > 8 || position == 7) && (position+1) % 8 == 0{//fall där kung nuddar vänster kant
            let list: [i16; 5] = [-9, -8, -1, 7, 8];
            for i in 0..list.len(){
                let index:i16 = position as i16 + list[i];
                if index >= 0 && index <= 63 && (self.board[index as usize].is_none() || self.board[index as usize].unwrap().colour != piece.colour){
                    moves.push(self.numToChessPos(index as usize));
                }
            }
        }else{
            let list: [i16; 8] = [-9, -8, -7, -1, 1, 7, 8, 9]; //resterande fall
            for i in 0..list.len(){
                let index:i16 = position as i16 + list[i];
                if index >= 0 && index <= 63 && (self.board[index as usize].is_none() || self.board[index as usize].unwrap().colour != piece.colour){
                    moves.push(self.numToChessPos(index as usize));
                }
            }
        }
        moves
    }

    pub fn get_white_pawn_moves(&self, piece: Piece, position: usize) -> Vec<String>{
        let mut moves:Vec<String> = Vec::new();
        if position >= 8{
            if self.board[position-8].is_none() {
                moves.push(self.numToChessPos(position-8));
            }
        }
        if position >= 9{
            if let Some(enePiece) = self.board[position-9]{
                if enePiece.colour != piece.colour{
                    moves.push(self.numToChessPos(position-9));
                }
            }
        }
        if position > 7{
            if let Some(enePiece) = self.board[position-7]{
                if enePiece.colour != piece.colour{
                    moves.push(self.numToChessPos(position-7));
                }
            }
        }
        if piece.hasMoved == false{
            if self.board[position-(8*2)].is_none() && self.board[position-8].is_none(){
                moves.push(self.numToChessPos(position-(8*2)));
            }
        }
        moves
    }

    pub fn get_black_pawn_moves(&self, piece: Piece, position: usize) -> Vec<String>{
        let mut moves: Vec<String> = Vec::new();
        if (position + 8) <= 63 {
            if self.board[position+8].is_none() {
                moves.push(self.numToChessPos(position+8));
            }
        }
        if (position+9) <= 63{
            if let Some(enePiece) = self.board[position+9]{
                if enePiece.colour != piece.colour{
                    moves.push(self.numToChessPos(position+9));
                }
            }
        }
        if (position+7) < 63{
            if let Some(enePiece) = self.board[position+7]{
                if enePiece.colour != piece.colour{
                    moves.push(self.numToChessPos(position+7));
                }
            }
        }
        if piece.hasMoved == false{
            if self.board[position+(8*2)].is_none() && self.board[position+8].is_none(){
                moves.push(self.numToChessPos(position+(8*2)));
            }
        }
        moves
    }
    
    pub fn get_knight_moves(&self, piece: Piece, position: usize) -> Vec<String>{
        let mut moves: Vec<String> = Vec::new();
        let pos:isize = position as isize;
        let target:Vec<isize> = [-17, -15, -10, -6, 6, 10, 15, 17].to_vec();
        let ref board = self.board;
         
        for i in 0..target.len(){
            let newPos = pos + target[i];
            if newPos <= 63 && newPos >= 0 && ((pos%8)-(newPos%8)).abs() <= 2{
                if board[newPos as usize].is_none() || board[newPos as usize].unwrap().colour != piece.colour{
                    moves.push(self.numToChessPos(newPos as usize));
                } 
            }
        }
        

        moves
    }

    pub fn get_possible_moves(&self, _postion: &str) -> Option<Vec<String>> { //returns all possible moves not all legal moves
        let position = self.chessPosToNum(_postion);
        let mut moves: Vec<String> = Vec::new();

        if let Some(piece) = self.board[position]{
            match piece.role{
                Roles::Pawn => match piece.colour {
                    Colour::White => { 
                        moves = self.get_white_pawn_moves(piece, position);
                    }
                    Colour::Black =>{
                        moves = self.get_black_pawn_moves(piece, position);
                    }
                }
                Roles::King => {
                    moves = self.get_king_moves(piece, position);
                },
                Roles::Rook => {
                    moves = self.get_rook_moves(piece, position);
                },
                Roles::Bishop => {
                    moves = self.get_bishop_moves(piece, position);
                },
                Roles::Queen => {
                    let mut rMoves: Vec<String> = self.get_rook_moves(piece, position);
                    let mut bMoves: Vec<String> = self.get_bishop_moves(piece, position);
                    rMoves.append(&mut bMoves);

                    moves = rMoves;
                },
                Roles::Knight => {
                    moves = self.get_knight_moves(piece, position);
                },
            }
        }
        Some(moves)
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

        assert_eq!(game.make_move("a2", "a4").unwrap(), GameState::InProgress);

        println!("{:?}", game);
        
        //assert_eq!(game.get_possible_moves("a4").unwrap(), ["a5"]); //detta ger fel då hasMoved inte ändras så den är alltid false
        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}