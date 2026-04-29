class Solution {
    public boolean isPalindrome(String s) {
        int l=0;
        int r=s.length()-1;
        char[] sArray=s.toLowerCase().toCharArray();
        while(l<r){
            while(l<s.length() && !Character.isLetterOrDigit(sArray[l])) l++;
            while(r>=0 && !Character.isLetterOrDigit(sArray[r])) r--;
            if(l<s.length() && r>=0 && sArray[l]!=sArray[r]) return false;
            l++;
            r--;
        }
        return true;
    }
}
