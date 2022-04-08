use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("service" / String)
        .map(|name| format!("Kiel says hello, {}!", name));
    let world = warp::path!("world" / String)
        .map(|name| format!("{}, world!", name));
    
    let routes = warp::get().and(
        hello
        .or(world)
    );

    warp::serve(routes)
        .run(([127, 0, 0, 1], 1996))
        .await;
}
