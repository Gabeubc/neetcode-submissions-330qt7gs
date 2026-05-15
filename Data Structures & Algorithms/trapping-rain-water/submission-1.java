class Solution {
    public int trap(int[] height) {
        int len=height.length;
        int l=0;
        int r=len-1;
        int maxL=height[0];
        int maxR=height[len-1];
        int sum=0;
        while(l<r){
            int vol;
            if(height[l]<height[r]){
                vol=maxL-height[l];
                if(vol>=0) sum+=vol;
                maxL=Math.max(maxL, height[l]);
                l++;
            }else{
                vol=maxR-height[r];
                if(vol>=0) sum+=vol;
                maxR=Math.max(maxR, height[r]);
                r--;
            }
        }
        return sum;
    }
}
