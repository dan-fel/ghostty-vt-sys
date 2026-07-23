use std::ffi::c_void;
use std::ptr;

fn create_terminal(
    options: ghostty_vt_sys::GhosttyTerminalOptions,
) -> ghostty_vt_sys::GhosttyTerminal {
    let mut terminal: ghostty_vt_sys::GhosttyTerminal = ptr::null_mut();

    // SAFETY: The default allocator is selected with null, `terminal` is a valid out-pointer,
    // and callers provide non-zero terminal dimensions.
    let result =
        unsafe { ghostty_vt_sys::ghostty_terminal_new(ptr::null(), &mut terminal, options) };

    assert_eq!(result, ghostty_vt_sys::GHOSTTY_SUCCESS);
    assert!(!terminal.is_null());
    terminal
}

#[test]
fn terminal_lifecycle_and_viewport_scroll_bindings_are_callable() {
    let terminal = create_terminal(ghostty_vt_sys::GhosttyTerminalOptions {
        cols: 5,
        rows: 2,
        max_scrollback: 10,
    });

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
fn write_pty_callback_receives_terminal_query_response() {
    let terminal = create_terminal(ghostty_vt_sys::GhosttyTerminalOptions {
        cols: 1,
        rows: 1,
        max_scrollback: 0,
    });
    let mut response = Vec::<u8>::new();
    let callback: ghostty_vt_sys::GhosttyTerminalWritePtyFn = capture_pty_write;

    // SAFETY: `terminal` is live. `response` remains at a stable address until after the
    // synchronous VT write, and `callback` uses the exact C calling convention.
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
                callback as *const () as *const c_void,
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
