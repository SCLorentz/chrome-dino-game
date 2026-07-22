import init, { Game } from "/pkg/chrome_dino_game.js";
await init()

const dino = new Game();

dino.resize_canvas(16.0 / 9.0);
dino.set_bg_color("black");

/*dino.new_text("Hello world", "red", "Roboto", "50.0");

window.addEventListener("click", () => {
    dino.update_sprite_value("buzz", "650.0", "250.0");
    dino.force_update();
})*/

window.addEventListener("resize", () => dino.resize_canvas());
