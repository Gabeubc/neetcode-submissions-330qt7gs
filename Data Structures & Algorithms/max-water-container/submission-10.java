class Solution {
    public int maxArea(int[] heights) {
        int l=0;
        int r=heights.length-1;
        int max=0;
        int area=0;
        while(l<r){
            if(heights[l]<heights[r]){
                area=(r-l)*heights[l];
                l++;
            }else{
                area=(r-l)*heights[r];
                r--;
            }
            max=Math.max(area, max);
        }
        return max;
    }
}
