use std::process::Command;
use std::clone::Clone;
// use std::string::String;
// use std::fmt

// Create array that is centered



fn main() {
    let a = Command::new("xrandr").output().expect("some error");


    let text = a.stdout.clone();

    let mut old_i: usize = 0;
    for i in 0..text.len(){
        if text[i]==10{ // 10 is newline utf-8 formatting
            println!("-------------------------NEW PRINT");
            println!("{}", String::from_utf8(text[old_i..i].to_vec()).unwrap());
            old_i = i+1
        }
    }


}

// Standalone
// open a window that lets you click on what you want and immidatly closes after click

// 1. Detect connected screens,
//      -- add them to a monitor struct

// 2. Make a window popup
//      -- selection choises
//          -- extend (highest available)
//          -- duplicate (lowest common resolution)
//          -- screen position left or right
//          -- Only Secondairy
//          -- Only Primary


//          Future feature screen positioning
//
//