//https://leetcode.com/problems/palindrome-number/

pub fn is_palindrome(number: i32) -> bool {
    if number.to_string() == String::from(number.to_string().chars().rev().collect::<String>())
    {
        true
    }   
    else
    {
        false
    }
}

pub fn is_palindrome2(n: i32) -> bool {
    if n<0{
        return false;
    }
       

    let mut number = n;
    let mut index_number = 0;
    let mut reverse=0;
    while number!=0 {
        index_number = number % 10;
        reverse = reverse *10 + index_number;
        number = number/10;
    }
    println!("n={}, reverse={}", n, reverse);

    if n == reverse {
        return true;
    }
    else{
        return false;
    }
}

#[cfg(test)]
 mod tests {
     use super::*;
    
     #[test]        fn test1() {
       assert_eq!(is_palindrome(1221).to_owned(), true);
    }
    
    #[test]
     fn test2() {
        assert_eq!(is_palindrome(1234).to_owned(), false);
    }
}
