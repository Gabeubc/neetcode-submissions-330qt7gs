impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len=nums.len();
        let mut res=vec![1;len];
        let mut left_to_rigth=vec![1;len];
        let mut right_to_left=vec![1;len];
        for i in 1..len{
            left_to_rigth[i]=left_to_rigth[i-1]*nums[i-1];
        }
        for i in (0..=len-2).rev(){
            right_to_left[i]=right_to_left[i+1]*nums[i+1];
        } 
        for i in 0..len{
            res[i]=left_to_rigth[i]*right_to_left[i];
        }
        return res;
    }
}
