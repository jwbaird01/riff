/* 
    Rust tool to find differences in two texts or files, basically diff but can take a piped input..

    Red text is new text.
    Yellow Background text is removed text.

        E.G.
            <command> | riff <command> (to get miniscule differences quickly)
            <command> | riff -m <command> (show missing) [non-functional yet]
            <command> | riff -w <command> (to get differences but waits on user unput to run the seccond command ... useful for idenifying usbs probably)
                        riff <file1> <file2> (Replaces basic diff usage.)

            riff -h || riff --help (shows above info)

*/

// Written By : SB
// No AI because im big brained!!! ;3

//use std::io::{self,BufRead};

fn main() {
    println!("{}",riff("this is the first string".to_string(), "this is the seccond string".to_string()));
}

fn riff(pt1:String,pt2:String) -> String {
    // Bash Color
    let red:String = "\\033[31m".to_string(); // if new
    let _yel_bg:String = "\\033[0;43m".to_string(); // if removed //UNUSED CURRENTLY
    let clr:String = "\\e[0m".to_string(); //end color

    let diffs: String = pt2.to_string();
    let marked_for_gutting1 = pt1.to_string();
    let marked_for_gutting2 = pt2.to_string();
    let mut lines1: std::str::Lines<'_> = marked_for_gutting1.lines();
    let mut lines2: std::str::Lines<'_> = marked_for_gutting2.lines();
    let mut line_idx: Vec<(usize,String)> = Vec::new();
    for (i,line) in lines1.clone().into_iter().enumerate() { // find different lines
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
    for (row,line) in line_idx { // find different charachters
        let mut d = line.len();
        while d < line.len() {
            let mut col_start = usize::MAX;
            let mut diff_txt = "".to_string();
            for (i,c) in line.chars().enumerate() {
                match lines1.nth(row) {
                    Some(tmp_line) => {
                        match tmp_line.chars().nth(i) {
                            Some(tmp_char) => {
                                if c != tmp_char {
                                    if col_start == usize::MAX {
                                        col_start = d;
                                    }
                                    diff_txt += &c.to_string();
                                } else if col_start != usize::MAX {
                                    full_idx.push((row,col_start,d,diff_txt));
                                    break;
                                }
                            }
                            None => print!("")
                        }
                    }
                    None => print!("")
                }
            }
            d = d+1;
        }
    }
    let mut all_diffs:Vec<String> = Vec::new();
    let mut gutted_diffs = diffs.lines();
    for (row, col_start,col_end,_diff_txt) in full_idx {
        match gutted_diffs.nth(row) {
            Some(txt) => {
                let mut tmp_line = txt.to_string();
                tmp_line.insert_str(col_start, &red);
                tmp_line.insert_str(col_end+&red.len(), &clr);
                all_diffs.push(tmp_line.to_string());
            }
            None => print!("")
        }
    }    
    return all_diffs.join("\n");
}