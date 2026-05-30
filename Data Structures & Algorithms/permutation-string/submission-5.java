class Solution {
    public boolean checkInclusion(String s1, String s2) {
        int[] count1=new int[26];
        int len1=s1.length();
        int len2=s2.length();
        for(int i=0; i<len1; i++){
            count1[s1.charAt(i)-'a']++;
        }
        int[] count2=new int[26];
        int l=0;
        for(int r=0; r<len2; r++){
            count2[s2.charAt(r)-'a']++;
            if(r-l+1==len1){
                boolean inclusion=true;
                for(int i=0; i<26; i++){
                    if(count2[i]!=count1[i]){
                        inclusion=false;
                        break;
                    }
                }
                if(inclusion) return true;
                count2[s2.charAt(l++)-'a']--;
            }
        }
        return false;
    }
}
