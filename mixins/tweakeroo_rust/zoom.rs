fn tweakeroo_zoom_view(game: &crate::game::Game<'_>, zoom: &mut Option<crate::game::native_hooks::NativeZoom>) {
    use crate::game::tweakeroo_rust as tweakeroo;
    if zoom.is_none()
        && game.native_on(tweakeroo::ID)
        && let Some((factor, smooth, turn)) = tweakeroo::view()
    {
        *zoom = Some(crate::game::native_hooks::NativeZoom { factor, smooth, turn });
    }
}
