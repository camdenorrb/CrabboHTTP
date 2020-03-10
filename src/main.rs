mod http;
mod server;

fn main() {
    async_std::task::block_on(server::main()).expect("Failed to start server?");
    println!("Complete");
}
