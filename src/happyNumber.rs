//https://leetcode.com/problems/happy-number/


pub fn is_happy(n: i32) -> bool 
{
    const RADIX: u32 = 10;
    let mut sum;
    let mut number = n.to_string();
    loop
    {
        sum = 0;
        for i in 0..number.len()
        {
            sum += number.chars().nth(i).unwrap().to_string().parse::<i32>().unwrap() * number.chars().nth(i).unwrap().to_string().parse::<i32>().unwrap();
        }
        if sum == 1
        {
            print!("To jest szczęśliwa liczba");
            return true;
        }
        number = sum.to_string();
        if number.len() == 1 && number != "7"
        {
            print!("To NIE jest szczęśliwa liczba");
            return false;
        }
    }
}

#[cfg(test)]
 mod tests {
     use super::*;
    
     #[test]        
     fn test1() {
        assert_eq!(is_happy(19).to_owned(), true);
    }
    
    #[test]
     fn test2() {
        assert_eq!(is_happy(85).to_owned(), false);
    }
}