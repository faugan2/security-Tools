use std::{fs};
mod usage;

fn main() {
    let args:Vec<String>=std::env::args().collect();
    let args_size=args.len();

    let path="./";
    let entries=fs::read_dir(path);
    let entries=match entries{
        Ok(entries)=>{
            entries
        },
        Err(err)=>{
            panic!("Oups!!! Somthing wrong happened {:?}",err);
        }
    };

    let mut all_files=Vec::new();

    for entry in entries{
        let name=entry.unwrap();
        let name=name.file_name().to_os_string();
        let name=format!("{:?}",name).replace("\"", "");
        //let name=name.to_string_lossy();
        
        all_files.push(name);
        //println!("{}",name);
    }

   
   match args_size{
    1=>{
        for file in all_files{
            if !file.starts_with('.'){
                print!("{}\t",file);
            }
            
          }
    },
    2=>{
        let option=&args[1].as_str();
        match option{
            &"-l"=>{
                for file in all_files{
                    if !file.starts_with('.'){
                        println!("{}",file);
                    }
                    
                }
            },
            &"-a"=>{
                for file in all_files{
                    if file.starts_with('.'){
                        println!("{}",file);
                    }
                    
                }
            },
            &"-al" | &"-la"=>{
                for file in all_files{
                    println!("{}",file);
                }
            },
            _=>{
                usage::help();
            }
        };
    },
    _=>{
        usage::help();
    }
   }
   


}
