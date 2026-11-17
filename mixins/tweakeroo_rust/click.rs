fn tweakeroo_rust_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::tweakeroo_rust as tweakeroo;
    if mod_id != tweakeroo::ID { return; }
    let mut settings = tweakeroo::settings(|key| game.native_setting(tweakeroo::ID, key));
    settings.click(button);
    tweakeroo::set_settings(settings);
    for (key, value) in settings.stored() {
        game.native_set_setting(tweakeroo::ID, key, &value);
    }
}
