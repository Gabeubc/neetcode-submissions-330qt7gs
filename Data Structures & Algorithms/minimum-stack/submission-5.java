class MinStack {

    private Deque<Integer> stack1;
    private Deque<Integer> stack2;

    public MinStack() {
        this.stack1=new ArrayDeque(); 
        this.stack2=new ArrayDeque(); 
    }
    
    public void push(int val) {
        int min=!stack2.isEmpty()?stack2.peek():val;
        min=Math.min(val, min);
        stack1.addFirst(val);
        stack2.addFirst(min);
    }
    
    public void pop() {
        stack1.removeFirst();
        stack2.removeFirst();
    }
    
    public int top() {
        return stack1.peekFirst();
    }
    
    public int getMin() {
        return stack2.peekFirst();   
    }
}
