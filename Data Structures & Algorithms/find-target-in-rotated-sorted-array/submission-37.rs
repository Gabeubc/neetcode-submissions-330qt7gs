impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len=nums.len();
        let mut l=0;
        let mut r=len-1;
        if nums[l]>nums[r]{
            while l<r && l<len && r<len{
                let mid=l+(r-l)/2;
                if nums[r]<nums[mid]{
                    l=mid+1;
                }else{
                    r=mid;
                }
            }
        }
        r=len-1;
        return if nums[r]>=target{
            Self::binary_search(nums, l, r, target)
        }else{
            Self::binary_search(nums, 0, l-1, target)
        }
    }

    fn binary_search(nums:Vec<i32>, l:usize, r:usize, target:i32) -> i32{
        let len=nums.len();
        let mut l=l;
        let mut r=r;
        while l<=r && l<len && r<len{
            let mid=l+(r-l)/2;
            if nums[mid]==target{
                    return mid as i32;
                }else if nums[mid]<target{
                    l=mid+1;
                    }else{
                r=mid-1;
            }
        }
        return -1;
    }
}
