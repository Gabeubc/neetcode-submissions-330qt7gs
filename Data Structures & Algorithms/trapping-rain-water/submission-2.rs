impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let len=height.len();
        let mut l=0;
        let mut r=len-1;
        let mut maxL=height[0];
        let mut maxR=height[r];
        let mut sum:i32=0;
        while l<r{
            let mut vol:i32=0;
            if height[l]<height[r]{
                vol=maxL-height[l];
                if vol>0 {sum+=vol;}
                maxL=maxL.max(height[l]);
                l+=1;
            }else{
                vol=maxR-height[r];
                if vol>0 {sum+=vol;}
                maxR=maxR.max(height[r]);
                r-=1;
            }
        }
        return sum;
    }
}
