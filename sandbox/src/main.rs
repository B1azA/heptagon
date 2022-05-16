mod game;

fn main() {
    let main_loop = heptagon::main_loop::MainLoop::new("Heptagon");
    let game = game::Game::new(&main_loop.window);
    main_loop.run(game);
}
