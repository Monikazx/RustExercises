//https://leetcode.com/problems/remove-duplicates-from-sorted-array/

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut length = nums.len();
    let mut index = 0;
    while  index < length
    {
        if index+1<length
        {
            if nums[index] == nums[index+1] 
            {
                 nums.remove(index);
                 length -= 1;
            }
            else{
                index += 1;
            }
        }
        else 
        {
            break;
        }
    }
    return nums.len() as i32;
}

#[cfg(test)]
 mod tests {
     use super::*;
    
     #[test]        
     fn test1() {
        let mut nums= vec![0,0,1,2,3,3,4,5,6,7,7,8];
        assert_eq!(remove_duplicates(&mut nums).to_owned(), 9);
    }
    
    #[test]
     fn test2() {
        let mut nums= vec![0,0,1,2,2,2,2,3,4,5];
        assert_eq!(remove_duplicates(&mut nums).to_owned(), 6);
    }
}
