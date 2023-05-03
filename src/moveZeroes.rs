pub fn move_zeroes(nums: &mut Vec<i32>) -> bool {
    let mut index = nums.len()-1;
    let mut index;

    for i in 0..nums.len()
    {
        if nums[i] == 0
        {
            index = i;
            while(index < nums.len()-1 && nums[index] == 0)
            {
                index = index + 1;
            }
            nums[i] = nums[index];
            nums[index] = 0;
        }

    }
    print!("Wynik ---> {:?}", nums);
    //return nums
    return true;
}


#[cfg(test)]
 mod tests {
     use super::*;
    
     #[test]        fn test1() {
        let mut nums= vec![0,3,4,0,6,2,0,0,3];
       assert_eq!(move_zeroes(&mut nums),true);
    }
}