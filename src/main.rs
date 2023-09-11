use std::{thread::current, 
    default, 
    collections::LinkedList,
    fs::File,
    io::{self, BufRead},
    path::Path, fmt
};

use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::{JsCast, JsValue};

fn main() {
    println!("Hello, world!");
    let mut game: ConnectFourGameV2 = ConnectFourGameV2::create();

    
    println!("{}",game);
    game.make_move(2);
    println!("{}",game);
    game.make_move(3);
    println!("{}",game);
    game.undo_move();
    println!("{}",game);

    
    // let mut run: bool = true;
    // while run {
    //     println!("{}",game.to_string());
    //     let mut input: String = String::new();
    //     println!("Your move: ");
    //     io::stdin().read_line(&mut input).expect("error");
    //     input = input.trim().to_string();
    //     if input.eq("end") {
    //         run = false;
    //     } else {
    //         game.make_move(input.parse::<u32>().unwrap());
    //         if game.check_p1_wins() {println!("You won!"); break;}
    //         let best_move: u32 = ConnectFourSolver::best_move(&mut game);
    //         game.make_move(best_move);
    //         if game.check_p2_wins() {println!("You lost..."); break;}
    //     }
    // }
    // println!("ended!");
}


struct ConnectFourGameV2{
    p1_positions: u64,
    p2_positions: u64,
    prev_positions: Vec<u64> // replace this with a transposition table
}
impl ConnectFourGameV2{
    fn create() -> ConnectFourGameV2{
        return Self{p1_positions: 0, p2_positions: 0, prev_positions: Vec::new()}
    }
    fn make_move(&mut self, column: u32) -> bool{
        if !self.can_play_move(column) {return false}
        if self.is_p1s_turn(){
            //self.p1_positions = self.p1_positions|((Self::col_mask(column) & self.p1_positions)<<7);
            println!("{}",column);
            println!("{:?}",to_base_2_rev(Self::row_mask(0)));
            println!("{}",Self::col_mask(2)&Self::row_mask(0));
            println!("{:?}",to_base_2_rev(1028));
            self.p1_positions += Self::col_mask(column)&Self::row_mask(self.col_height(column));
            println!("{}",self.p1_positions);
            self.prev_positions.push(self.p1_positions);
        }else{
            self.p2_positions += Self::col_mask(column)&Self::row_mask(self.col_height(column));
            self.prev_positions.push(self.p2_positions);
        }
        return true;
    }
    fn can_play_move(&self, column: u32) -> bool{
        return column<7 && !(self.non_empty_tiles() & Self::col_mask(column) > 1099511627775)
    }
    fn non_empty_tiles(&self) -> u64{
        return self.p1_positions|self.p2_positions;
    }
    fn row_mask(row: u32) -> u64{
        return 127<<(8*row)
    }
    fn col_mask(col: u32) -> u64{
        return 1103823438081<<col
    }
    fn col_height(&self, col: u32) -> u32{
        return (self.non_empty_tiles()&Self::col_mask(col)).count_ones();
    }
    fn undo_move(&mut self){
        if self.prev_positions.len()>0{
            let prev_position: u64 = self.prev_positions.pop().unwrap();
            if self.is_p1s_turn(){
                self.p1_positions=prev_position;
            }else{
                self.p2_positions=prev_position;
            }
        }
    }
    fn is_p1s_turn(&self) -> bool{
        return self.prev_positions.len()%2==0 
    }
    fn check_p1_wins(&self) -> bool{
        return (self.p1_positions & self.p1_positions<<1 & self.p1_positions<<2 & self.p1_positions<<3)>0||
        (self.p1_positions & self.p1_positions<<8 & self.p1_positions<<16 & self.p1_positions<<24)>0||
        (self.p1_positions & self.p1_positions<<7 & self.p1_positions<<14 & self.p1_positions<<21)>0||
        (self.p1_positions & self.p1_positions<<9 & self.p1_positions<<18 & self.p1_positions<<27)>0
    }
    fn check_p2_wins(&self) -> bool{
        return (self.p2_positions & self.p2_positions<<1 & self.p2_positions<<2 & self.p2_positions<<3)>0||
        (self.p2_positions & self.p2_positions<<8 & self.p2_positions<<16 & self.p2_positions<<24)>0||
        (self.p2_positions & self.p2_positions<<7 & self.p2_positions<<14 & self.p2_positions<<21)>0||
        (self.p2_positions & self.p2_positions<<9 & self.p2_positions<<18 & self.p2_positions<<27)>0;
    }
}

impl fmt::Display for ConnectFourGameV2{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res: String = String::new();
        let p1_board: Vec<u8> = to_base_2_rev(self.p1_positions);
        let p2_board: Vec<u8> = to_base_2_rev(self.p2_positions);
        for i in 0..6{
            for j in 0..8{
                let idx: usize = 47-i*8-(7-j);
                if j==7{
                    res.push('\n');
                }else if idx<p1_board.len() && p1_board[idx]==1{
                    res.push('X');
                }else if idx<p2_board.len() && p2_board[idx]==1{
                    res.push('O');
                }else{
                    res.push('_');
                }
            }
        }

        return write!(f,"{}",res)
    }
}

fn to_base_2_rev(num: u64) -> Vec<u8>{
    let mut temp: u64 = num.clone();
    let mut res: Vec<u8> = Vec::new();
    while temp>0{
        res.push((temp%2).try_into().unwrap_or(0));
        temp/=2;
    }
    // res.reverse();
    res
}






#[derive(Clone, Copy, PartialEq)]
enum TileState{
    Blank,
    Player1, 
    Player2 
}
impl ToString for TileState{
    fn to_string(&self) -> String{
        match self{
            TileState::Blank => String::from("_"),
            TileState::Player1 => String::from("X"),
            TileState::Player2 => String::from("O")
        }
    }
}
impl TileState{
    fn other(&self) -> TileState{
        match self{
            TileState::Blank => TileState::Blank,
            TileState::Player1 => TileState::Player2,
            TileState::Player2 => TileState::Player1
        }
    }
}
struct ConnectFourGame{
    board: [[TileState;6];7],
    heights: [u32;7],
    moves: Vec<u32> // number from 0-7
}
impl ConnectFourGame{
    fn create() -> ConnectFourGame{
        return Self{board: [[TileState::Blank;6];7], heights: [0;7], moves: Vec::new()}
    }
    fn make_move(&mut self, column: u32) -> bool{
        if column>7 || self.heights[column as usize] > 5 {return false}
        self.board[column as usize][self.heights[column as usize] as usize]=self.whos_turn();
        self.heights[column as usize]+=1;
        self.moves.push(column);
        return true;
    }
    fn undo_move(&mut self){
        if self.moves.len()>0{
            let last_move: usize = self.moves.pop().unwrap() as usize;
            self.heights[last_move]-=1;
            self.board[last_move][(self.heights[last_move]) as usize] = TileState::Blank;
        }
    }
    fn whos_turn(&self) -> TileState{
        return if self.moves.len()%2==0 {TileState::Player1} else {TileState::Player2}
    }
    fn check_prev_move_won(&self) -> bool{
        if self.moves.len()==0 {return false}
        let checking_tile: TileState = if self.moves.len()%2==0 {TileState::Player2} else {TileState::Player1};
        let last_move: i32 = *self.moves.last().unwrap() as i32;
        let last_move_height: i32 = (self.heights[last_move as usize]-1) as i32;
        //let col_span: (u32, u32) = ((last_move-4).clamp(0,6),(last_move+4).clamp(0,6));
        //let row_span: (u32, u32) = ((last_move_height-4).clamp(0,5),(last_move_height+4).clamp(0,5));
        let directions: [(i32,i32);4] = [(1,0),(0,1),(1,1),(1,-1)];
        for dir in directions{
            let mut counter: u32 = 1;
            let mut stop_checking_dir1: bool = false;
            let mut stop_checking_dir2: bool = false;
            for i in 1..5{
                if !stop_checking_dir1 && int_between(last_move+dir.0*i,-1,7) && int_between(last_move_height+dir.1*i,-1,6) && self.board[(last_move+dir.0*i) as usize][(last_move_height+dir.1*i) as usize]==checking_tile{
                    counter+=1;
                }else{
                    stop_checking_dir1=true;
                }
                if !stop_checking_dir2 && int_between(last_move-dir.0*i,-1,7) && int_between(last_move_height-dir.1*i,-1,6) && self.board[(last_move-dir.0*i) as usize][(last_move_height-dir.1*i) as usize]==checking_tile{
                    counter+=1;
                }else{
                    stop_checking_dir2=true;
                }
                if counter>3 {return true}
            }
        }
        return false;
    }
}

impl fmt::Display for ConnectFourGame{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res: String = String::new();
        for r in (0..self.board[0].len()).rev(){
            for c in 0..self.board.len(){
                res+=&self.board[c][r].to_string();
            }
            res+="\n";
        }
        return write!(f,"{}",res)
    }
}


struct ConnectFourSolver;

impl ConnectFourSolver{
    fn best_move(game: &mut ConnectFourGameV2) -> u32{
        let my_player: TileState = if game.is_p1s_turn() {TileState::Player1} else {TileState::Player2};
        let mut legal_moves: Vec<u32> = Vec::new();
        let mut not_losing_moves: Vec<u32> = Vec::new();
        for i in 0..7{
            if game.make_move(i){
                let res: TileState = Self::evaluate(game, 6);
                game.undo_move();
                println!("{}, {}",my_player.to_string(), res.to_string());
                legal_moves.push(i);
                if res==my_player {return i}
                else if res==TileState::Blank {not_losing_moves.push(i)}
            }
        }
        return not_losing_moves.pop().unwrap_or(legal_moves.pop().unwrap_or(0));
    }
    fn evaluate(game: &mut ConnectFourGameV2, max_depth: u32) -> TileState{ // -1 = player2 win, 1 = player1 win, 0 = draw
        if game.check_p1_wins(){
            return TileState::Player1
        }else if game.check_p2_wins(){
            return TileState::Player2
        }
        if max_depth==0 {return TileState::Blank}
        for i in 0..7{
            if game.make_move(i){
                let eval: TileState = Self::evaluate(game, max_depth-1);
                game.undo_move();
                if game.is_p1s_turn() && eval==TileState::Player1{
                    return eval
                }else if !game.is_p1s_turn() && eval==TileState::Player2{
                    return eval;
                }
            }
        }
        return TileState::Blank
    }
}

fn int_between(num: i32, min: i32, max: i32) -> bool{
    return num > min && num < max;
}