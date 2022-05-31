mod game;

fn main() {
    let main_loop = heptagon::main_loop::MainLoop::new("Heptagon");
    let render_belongings = main_loop.get_render_belongings();
    let renderer = heptagon::rendering::renderer::Renderer::custom_new(render_belongings.0, render_belongings.1, 
        render_belongings.2, render_belongings.3);
    let game = game::Game::new(&main_loop.window, renderer);
    main_loop.run(game);
}
