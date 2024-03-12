use std::{env,fmt::Error,fs,process};
// use colored::Colorize;
struct Config {query:String,filename:String,content:String}
pub fn run() {
    let args:Vec<String> = env::args().collect();
    if let true = args.len() < 3{
        eprint!("{}: {},{}    ","Please Input Two Parameters","KeyWord","Filename");
        process::exit(1);
    }
    
    let arg_iter = args.into_iter();
    let mut config = Config::build(arg_iter);
    let results = config.search();

    for item in results.into_iter(){
        println!("{item}")
    }
}
impl Config {
    fn build(mut arg_iter:impl Iterator<Item = String>) -> Self{
        arg_iter.next();

        let query = arg_iter.next().unwrap_or_else(||{eprint!("There is an error because of {}",Error.to_string());process::exit(1)});
        let filename = arg_iter.next().unwrap_or_else(||{eprint!("There is an error because of {}",Error.to_string());process::exit(1)});
        let content = fs::read_to_string(&filename).unwrap_or_else(|err|{eprint!("There is an error because of {}",err.to_string());process::exit(1)});
        
        Self {query,filename,content}
    }

    fn search<'l>(&'l mut self) -> Vec<&'l str> {
        let closure_query = |suffix:usize| self.query[..self.query.len()-suffix].to_string();  
        if self.query.contains("-cis")&&self.query.contains("-fwm") {
            let mut query = " ".to_owned();
            query.push_str(&closure_query(8));
            query.push_str(" ");
            let results:Vec<_> = self.content
            .lines()
            .filter(|item|item.to_lowercase().contains(&query)||item.to_lowercase()==closure_query(8))
            .collect();
            return results

        }
        else if self.query.contains("-cis") {
            let query = closure_query(8);
            let results:Vec<_> =  self.content
            .lines()
            .filter(|item|item.to_lowercase().contains(&query))
            .collect();
            return results

        }
        else if self.query.contains("-fwm") {
            let mut query = " ".to_owned();
            query.push_str(&closure_query(8));
            query.push_str(" ");

            let results:Vec<_> = self.content
            .lines()
            .filter(|item|item.contains(&query)||item==&&closure_query(8))
            .collect();
            return results

        }
        else {
            let results:Vec<_> =  self.content
            .lines()
            .filter(|item|item.contains(&self.query))
            .collect();
            results
        }
    
    }
}
//     fn judge_query(&self) -> () {
//     if self.query.contains("-cis")&&self.query.contains("-fwm") {

//     }
//     else if self.query.contains("-cis") {
        
//     }
//     else if self.query.contains("-fwm") {
        
//     }
//     else {
        
//     }



//     }
    

// }
// fn full_world_match(query:&str) ->() {

    
// }


// fn case_insensitive(query:&str) -> () {


// }

