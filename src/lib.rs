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
    Config::build(arg_iter).search().into_iter().map(|result|{println!("{}",result)}).collect()
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



        let closure_query = |suffix:usize| self.query[..self.query.len()-suffix-1].to_string(); 



        if self.query.contains("-cis")&&self.query.contains("-fwm") {

            
            let mut query = " ".to_owned();
            query.push_str(&closure_query(8));
            query.push_str(" ");


            self.content
            .lines()
            .filter(|item|item.to_lowercase().contains(&query)||item.to_lowercase()==closure_query(8))
            .collect()


        }
        else if self.query.contains("-cis") {
            let query = closure_query(4);
            self.content
            .lines()
            .filter(|item|item.to_lowercase().contains(&query))
            .collect()

        }
        else if self.query.contains("-fwm") {
            let mut query = " ".to_owned();
            query.push_str(&closure_query(4));
            query.push_str(" ");

            self.content
            .lines()
            .filter(|item|item.contains(&query)||item==&&closure_query(4))
            .collect()

        }
        else {
            self.content
            .lines()
            .filter(|item|item.contains(&self.query))
            .collect::<Vec<_>>()
        }
    
    }
}
