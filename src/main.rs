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
use colored::Colorize;

// needs rewrite for using colored::Colorize
// https://stackoverflow.com/questions/69981449/how-do-i-print-colored-text-to-the-terminal-in-rust

fn main() {
    let riffraff = riff("1234".to_string(), "123".to_string());
    println!("{}",riffraff);
}

fn riff(pt1: String, pt2: String) -> String {
    let lines1: Vec<&str> = pt1.lines().collect();
    let lines2: Vec<&str> = pt2.lines().collect();
    let max_lines = lines1.len().max(lines2.len());
    let mut all_diffs: Vec<String> = Vec::new();

    for row in 0..max_lines {
        let line1 = lines1.get(row).copied().unwrap_or("");
        let line2 = lines2.get(row).copied().unwrap_or("");
        let line1_chars: Vec<char> = line1.chars().collect();
        let line2_chars: Vec<char> = line2.chars().collect();
        let mut out = String::new();
        let mut i = 0usize;
        let max_len = line1_chars.len().max(line2_chars.len());

        while i < max_len {
            match (line1_chars.get(i).copied(), line2_chars.get(i).copied()) {
                (Some(ch1), Some(ch2)) if ch1 == ch2 => {
                    out.push(ch2);
                    i += 1;
                }
                (Some(_), Some(_)) => {
                    let mut diff = String::new();

                    while i < max_len {
                        match (line1_chars.get(i).copied(), line2_chars.get(i).copied()) {
                            (Some(a), Some(b)) if a == b => break,
                            (Some(_), Some(b)) => {
                                diff.push(b);
                                i += 1;
                            }
                            (Some(_), None) => {
                                i += 1;
                            }
                            (None, Some(b)) => {
                                diff.push(b);
                                i += 1;
                            }
                            (None, None) => break,
                        }
                    }

                    if !diff.is_empty() {
                        out.push_str(&format!("{}", diff.bold().red()));
                    }
                }
                (Some(_), None) => {
                    i += 1;
                }
                (None, Some(ch2)) => {
                    out.push_str(&format!("{}", ch2.to_string().bold().red()));
                    i += 1;
                }
                (None, None) => break,
            }
        }

        if !out.is_empty() {
            all_diffs.push(out);
        }
    }

    all_diffs.join("\n")
}