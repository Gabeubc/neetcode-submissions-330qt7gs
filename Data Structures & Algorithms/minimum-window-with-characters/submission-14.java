class Solution {
    public String minWindow(String s, String t) {
        int lenS=s.length();
        int lenT=t.length();
        Map<Character, Integer> need=new HashMap<>();
        Map<Character, Integer> have=new HashMap<>();
        String res="";
        int min=Integer.MAX_VALUE;
        for(int i=0; i<lenT; i++){
            char c=t.charAt(i);
            int freq=need.getOrDefault(c, 0)+1;
            need.put(c, freq);
        }
        int l=0;
        int count=0;
        for(int r=0; r<lenS; r++){
            char c=s.charAt(r);
            int freq=have.getOrDefault(c, 0)+1;
            have.put(c, freq);
            if(need.containsKey(c) && freq==need.get(c)){
                count++;
            }
            while(count==need.size()){
                c=s.charAt(l);
                freq=have.get(c)-1;
                have.put(c, freq);
                if(need.containsKey(c) && freq<need.get(c)){
                    if(min>r-l+1){
                        min=r-l+1;
                        res=s.substring(l, r+1);
                    }
                    count--;
                }
                l++;
            }
        }
        System.out.println(count);
        return res;
    }
}
