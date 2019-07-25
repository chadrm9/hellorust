use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // macros
    let xs = vec![1, 2, 3];
    println!("The list is: {:?}", xs);

    println!("This is information");
    eprintln!("This is an error! :(");

    // progress bar
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        // do hard work
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
    
    // print buffer
    let stdout = io::stdout(); // get the global stdout entity

    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    // let mut handle = stdout.lock(); // optional: aquire a lock on it

    writeln!(handle, "foo: {}", 42)?; // add `?` if you care about errors here
    Ok(())
}
