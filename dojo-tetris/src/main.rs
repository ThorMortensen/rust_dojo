fn hello_tetris() {
    println!("Hello, tetris");
}


fn main() {
    env_logger::init(); // Must be here for error msg form wgpu

    hello_tetris();
}
