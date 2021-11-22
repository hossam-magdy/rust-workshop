use std::net::TcpListener;

use web_server2::handle_request::handle_request;
use web_server2::thread_pool::ThreadPool;

static ADDRESS: &str = "127.0.0.1:7878";

fn main() {
    let tcp_listener = TcpListener::bind(ADDRESS).unwrap();

    for stream in tcp_listener.incoming() {
        let mut stream = stream.unwrap();
        let pool = ThreadPool::new(4);

        // do we "move" in thread::spawn?
        pool.execute(move || {
            handle_request(&mut stream);
        });
    }
}
