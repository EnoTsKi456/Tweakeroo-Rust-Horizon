fn tweakeroo_zoom_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::tweakeroo_rust as tweakeroo;
    if !game.native_on(tweakeroo::ID) {
        tweakeroo::reset_zoom();
        return;
    }
    let bound = game.native_mod_key(tweakeroo::ID, tweakeroo::KEY, tweakeroo::DEFAULT_KEY);
    if key != bound {
        return;
    }
    let playing = game.native_playing();
    let game = &*game;
    tweakeroo::key(playing, pressed, |name| game.native_setting(tweakeroo::ID, name));
}
