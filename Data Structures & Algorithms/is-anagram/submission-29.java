class Solution {
    public boolean isAnagram(String s, String t) {
        int[] count=new int[26];
        char[] sArr=s.toCharArray();
        char[] tArr=t.toCharArray();
        for(char c: sArr){
            count[c-'a']++;
        }
        for(char c:tArr){
            int idx=c-'a';
            if(count[idx]==0) return false;
            count[c-'a']--;
        }
        for(int i=0; i<26; i++){
            if(count[i]>0) return false;
        }
        return true;
    }
}
