impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l=0;
        let mut r=numbers.len()-1;
        while l<r{
            let sum=numbers[l]+numbers[r];
            if target==sum{
                return vec![(l as i32)+1, (r as i32)+1];
            }else if target<sum{
                r-=1;
            }else{
                l+=1;
            }
        }
        return vec![];
    }
}
