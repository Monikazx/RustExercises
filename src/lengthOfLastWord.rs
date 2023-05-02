use std::ops::Index;

/*
    https://leetcode.com/problems/length-of-last-word/
    Given a string s consisting of words and spaces, return the length of the last word in the string.
    A word is a maximal substring consisting of non-space characters only.
*/
fn length_of_last_word(s: String) -> i32 
{
    let mut newS = s.trim();
    let mut index = newS.chars().count()-1 as usize;
    let mut my_vec: Vec<char> = newS.chars().collect();
    
    while my_vec[index]!=' ' && index>0
    {
        println!("{}",my_vec[index]);
        index = index-1;
    }
    if index == 0
    {
        return newS.len() as i32;
    }
    return newS.len() as i32 -1 -index as i32;
}

#[cfg(test)]
 mod tests {
     use super::*;
    
     #[test]        
     fn test1() {
        let text = "tekst tek st".to_string();
        assert_eq!(length_of_last_word(text).to_owned(), 2);
    }
    
    #[test]
     fn test2() {
        let text = "teks te tekst".to_string();
        assert_eq!(length_of_last_word(text).to_owned(), 5);
    }
}