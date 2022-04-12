use warp::Filter;

#[tokio::main]
async fn main() {
    println!("Running server...");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("service" / String)
        .map(|name| {
            println!("Kiel service request received!");
            format!("Kiel says hello, {}!", name)
        });
    let world = warp::path!("world" / String)
        .map(|name| format!("{}, world!", name));
    
    let routes = warp::get().and(
        hello
        .or(world)
    );

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8196))
        .await;
}
