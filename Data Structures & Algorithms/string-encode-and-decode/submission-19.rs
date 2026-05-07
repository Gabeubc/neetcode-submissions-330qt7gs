impl Solution {

    const SEP:char=';';

    pub fn encode(strs: Vec<String>) -> String {
        let mut code=String::new();
        for s in strs{
            code.push_str(&format!("{}{}{}", s.len(), Self::SEP, s));
        }
        return code;
    }

    pub fn decode(s: String) -> Vec<String> {
        println!("{}",s);
        let len=s.len();
        let mut start=0;
        let mut res:Vec<String>=Vec::new();
        while start<len{
            if let Some(pos_sep)=&s[start..len].find(Self::SEP){
                let pos_sep=*pos_sep as usize;
                if let Ok(size_to_read)=(&s[start..start+pos_sep]).parse::<usize>(){
                    let from=start+pos_sep+1;
                    let end=from+size_to_read;
                    let item=&s[from..end].to_string();
                    res.push(item.to_string());
                    start=end;
                }else{
                    break;
                }
            }else{
                break;
            }
        }
        return res;
    }

}
