class Solution {
    public int maxProduct(int[] nums) {
        int cMax=nums[0];
        int cMin=nums[0];
        int res=nums[0];
        for(int i=1; i<nums.length; i++){
            int max=cMax*nums[i];
            int min=cMin*nums[i];
            cMax=Math.max(Math.max(max, min), nums[i]);
            cMin=Math.min(Math.min(max, min), nums[i]);
            res=Math.max(res, cMax);
        }
        return res;
    }
}
