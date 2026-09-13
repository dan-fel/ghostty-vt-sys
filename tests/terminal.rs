use std::ffi::c_void;
use std::ptr;

fn create_terminal(
    cols: u16,
    rows: u16,
    max_scrollback_bytes: usize,
) -> ghostty_vt_sys::GhosttyTerminal {
    let mut terminal: ghostty_vt_sys::GhosttyTerminal = ptr::null_mut();

    // SAFETY: The default allocator is selected with null, `terminal` is a valid out-pointer,
    // and callers provide non-zero terminal dimensions.
    let result =
        unsafe { ghostty_vt_sys::ghostty_terminal_new(ptr::null(), &mut terminal, cols, rows) };

    assert_eq!(result, ghostty_vt_sys::GHOSTTY_SUCCESS);
    assert!(!terminal.is_null());
    // SAFETY: The terminal is live and this option borrows a size_t for the call.
    unsafe {
        assert_eq!(
            ghostty_vt_sys::ghostty_terminal_set(
                terminal,
                ghostty_vt_sys::GHOSTTY_TERMINAL_OPT_SCROLLBACK_MAX_BYTES,
                ptr::from_ref(&max_scrollback_bytes).cast(),
            ),
            ghostty_vt_sys::GHOSTTY_SUCCESS,
        );
    }
    terminal
}

#[test]
fn terminal_lifecycle_and_viewport_scroll_bindings_are_callable() {
    let terminal = create_terminal(5, 2, 10);

    let output = b"one\r\ntwo\r\nthree";
    // SAFETY: `terminal` is live, and `output` remains valid for the duration of the call.
    unsafe {
        ghostty_vt_sys::ghostty_terminal_vt_write(terminal, output.as_ptr(), output.len());
    }

    for behavior in [
        ghostty_vt_sys::GhosttyTerminalScrollViewport {
            tag: ghostty_vt_sys::GHOSTTY_SCROLL_VIEWPORT_TOP,
            value: ghostty_vt_sys::GhosttyTerminalScrollViewportValue { delta: 0 },
        },
        ghostty_vt_sys::GhosttyTerminalScrollViewport {
            tag: ghostty_vt_sys::GHOSTTY_SCROLL_VIEWPORT_BOTTOM,
            value: ghostty_vt_sys::GhosttyTerminalScrollViewportValue { delta: 0 },
        },
        ghostty_vt_sys::GhosttyTerminalScrollViewport {
            tag: ghostty_vt_sys::GHOSTTY_SCROLL_VIEWPORT_DELTA,
            value: ghostty_vt_sys::GhosttyTerminalScrollViewportValue { delta: -1 },
        },
    ] {
        // SAFETY: `terminal` is live and `behavior` matches Ghostty's tagged-union ABI.
        unsafe {
            ghostty_vt_sys::ghostty_terminal_scroll_viewport(terminal, behavior);
        }
    }

    // SAFETY: Successful creation returned this owned terminal handle, which has not been freed.
    unsafe {
        ghostty_vt_sys::ghostty_terminal_free(terminal);
    }
}

#[test]
fn terminal_state_and_mouse_encoder_bindings_are_callable() {
    let terminal = create_terminal(5, 2, 10);
    let output = b"one\r\ntwo\r\nthree\x1b[?1000h\x1b[?1006h";

    // SAFETY: `terminal` is live, and `output` remains valid for the call.
    unsafe {
        ghostty_vt_sys::ghostty_terminal_vt_write(terminal, output.as_ptr(), output.len());
    }

    let mut screen = ghostty_vt_sys::GHOSTTY_TERMINAL_SCREEN_PRIMARY;
    let mut scrollbar = ghostty_vt_sys::GhosttyTerminalScrollbar {
        total: 0,
        offset: 0,
        len: 0,
    };
    let mut mouse_tracking = false;
    let mut alternate_scroll = ghostty_vt_sys::GhosttyTerminalModeConfig {
        mode: 1007, // DEC private alternate-scroll mode.
        value: false,
    };

    // SAFETY: Every output pointer matches the type documented for its queried data or mode.
    unsafe {
        assert_eq!(
            ghostty_vt_sys::ghostty_terminal_get(
                terminal,
                ghostty_vt_sys::GHOSTTY_TERMINAL_DATA_ACTIVE_SCREEN,
                ptr::from_mut(&mut screen).cast(),
            ),
            ghostty_vt_sys::GHOSTTY_SUCCESS
        );
        assert_eq!(
            ghostty_vt_sys::ghostty_terminal_get(
                terminal,
                ghostty_vt_sys::GHOSTTY_TERMINAL_DATA_SCROLLBAR,
                ptr::from_mut(&mut scrollbar).cast(),
            ),
            ghostty_vt_sys::GHOSTTY_SUCCESS
        );
        assert_eq!(
            ghostty_vt_sys::ghostty_terminal_get(
                terminal,
                ghostty_vt_sys::GHOSTTY_TERMINAL_DATA_MOUSE_TRACKING,
                ptr::from_mut(&mut mouse_tracking).cast(),
            ),
            ghostty_vt_sys::GHOSTTY_SUCCESS
        );
        assert_eq!(
            ghostty_vt_sys::ghostty_terminal_get(
                terminal,
                ghostty_vt_sys::GHOSTTY_TERMINAL_DATA_MODE,
                ptr::from_mut(&mut alternate_scroll).cast(),
            ),
            ghostty_vt_sys::GHOSTTY_SUCCESS
        );
    }

    assert_eq!(screen, ghostty_vt_sys::GHOSTTY_TERMINAL_SCREEN_PRIMARY);
    assert_eq!(scrollbar.len, 2);
    assert!(scrollbar.total >= scrollbar.offset + scrollbar.len);
    assert!(mouse_tracking);
    assert!(alternate_scroll.value);

    let mut event = ptr::null_mut();
    let mut encoder = ptr::null_mut();
    // SAFETY: Both values are valid out-pointers and null selects Ghostty's default allocator.
    unsafe {
        assert_eq!(
            ghostty_vt_sys::ghostty_mouse_event_new(ptr::null(), &mut event),
            ghostty_vt_sys::GHOSTTY_SUCCESS
        );
        assert_eq!(
            ghostty_vt_sys::ghostty_mouse_encoder_new(ptr::null(), &mut encoder),
            ghostty_vt_sys::GHOSTTY_SUCCESS
        );
    }
    assert!(!event.is_null());
    assert!(!encoder.is_null());

    let encoder_size = ghostty_vt_sys::GhosttyMouseEncoderSize {
        size: std::mem::size_of::<ghostty_vt_sys::GhosttyMouseEncoderSize>(),
        screen_width: 50,
        screen_height: 40,
        cell_width: 10,
        cell_height: 20,
        padding_top: 0,
        padding_bottom: 0,
        padding_right: 0,
        padding_left: 0,
    };
    let mut encoded = [0_i8; 32];
    let mut encoded_len = 0;

    // SAFETY: The event and encoder are live, the option pointer has the documented size type,
    // and the output buffer remains writable for its declared length.
    let result = unsafe {
        ghostty_vt_sys::ghostty_mouse_event_set_action(
            event,
            ghostty_vt_sys::GHOSTTY_MOUSE_ACTION_PRESS,
        );
        ghostty_vt_sys::ghostty_mouse_event_set_button(
            event,
            ghostty_vt_sys::GHOSTTY_MOUSE_BUTTON_FOUR,
        );
        ghostty_vt_sys::ghostty_mouse_event_set_position(
            event,
            ghostty_vt_sys::GhosttyMousePosition { x: 0.0, y: 0.0 },
        );
        ghostty_vt_sys::ghostty_mouse_encoder_setopt(
            encoder,
            ghostty_vt_sys::GHOSTTY_MOUSE_ENCODER_OPT_SIZE,
            ptr::from_ref(&encoder_size).cast(),
        );
        ghostty_vt_sys::ghostty_mouse_encoder_setopt_from_terminal(encoder, terminal);
        ghostty_vt_sys::ghostty_mouse_encoder_encode(
            encoder,
            event,
            encoded.as_mut_ptr(),
            encoded.len(),
            &mut encoded_len,
        )
    };

    assert_eq!(result, ghostty_vt_sys::GHOSTTY_SUCCESS);
    let encoded = encoded[..encoded_len]
        .iter()
        .map(|byte| *byte as u8)
        .collect::<Vec<_>>();
    assert_eq!(encoded, b"\x1b[<64;1;1M");

    // SAFETY: Each handle is still live and freed exactly once.
    unsafe {
        ghostty_vt_sys::ghostty_mouse_encoder_free(encoder);
        ghostty_vt_sys::ghostty_mouse_event_free(event);
        ghostty_vt_sys::ghostty_terminal_free(terminal);
    }
}

#[test]
fn write_pty_callback_receives_terminal_query_response() {
    let terminal = create_terminal(1, 1, 0);
    let mut response = Vec::<u8>::new();

    // SAFETY: `terminal` is live. `response` remains at a stable address until after the
    // synchronous VT write, and `capture_pty_write` uses the exact C calling convention.
    unsafe {
        assert_eq!(
            ghostty_vt_sys::ghostty_terminal_set(
                terminal,
                ghostty_vt_sys::GHOSTTY_TERMINAL_OPT_USERDATA,
                ptr::from_mut(&mut response).cast(),
            ),
            ghostty_vt_sys::GHOSTTY_SUCCESS
        );
        assert_eq!(
            ghostty_vt_sys::ghostty_terminal_set(
                terminal,
                ghostty_vt_sys::GHOSTTY_TERMINAL_OPT_WRITE_PTY,
                capture_pty_write as *const () as *const c_void,
            ),
            ghostty_vt_sys::GHOSTTY_SUCCESS
        );

        let query = b"\x1b[?7$p";
        ghostty_vt_sys::ghostty_terminal_vt_write(terminal, query.as_ptr(), query.len());
    }

    assert_eq!(response, b"\x1b[?7;1$y");

    // SAFETY: `terminal` remains live and owned by this test.
    unsafe {
        ghostty_vt_sys::ghostty_terminal_free(terminal);
    }
}

unsafe extern "C" fn capture_pty_write(
    _terminal: ghostty_vt_sys::GhosttyTerminal,
    userdata: *mut c_void,
    data: *const u8,
    length: usize,
) {
    if length == 0 {
        return;
    }

    // SAFETY: The test registered a live `Vec<u8>` as userdata, and Ghostty guarantees that
    // non-empty callback data is valid for the duration of this call.
    let response = unsafe { &mut *userdata.cast::<Vec<u8>>() };
    let data = unsafe { std::slice::from_raw_parts(data, length) };
    response.extend_from_slice(data);
}
