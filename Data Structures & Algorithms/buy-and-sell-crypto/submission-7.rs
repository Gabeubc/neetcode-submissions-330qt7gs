impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut l=0;
        let mut max=0;
        let len=prices.len();
        for r in 1..len{
            if prices[r]<prices[l]{
                l=r;
                continue;
            }
            while prices[l]<prices[r]{
                max=max.max(prices[r]-prices[l]);
                if prices[l+1]<prices[l]{
                    l+=1;
                }else{
                    break;
                }
            }
        }
        return max;
    }
}
