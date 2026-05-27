class Solution {

    public String encode(List<String> strs) {
        StringBuilder sb=new StringBuilder();
        for(String str: strs){
            sb
            .append(str.length())
            .append(";")
            .append(str);
        }
        return sb.toString();
    }

    public List<String> decode(String str) {
        List<String> res=new LinkedList<>();
        char[] strArr=str.toCharArray();
        int len=strArr.length;
        int start=0;
        while(start<len){
            int posSep=str.indexOf(";", start);
            String sizeStr=str.substring(start, posSep);
            System.out.println(sizeStr);
            int size=Integer.parseInt(sizeStr);
            res.add(str.substring(posSep+1, posSep+1+size));
            start=posSep+size+1;
        }
        return res;
    }
}
