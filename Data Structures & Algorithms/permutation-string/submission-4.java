class Solution {
    public boolean checkInclusion(String s1, String s2) {
        int len1=s1.length();
        int len2=s2.length();
        if(len1>len2) return false;
        int[] countSort1=new int[26];
        int[] countSort2=new int[26];
        for(int i=0; i<len1; i++){
            countSort1[s1.charAt(i)-'a']+=1;
            countSort2[s2.charAt(i)-'a']+=1;
        }
        int l=0;
        for(int r=len1; r<len2; r++, l++){
            if(matches(countSort1, countSort2)) return true;
            countSort2[s2.charAt(r)-'a']+=1;
            countSort2[s2.charAt(l)-'a']-=1;
        }
        return matches(countSort1, countSort2);
    }
    boolean matches(int[] s1, int[] s2){
        for(int i=0; i<26; i++){
            if(s1[i]!=s2[i]) return false;
        }
        return true;
    }

}
