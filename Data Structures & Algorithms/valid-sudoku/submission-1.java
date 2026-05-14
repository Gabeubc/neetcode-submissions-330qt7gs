class Solution {
    public boolean isValidSudoku(char[][] board) {
        for(int i=0; i<9; i+=3){
            for(int j=0; j<9; j+=3){
                boolean checkSubCaseRes=checkSubCase(board, i, j);
                if(!checkSubCaseRes) return false;
            }
        }
        for(int i=0; i<9; i++){
            HashSet<Character> checkSetRow=new HashSet();
            HashSet<Character> checkSetCol=new HashSet();
            for(int j=0; j<9; j++){
                if(board[i][j]!='.' || board[j][i]!='.'){
                    if(checkSetRow.contains(board[i][j]) || checkSetCol.contains(board[j][i])) return false;
                    if(board[i][j]!='.') checkSetRow.add(board[i][j]);
                    if(board[j][i]!='.') checkSetCol.add(board[j][i]);
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
