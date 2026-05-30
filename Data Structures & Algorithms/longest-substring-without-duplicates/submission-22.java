class Solution {
    public int lengthOfLongestSubstring(String s) {
        int len=s.length();
        int l=0;
        Set<Character> set=new HashSet<>();
        int max=0;
        for(int r=0; r<len; r++){
            while(set.contains(s.charAt(r))){
                set.remove(s.charAt(l));
                l++;
            }
            max=Math.max((r-l+1), max);
            set.add(s.charAt(r));
        }
        return max;
    }
}
