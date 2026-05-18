class Solution {
    public int evalRPN(String[] tokens) {
        Deque<Integer> operands=new ArrayDeque();
        for(String c:tokens){
            switch(c){
                case "+":{
                    if(operands.size()==1){
                        int operand=operands.removeFirst();
                        operands.addFirst(operand+Integer.parseInt(c));
                    }else{
                        int operand1=operands.removeFirst();
                        int operand2=operands.removeFirst();
                        operands.addFirst(operand2+operand1);
                    }
                    break;
                }
                case "-":{
                    if(operands.size()==1){
                        int operand=operands.removeFirst();
                        operands.addFirst(operand-Integer.parseInt(c));
                    }else{
                        int operand1=operands.removeFirst();
                        int operand2=operands.removeFirst();
                        operands.addFirst(operand2-operand1);
                    }
                    break;
                }
                case "*":{
                    if(operands.size()==1){
                        int operand=operands.removeFirst();
                        operands.addFirst(operand*Integer.parseInt(c));
                    }else{
                        int operand1=operands.removeFirst();
                        int operand2=operands.removeFirst();
                        operands.addFirst(operand2*operand1);
                    }
                    break;
                }
                case "/":{
                    if(operands.size()==1){
                        int operand=operands.removeFirst();
                        operands.addFirst(operand/Integer.parseInt(c));
                    }else{
                        int operand1=operands.removeFirst();
                        int operand2=operands.removeFirst();
                        operands.addFirst(operand2/operand1);
                    }
                    break;
                }
                default:{
                    operands.addFirst(Integer.valueOf(c));
                    break;
                }
            }
        }
        return operands.removeFirst();
    }
}
