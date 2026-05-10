impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let len=nums.len();
        let mut l:usize=0;
        let mut r:usize=len-1;
        let mut mid: usize=0;
        if nums[l]<=nums[r] { return nums[l]; }
        while l<r && l>=0 && l<len && r>=0 && r<len{
            mid=l+(r-l)/2;
            if nums[r]<nums[mid]{
                l=mid+1;
            }else{
                r=mid;
            }
        }
        return nums[r];
    }
}
