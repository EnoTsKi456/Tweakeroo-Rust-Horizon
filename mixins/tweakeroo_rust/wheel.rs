fn tweakeroo_zoom_wheel(game: &crate::game::Game<'_>, steps: f32, claimed: &mut bool) {
    use crate::game::tweakeroo_rust as tweakeroo;
    if game.native_on(tweakeroo::ID) && tweakeroo::wheel(steps) {
        *claimed = true;
    }
}
