class Solution {
    public int characterReplacement(String s, int k) {
        int len=s.length();
        int l=0;
        int res=0;
        int max=0;
        Map<Character, Integer> freqs=new HashMap();
        for(int r=0; r<len; r++){
            freqs.put(s.charAt(r), freqs.getOrDefault(s.charAt(r), 0)+1);
            max=Math.max(max, freqs.get(s.charAt(r)));
            while((r-l+1)-max>k){
                freqs.put(s.charAt(l), freqs.getOrDefault(s.charAt(l), 0)-1);
                l++;
            }
            res=r-l+1;
        }
        return res;
    }
}
