class Solution {
    public int[] topKFrequent(int[] nums, int k) {
        Map<Integer, Integer> freqs=new HashMap();
        int len=nums.length;
        List<Integer>[] buckets=new LinkedList[len+1];
        List<Integer> res=new LinkedList();
        for(int i=1; i<len+1; i++){
            buckets[i]=new LinkedList();
        }
        for(int num:nums){
            freqs.put(num, freqs.getOrDefault(num, 0)+1);
        }
        for(Map.Entry<Integer, Integer> e:freqs.entrySet()){
            buckets[e.getValue()].add(e.getKey());
            System.out.println(e.getKey()+"="+e.getValue());
        }
        for(int i=len; i>0; i--){
            int count=0;
            for(int num:buckets[i]){
                res.add(num);
                if(res.size()==k){
                    return res
                                .stream()
                                .mapToInt(Integer::intValue)
                                .toArray();
                }
            }
        }
        return res
                    .stream()
                    .mapToInt(Integer::intValue)
                    .toArray();
    }
}
