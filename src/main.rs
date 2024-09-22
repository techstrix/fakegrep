use anyhow::Context;
use clap::Parser;


#[derive(Parser)]
struct Cli{
    pattern:String,
    path:std::path::PathBuf
}

//Forms of errors in rust
#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(),Box<dyn std::error::Error>> {
    let args = Cli::parse();
    // let content  = std::fs::read_to_string(&args.path).map_err(|err| CustomError(format!("Error reading `{}`: {}", &args.path.display(), err)))?;
    //Anyhow offers better and more concise formatting for the errors
    let content  = std::fs::read_to_string(&args.path).with_context(|| format!("could not read file `{}`", &args.path.display()))?; 

    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}",line);
        }
    }
    Ok(())
}
