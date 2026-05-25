class Solution {
    public int[] dailyTemperatures(int[] temperatures) {
        Deque<Integer> stack=new ArrayDeque();
        int len=temperatures.length;
        int[] res=new int[len];
        for(int i=0; i<len; i++){
            while(!stack.isEmpty() && temperatures[stack.peek()]<temperatures[i]){
                int pos=stack.pop();
                res[pos]=i-pos;
            }
            stack.push(i);
        }
        return res;
    }
}
