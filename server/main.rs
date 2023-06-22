use warp::Filter;
use std::process::Command;

fn execute_hour() {
    let output = Command::new("/usr/local/bin/kiel")
        .arg("hour-force")
        .arg("--enact")
        .output();
    match output {
        Err(e) => {
            eprintln!("{}", e);
        }
        Ok(out) => println!("{:?}", out),
        // _ => (),
    }
}

#[tokio::main]
async fn main() {
    println!("Running server...");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("service" / String)
        .map(|name| {
            println!("Kiel service request received!");
            format!("Kiel says hello, {}!", name)
        });
    let world = warp::path!("hour")
        .map(|| {
            println!("Hour executed");
            execute_hour();
            format!("Hour executed")
        });
    
    let routes = warp::get().and(
        hello
        .or(world)
    );

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8196))
        .await;
}
