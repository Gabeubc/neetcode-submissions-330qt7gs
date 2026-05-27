class Solution {
    public int[] topKFrequent(int[] nums, int k) {
        int len=nums.length;
        List<Integer>[] buckets=new LinkedList[len+1];
        Map<Integer, Integer> freqs=new HashMap<>();
        for(int num: nums){
            freqs.put(num, freqs.getOrDefault(num, 0)+1);
        }
        for(Map.Entry<Integer, Integer> e:freqs.entrySet()){
            if(buckets[e.getValue()]==null) buckets[e.getValue()]=new LinkedList<>();
            buckets[e.getValue()].add(e.getKey());
        }
        List<Integer> res=new ArrayList<>();
        for(int i=len; i>=0; i--){
            if(buckets[i]!=null)
            for(int num: buckets[i]){
                res.add(num);
                if(res.size()==k){
                    int[] resArray=new int[k];
                    for(int j=0; j<k; j++){
                        resArray[j]=res.get(j);
                    }
                    return resArray;
                }
            }
        }
        return new int[k];
    }
}
