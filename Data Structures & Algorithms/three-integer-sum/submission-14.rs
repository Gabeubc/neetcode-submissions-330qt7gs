impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums:Vec<i32>=nums.iter().cloned().collect();
        nums.sort();
        let len=nums.len();
        let mut res:Vec<Vec<i32>>=Vec::new();
        for i in 0..len-2{
            if i>0 && nums[i-1]==nums[i]{
                continue;
            }
            let mut l=i+1;
            let mut r=len-1;
            let target=-nums[i];
            while l<r{
                let sum=nums[l]+nums[r];
                if sum==target{
                    res.push(vec![nums[l], nums[r], -target]);
                    while l<r {
                        if nums[l]==nums[l+1]{
                            l+=1;
                        }else{
                            break;
                        }
                    }
                    while l<r {
                        if nums[r]==nums[r-1]{
                            r-=1;
                        }else{
                            break;
                        }
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
