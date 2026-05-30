class Solution {
    record Max(
        int val,
        int pos
    ){}
    public int[] maxSlidingWindow(int[] nums, int k) {
      int len=nums.length;
      int l=0;
      Max max=new Max(Integer.MIN_VALUE, -1);
      int[] res=new int[len-k+1];
      for(int i=0; i<k; i++){
        if(max.val<nums[i]){
            max=new Max(nums[i], i);
        }
      }
      int j=0;
      res[j++]=max.val;
      l++;
      for(int r=k; r<len; r++, l++){
        if(max.pos<l){
            max=new Max(Integer.MIN_VALUE, -1);
            for(int i=l; i<=r; i++){
                if(max.val<nums[i]){
                    max=new Max(nums[i], i);
                }
            }
        }else{
            if(max.val<nums[r]){
                max=new Max(nums[r], r);
            }
        }
        res[j++]=max.val;
      }
      return res;  
    }
}
