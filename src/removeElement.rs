// https://leetcode.com/problems/remove-element/
//??
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k: i32 = 0;
    let mut numbers_quantity = nums.len() as i32;
    for i in 0..nums.len()
    {
        if nums[i] == val
        {
            k += 1;
        }
    }
    nums.retain(|value| *value != val);
    return numbers_quantity - k;
}

mod tests {
    use super::*;
   
    #[test]        
    fn test1() {
        let mut nums= vec![0,3,4,0,6,2,0,0,3];
        assert_eq!(remove_element(&mut nums, 0), 5);
   }
   
   #[test]
    fn test2() {
        let mut nums= vec![0,3,4,0,6,2,0,0,3];
        assert_eq!(remove_element(&mut nums,3).to_owned(), 7);
   }
}
