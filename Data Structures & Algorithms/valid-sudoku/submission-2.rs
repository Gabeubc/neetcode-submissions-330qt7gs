impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in (0..9).step_by(3){
            for j in (0..9).step_by(3){
                if !Self::is_sub_sudoku_valid(&board, i, j){
                    return false;
                }
            }
        }
        for i in 0..9{
            let mut col_set:HashSet<char>=HashSet::new();
            let mut row_set:HashSet<char>=HashSet::new();
            for j in 0..9{
                if board[i][j]!='.' || board[j][i]!='.'{
                    if row_set.contains(&board[i][j]) || col_set.contains(&board[j][i]){
                        return false;
                    }
                    if board[i][j]!='.' {row_set.insert(board[i][j]);}
                    if board[j][i]!='.' {col_set.insert(board[j][i]);}
                }
            }
        }
        return true;
    }

    fn is_sub_sudoku_valid(board:&Vec<Vec<char>>, i:usize, j:usize) -> bool{
        let mut checkSubSet:HashSet<char>=HashSet::new();
        for k in i..i+3{
            for l in j..j+3{
                if board[k][l]!='.' {
                    if checkSubSet.contains(&board[k][l]){
                        return false;
                    }
                    checkSubSet.insert(board[k][l]);
                }
            }
        }
        return true;
    }
}
