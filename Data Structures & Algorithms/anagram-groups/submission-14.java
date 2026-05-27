class Solution {
    public List<List<String>> groupAnagrams(String[] strs) {
        Map<String, List<String>> groups=new HashMap<>();
        for(String str: strs){
            int[] key=new int[26];
            char[] strArray=str.toCharArray();
            for(char c:strArray){
                key[c-'a']++;
            }
            String keyStr=Arrays.toString(key);
            List<String> group=groups.getOrDefault(keyStr, new LinkedList<>());
            group.add(str);
            groups.put(keyStr, group);
        }
        return List.copyOf(groups.values());
    }
}
