//https://leetcode.com/problems/longest-common-prefix/

pub fn longest_common_prefix(strs: Vec<String>) -> String {
     let mut commonLength = 0;
     let mut consideredChar;
     let mut wordLength = strs[0].len() as i32;
     let mut wordIndex=0;

     for i in 1..strs.len() 
     {
        if (strs[i].len() as i32)  < wordLength 
        {
            wordLength = strs[i].len() as i32;
            wordIndex = i;
        } 
     }
     for i in 0..wordLength as usize
     {
        consideredChar = strs[wordIndex].chars().nth(i);
        for j in 0..strs.len()
        {
            if strs[j].chars().nth(i) != consideredChar
            {
                return strs[wordIndex][..commonLength].to_string();
            }
        }
        commonLength += 1;
        if wordLength == 1 || commonLength as i32 == wordLength as i32
        {
            return strs[wordIndex][..commonLength].to_string();
        }
     }  
     return "".to_string(); 
}

mod tests {
    use super::*;
   
    #[test]        
    fn test1() {
       let mut strings = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
       assert_eq!(longest_common_prefix(strings).to_owned(), "fl");
   }
   
   #[test]
    fn test2() {
        let mut strings= vec!["dog".to_string(),"racecar".to_string(),"car".to_string()];
        assert_eq!(longest_common_prefix(strings).to_owned(), "");
   }
}