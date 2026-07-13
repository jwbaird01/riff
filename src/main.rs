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

// .nth(i) destroys the iterators
    // need to use collect ( already added where needed)
    // then use .get(i) instead of nth. should be able to write without the while loop this way.
    // can also use [i] for some 

fn main() {
    let riffraff = riff("this is the first string".to_string(), "this is the seccond string".to_string());
    println!("{}",riffraff);
}

fn riff(pt1:String,pt2:String) -> String {
    // Bash Color
    let red:String = "\\033[31m".to_string(); // if new
    let _yel_bg:String = "\\033[0;43m".to_string(); // if removed //UNUSED CURRENTLY
    let clr:String = "\\e[0m".to_string(); //end color

    let diffs: String = pt2.to_string();
    let marked_for_gutting1 = pt1.to_string();
    let marked_for_gutting2 = pt2.to_string();
    let lines1: Vec<&str> = marked_for_gutting1.lines().collect();
    let lines2: Vec<&str> = marked_for_gutting2.lines().collect();
    let mut line_idx: Vec<(usize,String)> = Vec::new();
    for (i,line) in lines1.clone().into_iter().enumerate() { // find different lines
        match lines2.get(i) {
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
            let mut col_start = usize::MAX;
            let mut diff_txt = "".to_string();
            for (i,c) in line.chars().enumerate() {
                match lines1.get(row) {
                    Some(tmp_line) => {
                        let tmp_tmp_line:Vec<char> = tmp_line.chars().collect();
                        match tmp_tmp_line.get(i) {
                            Some(tmp_char) => {
                                if &c != tmp_char {
                                    if col_start == usize::MAX {
                                        col_start = i;
                                    }
                                    diff_txt += &c.to_string();
                                } else if col_start != usize::MAX {
                                    full_idx.push((row,col_start,i,diff_txt));
                                    break;
                                }
                            }
                            None => print!("")
                        }
                    }
                    None => print!("")
                }
            }
    }
    let mut all_diffs:Vec<String> = Vec::new();
    let gutted_diffs:Vec<&str> = diffs.lines().collect();
    for (row, col_start,col_end,_diff_txt) in full_idx {
        match gutted_diffs.get(row) {
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