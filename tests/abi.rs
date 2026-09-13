use std::ffi::CStr;
use std::mem::{align_of, offset_of, size_of};

#[test]
fn embedding_layouts_match_the_native_library() {
    // SAFETY: The library returns a static, null-terminated manifest.
    let manifest = unsafe {
        let pointer = ghostty_vt_sys::ghostty_type_json();
        assert!(!pointer.is_null());
        CStr::from_ptr(pointer)
    };
    let manifest: serde_json::Value = serde_json::from_slice(manifest.to_bytes()).unwrap();
    assert_eq!(manifest["schema"], 1);
    let types = &manifest["types"];

    // Bindgen checks Rust against the C headers at compile time. Check against
    // Zig's compiled layouts too, so a header/library mismatch fails validation.
    macro_rules! check_layout {
        ($name:ident { $($field:ident),* $(,)? }) => {{
            let layout = &types[stringify!($name)];
            assert_eq!(layout["size"], size_of::<ghostty_vt_sys::$name>(),
                "{} size", stringify!($name));
            assert_eq!(layout["align"], align_of::<ghostty_vt_sys::$name>(),
                "{} alignment", stringify!($name));
            $(assert_eq!(layout["fields"][stringify!($field)]["offset"],
                offset_of!(ghostty_vt_sys::$name, $field),
                "{}::{} offset", stringify!($name), stringify!($field));)*
        }};
    }

    check_layout!(GhosttyTerminalModeConfig { mode, value });
    check_layout!(GhosttyTerminalScrollViewport { tag, value });
    check_layout!(GhosttyTerminalScrollViewportValue { delta, row });
    check_layout!(GhosttyTerminalScrollbar { total, offset, len });
    check_layout!(GhosttyMousePosition { x, y });
    check_layout!(GhosttyMouseEncoderSize {
        size,
        screen_width,
        screen_height,
        cell_width,
        cell_height,
        padding_top,
        padding_bottom,
        padding_right,
        padding_left,
    });
    check_layout!(GhosttyColorRgb { r, g, b });
    check_layout!(GhosttyStyleColorValue { palette, rgb });
    check_layout!(GhosttyStyleColor { tag, value });
    check_layout!(GhosttyStyle {
        size,
        fg_color,
        bg_color,
        underline_color,
        bold,
        italic,
        faint,
        blink,
        inverse,
        invisible,
        strikethrough,
        overline,
        underline,
    });
    check_layout!(GhosttyRenderStateCursor {
        size,
        viewport_has_value,
        viewport_x,
        viewport_y,
        wide_tail,
        visible,
        blinking,
        password_input,
        visual_style,
    });

    for (name, value) in [
        (
            "SCROLLBACK_MAX_BYTES",
            ghostty_vt_sys::GHOSTTY_TERMINAL_OPT_SCROLLBACK_MAX_BYTES,
        ),
        ("MODE", ghostty_vt_sys::GHOSTTY_TERMINAL_OPT_MODE),
    ] {
        assert_eq!(types["GhosttyTerminalOption"]["values"][name], value);
    }
    assert_eq!(
        types["GhosttyTerminalData"]["values"]["MODE"],
        ghostty_vt_sys::GHOSTTY_TERMINAL_DATA_MODE
    );
    assert_eq!(
        types["GhosttyTerminalScrollViewportTag"]["values"]["ROW"],
        ghostty_vt_sys::GHOSTTY_SCROLL_VIEWPORT_ROW
    );
    assert_eq!(
        types["GhosttyRenderStateData"]["values"]["CURSOR"],
        ghostty_vt_sys::GHOSTTY_RENDER_STATE_DATA_CURSOR
    );
}
