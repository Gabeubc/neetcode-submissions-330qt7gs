impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums: Vec::<i32>=nums;
        nums.sort();
        let mut res:Vec<Vec<i32>>=Vec::new();
        let len=nums.len();
        if len<3 { 
            return res; 
        }
        for i in 0..len-2 {
            if i>0 && nums[i]==nums[i-1]{
                continue;
            }
            let mut l=i+1;
            let mut r=len-1;
            let target=-nums[i];
            while l<r{
                let sum=nums[l]+nums[r];
                if sum==target{
                    res.push(vec![nums[l], nums[r], nums[i]]);
                    while l<r && nums[l]==nums[l+1]{
                        l+=1;
                    }
                    while l<r && nums[r]==nums[r-1]{
                        r-=1;
                    }
                    l+=1;
                    r-=1;
                }else if sum<target{
                    l+=1;
                }else{
                    r-=1;
                }
            }
        }
        return res;
    }
}
