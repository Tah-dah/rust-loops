extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
fn main() {
    // let  mut i = 1;
    // while i < 21 {
    //     println!("{}", i);
    //     i = i+ 1;
    // }
    let mut i = 12;
    while i >= -14 {
        if (i % 2) == 0 {
            println!("{}", i); 
         } 
            
         i = i - 1;
    }

    let lc = "launchcode";
    
    let cl: String = lc.graphemes(true).rev().collect();
    let vec_cl: Vec<char> = cl.chars().collect();
    
    for c in vec_cl {
        println!("{}", c);
    }
    
    

}

