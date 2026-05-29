class Solution {
    public boolean isValidSudoku(char[][] board) {
        int r_len=board.length;
        int c_len=board[0].length;
        for(int i=0; i<r_len; i+=3){
            for(int j=0; j<c_len; j+=3){
                int R=i+3;
                int C=j+3;
                Set<Character> set=new HashSet<>();
                for(int k=i; k<R; k++){
                    for(int l=j; l<C; l++){
                        if(board[k][l]!='.' && set.contains(board[k][l])) return false;
                        set.add(board[k][l]);
                    }
                }
            }
        }
        for(int i=0; i<r_len; i++){
            Set<Character> r_set=new HashSet<>();
            Set<Character> c_set=new HashSet<>();
            for(int j=0; j<c_len; j++){
                if((board[i][j]!='.' && r_set.contains(board[i][j]))|| 
                (board[j][i]!='.' && c_set.contains(board[j][i]))) return false;
                if(board[i][j]!='.') r_set.add(board[i][j]);
                if(board[j][i]!='.') c_set.add(board[j][i]);
            }
        }
        return true;
    }
}
