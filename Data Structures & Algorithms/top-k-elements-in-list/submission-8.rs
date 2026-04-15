impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut buckets=vec![vec![];nums.len()+1];
        let mut freq: HashMap::<i32, i32>=HashMap::new();
        let mut res: Vec<i32>=Vec::new();
        for &num in &nums{
            *freq.entry(num).or_insert(0)+=1;
        }
        for (el, freq) in freq{
            buckets[freq as usize].push(el);
        }
        for bucket in buckets.iter().rev(){
            for &el in bucket{
                res.push(el);
                if res.len()==(k as usize){
                    return res;
                }
            }
        }
        return res;
    }
}
