impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len=nums.len()+1;
        let mut buckets:Vec<Vec<i32>>=vec![Vec::new();len];
        let mut freqs:HashMap<i32, usize>=HashMap::new();
        let mut res:Vec<i32>=Vec::new();
        for num in nums{
            *freqs.entry(num).or_insert(0)+=1;
        }
        for (k, v) in freqs{
            buckets[v].push(k);
        }
        for i in (0..len).rev(){
            for &num in buckets[i].iter(){
                res.push(num);
            }
            if res.len()==(k as usize){
                return res;
            }
        }
        return res;
    }
}
