class Solution {
    record Triplet(
        int first,
        int second,
        int third
    ){}
    public List<List<Integer>> threeSum(int[] nums) {
        int len=nums.length;
        List<List<Integer>> res=new LinkedList<>();
        Set<Triplet> set=new HashSet<>(); 
        Arrays.sort(nums);
        for(int i=0; i<len; i++){
            if(i>0 && nums[i]==nums[i-1]) continue;
            int target=-nums[i];
            int l=0;
            int r=len-1;
            while(l<r){
                if(l==i){
                    l++;
                    continue;
                }
                if(r==i){
                    r--;
                    continue;
                }
                if(nums[l]+nums[r]==target){
                    int[] sol=new int[]{nums[l], nums[r], nums[i]};
                    Arrays.sort(sol);
                    Triplet triplet=new Triplet(sol[0], sol[1], sol[2]);
                    if(!set.contains(triplet)){
                        res.add(List.of(triplet.first, triplet.second, triplet.third));
                        set.add(triplet);
                    }
                    l++;
                    r--;
                }else if(nums[l]+nums[r]<target){
                    l++;
                }else{
                    r--;
                }
            }
        }
        return res;
    }
}
