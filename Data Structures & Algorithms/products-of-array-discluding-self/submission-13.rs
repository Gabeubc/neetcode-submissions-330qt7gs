impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut res = vec![0; len];
        let mut to_right = vec![0; len];
        let mut to_left = vec![0; len];
        to_right[0]=1;
        for i in 1..len{
            to_right[i]=to_right[i-1]*nums[i-1];
        }
        to_left[len-1]=1;
        for i in (0..len-1).rev(){
            to_left[i]=to_left[i+1]*nums[i+1];
        }
        for i in 0..len{
            res[i]=to_left[i]*to_right[i];
        }
        return res;
    }
}
