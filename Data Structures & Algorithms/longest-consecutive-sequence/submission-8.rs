impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set:HashSet<i32>=nums.iter().cloned().collect();
        let mut max=0;
        for mut num in nums{
            if !set.contains(&(num-1)){
                let mut count:usize=0;
                while set.contains(&num){
                    count+=1;
                    num+=1;
                }
                if max<count{
                    max=count;
                }
            }
        }
        return max as i32;
    }
}
