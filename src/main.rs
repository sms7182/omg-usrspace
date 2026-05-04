use std::env::args;
use std::env::var;
use std::process::exit;
use std::io::stdout;
use std::io::stderr;
use std::fs;

struct Command{
  name:String,
  args:Vec<String>,
  background:bool,
  
}
impl Command {
   fn new(self)->Self {
      let cm= Command{
        args:vec![],
        background:false,
        name:"cd".to_string(),
       };
       return cm;
   } 
}

fn main() {


  loop {
      print!("gosh>");
      use std::io::Write;
      stdout().flush().unwrap();
      let mut input=String::new();
      std::io::stdin().read_line(&mut input).unwrap();
      let parts:Vec<&str>=input.trim().split_whitespace().collect();
      if parts.len()==0{
        continue;
      }
      match parts[0]{
        "exit"=>break,
        "ls"=>ls_command(&parts[1..]),
        "pwd"=>println!("gosh> pwd is loading"),
        "cd"=>println!("gosh> cd is loading"),
        _=>execute_external(parts[0],&parts[1..]),
        

      }

  }
   
  
}

fn execute_external(f_param:&str,oth:&[&str]){

}

fn green(text: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", text)
}

fn red(text: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", text)
}
fn ls_command(command_parts:&[&str]){
  let mut path=".";
  if command_parts.len()>0{
    
    path=command_parts[1];

  }

 
  match fs::read_dir(path){
    Ok(entries)=>{
      for entry in entries{
        match entry{
          Ok(entry)=>{
            let pt=entry.path();
            let file_name=pt.file_name().unwrap_or_default();
            let file_type= if pt.is_dir(){"Dir"} else {
                "File"
            };
             let name_str = file_name.to_string_lossy();
             if file_type == "Dir" {
                            println!("{}", green(&name_str));
                        } else {
                            println!("{}", red(&name_str));
                        }
          }
          Err(e)=>eprintln!("error reading content")
        }
      }
    }
    Err(e)=>eprintln!("Error reading '{}':{}",path,e),
  }
}

