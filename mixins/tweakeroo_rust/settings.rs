fn tweakeroo_rust_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::tweakeroo_rust as tweakeroo;
    if mod_id != tweakeroo::ID { return; }
    let game = &*game;
    let settings = tweakeroo::settings(|key| game.native_setting(tweakeroo::ID, key));
    buttons.extend(settings.labels());
}
