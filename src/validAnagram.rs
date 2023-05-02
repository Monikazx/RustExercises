//https://leetcode.com/problems/valid-anagram/

use std::{collections::HashMap, hash::Hash};


pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len()
    {
        return false;
    }

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    for i in 0..s.len()
    {
        vec1.push(s.chars().nth(i).unwrap());
        vec2.push(t.chars().nth(i).unwrap());
    }

    vec1.sort();
    vec2.sort();
    
    if vec1==vec2
    {
        return true;
    }
    return false;    
}


mod tests {
    use super::*;
   
    #[test]        
    fn test1() {
       assert_eq!(is_anagram(("niedziela").to_owned(), ("dzielenia").to_owned()), true);
   }
   
   #[test]
    fn test2() {
       assert_eq!(is_anagram(("tak").to_owned(), ("nie").to_owned()), false);
   }
}


