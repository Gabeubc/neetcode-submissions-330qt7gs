class Solution {
    public int search(int[] nums, int target) {
        int len=nums.length;
        int l=0;
        int r=len-1;
        int mid=0;
        int pivot=-1;
        if(nums[l]>nums[r]){
            while(l<r){
                mid=l+(r-l)/2;
                if(nums[r]<nums[mid]){
                    l=mid+1;
                }else{
                    r=mid;
                }
            }
            pivot=l;
        }
        int res=-1;
        if(pivot!=-1){
            res=binarySearch(nums, 0, pivot-1, target);
            if(res==-1){
                res=binarySearch(nums, pivot, len-1, target);
            }
        }else{
            System.out.println(pivot);
            res=binarySearch(nums, 0, len-1, target);
        }
        return res;

    }
    int binarySearch(int[] nums, int l, int r, int target){
        int mid=0;
        while(l<=r){
            mid=l+(r-l)/2;
            if(target<nums[mid]){
                r=mid-1;
            }else if(target>nums[mid]){
                l=mid+1;
            }else{
                return mid;
            }
        }
        return -1;
    }
}
