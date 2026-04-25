impl Solution {

    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let max_freq=nums.len();
        let mut freqs:HashMap::<i32, usize>=HashMap::new();
        let mut buckets:Vec<Vec<i32>>=vec![Vec::<i32>::new();max_freq+1];
        let mut res:Vec::<i32>=Vec::new();
        for num in nums{
            *freqs.entry(num).or_insert(0)+=1;
        }
        for (k, freq) in freqs{
            buckets[freq].push(k);
        }
        for i in (0..max_freq+1).rev(){
            for &k in buckets[i].iter(){
                res.push(k);
            }
            if res.len()==(k as usize){
                return res;
            }
        }
        return res;
    }
}
