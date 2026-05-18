impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len=nums.len();
        let k=k as usize;
        let mut heap:BinaryHeap::<(i32, usize)>=BinaryHeap::new();
        let mut res:Vec<i32>=Vec::new();
        for i in 0..k{
            heap.push((nums[i], i));
        }
        res.push(heap.peek().unwrap().0);
        let mut l=1;
        for r in k..len{
            heap.push((nums[r], r));
            while heap.peek().unwrap().1<l{
                heap.pop();
            }
            res.push(heap.peek().unwrap().0);
            l+=1;
        }
        return res;
    }
}
