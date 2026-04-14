impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len()==0{
            return 0;
        }
        let nums_set: HashSet::<i32>=nums.iter().cloned().collect();
        let mut res=1;
        for &num in &nums{
            if nums_set.contains(&(num-1)){
                let mut len=1;
                let mut curr=num;
                while nums_set.contains(&curr){
                    curr+=1;
                    len+=1;
                }
                if res<len{
                    res=len;
                }
            }
        }
        return res;
    }
}
