//https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/


pub fn str_str(haystack: String, needle: String) -> i32 {
    let mut count = 0;
    for i in 0..haystack.len() 
    {
        count = 0;
        for j in 0..needle.len()
        {
            if haystack.chars().nth(i+j) == needle.chars().nth(j)
            {
                count += 1;
            }
            else
            {
                count = 0;
                break;
            }
            
            if count == needle.len()
            {
                return i as i32;
            }
        }
    }
    return -1;
}

#[test]        
fn test1() {
   assert_eq!(str_str(("tkst te st").to_owned(), ("te").to_owned()), 5);
}

#[test]
fn test2() {
    assert_eq!(str_str(("tekst te st").to_owned(), ("tex").to_owned()), -1);
}