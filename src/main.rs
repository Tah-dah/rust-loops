extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
fn main() {
    
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
    
    let mut even_vec:Vec<i8> = vec![];
    //let odd_vec:Vec<i8> = vec![];
    let full_vec:Vec<i8> = vec![2, 3, 13, 18, -5, 38, -10, 11, 0, 104];
    
    for e  in full_vec {
        if e % 2 == 1 { 
            even_vec.push(e);
            println!("{}", e);
    } else {
        continue;
    }

    
}

}

