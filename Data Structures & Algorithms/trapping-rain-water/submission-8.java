class Solution {
    public int trap(int[] height) {
        int len=height.length;
        if(len==2) return 0;
        int l=0;
        int r=len-1;
        int maxR=height[r];
        int maxL=height[l];
        int sum=0;
        while(l<r){
            if(height[l]<height[r]){
                if(maxL>height[l]){
                    sum+=(maxL-height[l]);
                }else{
                    maxL=height[l];
                }
                l++;
            }else{
                if(maxR>height[r]){
                    sum+=(maxR-height[r]);
                }else{
                    maxR=height[r];
                }
                r--;
            }
        }
        return sum;
    }
}
