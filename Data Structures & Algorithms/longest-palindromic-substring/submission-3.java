class Solution {
    public String longestPalindrome(String s) {
        int len=s.length();
        if(isPalindrome(s, 0, len-1)) return new String(s);
        int resLen=0;
        String res=""+s.charAt(0);
        for(int l=0; l<len-1; l++){
            int end=l;
            for(int r=l+1; r<len; r++){
                if(s.charAt(l)==s.charAt(r)) end=r;
            }
            while(end>l && !isPalindrome(s, l , end)) end--;
            resLen=Math.max(resLen, end-l+1);
            if(resLen==end-l+1) res=s.substring(l, end+1);
        }
        return res;
    }
    private boolean isPalindrome(String s, int l, int r){
        while(l<=r){
            if(s.charAt(l++)!=s.charAt(r--)) return false;
        }
        return true;
    }
}
