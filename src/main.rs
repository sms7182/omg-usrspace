use std::env::args;
use std::env::var;
use std::process::exit;
use std::io::stdout;
use std::io::stderr;



fn main() {
   
  let args:Vec<String>=args().collect();
  if args.len()==1{
      println!("i'm run without args");
    }else {
        for arg in args{
            println!("{}",arg);
        }
    }
  match var("HOME"){
    Ok(h_d)=>{
        println!("Home directory :{}",h_d);
        exit(0);
    }Err(e)=>{
        eprintln!("Error reading HOME environment variable:{}",e);
        exit(1);
    }
  }

  
}
