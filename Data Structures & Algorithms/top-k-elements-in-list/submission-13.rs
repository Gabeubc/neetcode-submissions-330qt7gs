impl Solution {

    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let max_freqs=nums.len();
        let mut freqs:HashMap<i32, usize>=HashMap::new();
        let mut buckets:Vec::<Vec<i32>>=vec![Vec::new();max_freqs+1];
        let mut res:Vec<i32>=Vec::new();
        for num in nums{
            *freqs.entry(num).or_insert(0)+=1;
        }
        for (key, freq) in freqs{
            buckets[freq].push(key);
        }
        for freq in (0..max_freqs+1).rev(){
            for &key in buckets[freq].iter(){
                res.push(key);
                if res.len()==(k as usize){
                    return res;
                }
            }
        }
        return res;
    }
}
