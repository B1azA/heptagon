mod game;

fn main() {
    let main_loop = heptagon::main_loop::MainLoop::new("Heptagon");
    let bundle = heptagon::rendering::bundle::Bundle::new(&main_loop.window);
    let game = game::Game::new(&main_loop.window, bundle);
    main_loop.run(game);
}
