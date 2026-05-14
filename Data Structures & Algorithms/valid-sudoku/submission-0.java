class Solution {
    public boolean isValidSudoku(char[][] board) {
        boolean[][] reducedSudoku=new boolean[3][3];
        for(int i=0; i<9; i+=3){
            for(int j=0; j<9; j+=3){
                boolean checkSubCaseRes=checkSubCase(board, i, j);
                if(!checkSubCaseRes) return false;
            }
        }
        for(int i=0; i<9; i++){
            HashSet<Character> checkSet=new HashSet();
            for(int j=0; j<9; j++){
                if(board[i][j]!='.'){
                    if(checkSet.contains(board[i][j])) return false;
                    checkSet.add(board[i][j]);
                }
            }
        }
        for(int i=0; i<9; i++){
            HashSet<Character> checkSet=new HashSet();
            for(int j=0; j<9; j++){
                if(board[j][i]!='.'){
                    if(checkSet.contains(board[j][i])) return false;
                    checkSet.add(board[j][i]);
                }
            }
        }
        return true;
    }

    boolean checkSubCase(char[][] board, int i, int j){
        HashSet<Character> set=new HashSet();
        for(int k=i; k<i+3; k++){
            for(int l=j; l<j+3; l++){
                if(board[k][l]!='.'){
                    if(set.contains(board[k][l])) return false;
                    set.add(board[k][l]);
                }
            }
        }
        return true;
    }

}
