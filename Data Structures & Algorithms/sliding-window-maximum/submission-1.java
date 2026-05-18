class Solution {
    record MaxCandidate(
        int pos,
        int val
    ){}
    public int[] maxSlidingWindow(int[] nums, int k) {
        int len=nums.length;
        int[] res=new int[len-k+1];
        PriorityQueue<MaxCandidate> pq=new PriorityQueue<>((a,b) -> b.val-a.val);
        int index=0;
        for(int i=0; i<k; i++) pq.add(new MaxCandidate(i, nums[i]));
        res[index++]=pq.peek().val;
        int l=1;
        for(int i=k; i<len; i++){
            pq.add(new MaxCandidate(i, nums[i]));
            while(pq.peek().pos<l) pq.poll();
            res[index++]=pq.peek().val;
            l++;
        }
        return res;
    }
}
