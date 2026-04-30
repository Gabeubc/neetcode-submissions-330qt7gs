class Solution {
    public int maxProfit(int[] prices) {
        int len=prices.length;
        int l=0;
        int max=0;
        int r=1;
        while(r<len){
            if(prices[r]>prices[l]){
                max=Math.max(max, prices[r]-prices[l]);
            }else{
                l=r;
            }
            r+=1;
        }
        return max;
    }
}
