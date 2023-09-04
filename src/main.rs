use std::{thread::current, 
    default, 
    collections::LinkedList,
    fs::File,
    io::{self, BufRead},
    path::Path
};

use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::{JsCast, JsValue};

fn main() {
    println!("Hello, world!");
    let val:usize = 0;
    println!("{}",val-1);

}

#[derive(Clone, Copy, PartialEq)]
enum TileState{
    Blank,
    Player1, 
    Player2 
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
        if column<0 || column>7 || self.heights[column as usize] > 6 {return false}
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
            for i in 1..4{
                if !stop_checking_dir1 && last_move+dir.0*i<7 && last_move_height+dir.1*i<6 && self.board[(last_move+dir.0*i) as usize][(last_move_height+dir.1*i) as usize]==checking_tile{
                    counter+=1;
                }else{
                    stop_checking_dir1=true;
                }
                if !stop_checking_dir2 && last_move-dir.0*i>=0 && last_move_height-dir.1*i>=0 && self.board[(last_move-dir.0*i) as usize][(last_move_height-dir.1*i) as usize]==checking_tile{
                    counter+=1;
                }else{
                    stop_checking_dir2=true;
                }
                if counter>=3 {return true}
            }
        }
        return false;
    }
}


struct ConnectFourSolver{
}

impl ConnectFourSolver{
    fn best_move(game: &mut ConnectFourGame) -> u32{
        let my_player: TileState = game.whos_turn();

        let mut not_losing_moves: Vec<u32> = Vec::new();
        for i in 0..6{
            if game.make_move(i){
                let res: TileState = Self::evaluate(game);
                game.undo_move();
                if res==my_player {return i}
                else if res==TileState::Blank {not_losing_moves.push(i)}
            }
        }
        return not_losing_moves.pop().unwrap_or(0);
    }
    fn evaluate(game: &mut ConnectFourGame) -> TileState{ // -1 = player2 win, 1 = player1 win, 0 = draw
        if game.check_prev_move_won(){
            game.undo_move();
            return game.whos_turn()
        }
        for i in 0..6{
            if game.make_move(i){
                let eval: TileState = Self::evaluate(game);
                game.undo_move();
                if eval == game.whos_turn(){
                    return eval
                }
            }
        }

        return TileState::Blank;
    }
}