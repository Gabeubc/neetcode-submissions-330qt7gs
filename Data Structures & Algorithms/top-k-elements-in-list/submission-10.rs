impl Solution {

    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freqs:HashMap<i32,i32>=HashMap::new();
        let mut buckets=vec![vec![];nums.len()+1];
        let mut res=vec![];
        for num in nums{
            *freqs.entry(num).or_insert(0)+=1;
        }
        for (num, v) in freqs{
            buckets[v as usize].push(num);
        }
        for bucket in buckets.iter().rev(){
            for &num in bucket{
                res.push(num);
                if res.len()==(k as usize){
                    return res;
                }
            }
        }
        return res;
    }
}
