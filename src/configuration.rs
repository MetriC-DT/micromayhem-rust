use ggez::conf::Conf;
use ggez::ContextBuilder;
use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;

pub fn load_configuration(cb: ContextBuilder) -> ContextBuilder {
    // tries to read any user configuration, if it exists.
    // if it does not exist, or errors when trying to read,
    // then just load the default configurations.
    let default_window_mode = WindowMode {
        width: 1920.0,
        height: 1080.0,
        maximized: true,
        fullscreen_type: ggez::conf::FullscreenType::True,
        borderless: true,
        min_width: 0.0,
        min_height: 0.0,
        max_width: 0.0,
        max_height: 0.0,
        resizable: false,
        visible: true,
        resize_on_scale_factor_change: false,
    };

    let default_window_setup = WindowSetup {
        title: crate::GAME_TITLE.to_owned(),
        samples: ggez::conf::NumSamples::One,
        vsync: true,
        icon: crate::ICON_PATH.to_owned(),
        srgb: true,
    };

    // builds the default configuration
    let default_config = Conf {
        window_mode: default_window_mode,
        window_setup: default_window_setup,
        backend: ggez::conf::Backend::default(),
        modules: ggez::conf::ModuleConf::default(),
    };

    return cb.default_conf(default_config);
}
