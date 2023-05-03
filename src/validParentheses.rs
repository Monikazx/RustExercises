//https://leetcode.com/problems/valid-parentheses/

pub fn is_valid(s: String) -> bool {
    let mut opening = Vec::new();
    let mut openingChar: Option<char> ;
    let mut length;
    for c in s.chars()
    {
        length = opening.len();
        if c == '(' || c == '[' || c == '{'
        {
            opening.push(c);
        }
        else if c == ')' 
        {
            openingChar = opening.pop();
            if openingChar != Some('(') || length == 0
            {
                return  false;
            }
        }
        else if c == ']' 
        {
            openingChar = opening.pop();
            if openingChar != Some('[') || length == 0
            {
                return  false;
            }
        }
        else if c == '}' 
        {
            openingChar = opening.pop();
            if openingChar != Some('{') || length == 0
            {
                return  false;
            }
        }

    }

    if opening.len() == 0
    {
        return true;
    }
    return false;
}


mod tests {
    use super::*;
   
    #[test]        
    fn test1() {
       assert_eq!(is_valid("()[]{}".to_owned()), true);
   }
   
   #[test]
    fn test2() {
       assert_eq!(is_valid("(])".to_owned()), false);
   }
}