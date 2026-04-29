class Solution {
    public List<List<String>> groupAnagrams(String[] strs) {
        Map<List<Integer>, List<String>> groupAnagrams=new HashMap<>();
        List<Integer> key=Arrays
        .stream(new int[26])
        .boxed()
        .collect(Collectors.toList());
        for(String s:strs){
            groupAnagrams.computeIfAbsent(
                computeKey(s, key),
                k->new LinkedList()
            ).add(s);
        }
        return List.copyOf(groupAnagrams.values());
    }
    private List<Integer> computeKey(String s, List<Integer> key){
        for(int i=0; i<26; i++){
            key.set(i, 0);
        }
        for(char c:s.toCharArray()){
            int pos=((int) c)%26;
            key.set(pos, key.get(pos)+1);
        }
        return key;
    }
}
