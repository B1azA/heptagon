use heptagon::main_loop::*;

mod game;

fn main() {
    let main_loop = MainLoop::new();
    main_loop.run(game::Game::new());
}
