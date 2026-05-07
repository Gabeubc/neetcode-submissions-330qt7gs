impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut l=0;
        let mut r=heights.len()-1;
        let mut max=0;
        let mut area=0;
        while l<r{
            let base=(r-l) as i32;
            if heights[l]<heights[r]{
                area=area.max(base*heights[l]);
                l+=1;
            }else{
                area=area.max(base*heights[r]);
                r-=1;
            }
        }
        return area;
    }
}
