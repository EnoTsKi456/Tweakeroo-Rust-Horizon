fn tweakeroo_permanent_sneak(game: &crate::game::Game<'_>, sneak: &mut bool) {
    use crate::game::tweakeroo_rust as tweakeroo;
    if game.native_on(tweakeroo::ID)
        && tweakeroo::permanent_sneak(|key| game.native_setting(tweakeroo::ID, key))
    {
        *sneak = true;
    }
}
