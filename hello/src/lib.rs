/* Count distinc anagrams*/

/* Accepts a string list.
Calculate the number anagrams in the file
*/


pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn back_track(s: String, open: i32, close: i32) -> Vec<String> {
        let mut res = vec![];
        if open == 0 && close == 0 {
            return vec![s];
        }
        if open > 0 {
            res.append(&mut back_track(s.clone()+"(", open-1, close+1));
        }
        if close > 0 {
            res.append(&mut back_track(s.clone()+")", open, close-1));
        }
        res
    }
    back_track("".to_string(), n, 0)
    
}