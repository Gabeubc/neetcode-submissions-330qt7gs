class Solution {
    public int characterReplacement(String s, int k) {
        int len=s.length();
        int l=0;
        int max=0;
        Map<Character, Integer> freqs=new HashMap<>();
        for(int r=0; r<len; r++){
            char c=s.charAt(r);
            int freq=freqs.getOrDefault(c, 0)+1;
            freqs.put(c, freq);
            max=Math.max(max, freq);
            while((r-l+1)-max>k){
                freqs.put(s.charAt(l), freqs.getOrDefault(s.charAt(l), 0)-1);
                l++;
            }
        }
        return len-l;
    }
}
