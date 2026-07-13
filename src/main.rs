/* 
    Rust tool to find differences in two texts or files, basically diff but can take a piped input..

    Red text is new text.
    Yellow Background text is removed text.

        E.G.
            <command> | riff <command> (to get miniscule differences quickly)
            <command> | riff -w <command> (to get differences but waits on user unput to run the seccond command ... useful for idenifying usbs probably)
                        riff <file1> <file2> (Replaces basic diff usage.)

*/

// Written By : SB
// No AI because im big brained!!! 

use std::io::{self,BufRead};



fn main() {
    // Bash Color
    let red:String = "\\033[31m".to_string(); // if new
    let yel_bg:String = "\\033[0;43m".to_string(); // if removed
}

fn riff(pt1:String,pt2:String) -> String {
    let diffs: String = pt1.to_string();
    let marked_for_gutting1 = pt1.to_string();
    let marked_for_gutting2 = pt2.to_string();
    let lines1: std::str::Lines<'_> = marked_for_gutting1.lines();
    let mut lines2: std::str::Lines<'_> = marked_for_gutting2.lines();
    let mut line_idx: Vec<(usize,String)> = Vec::new();
    for (i,line) in lines1.into_iter().enumerate() {
        match &lines2.nth(i) {
            Some(tmp_line) => {
                if tmp_line != &line {
                    line_idx.push((i,tmp_line.to_string()));
                }
            }
            None => print!("")
        }
    }              // row, col_start,col_end,diff_txt
    let mut full_idx:Vec<(usize,usize,usize,String)> = Vec::new();
    for (row,line) in line_idx {
        for (i,c) in line.chars().enumerate() {
            
        }
    }

    diffs
}