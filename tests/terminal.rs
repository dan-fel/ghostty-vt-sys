use std::ptr;

#[test]
fn terminal_lifecycle_and_viewport_scroll_bindings_are_callable() {
    let mut terminal: ghostty_vt_sys::GhosttyTerminal = ptr::null_mut();

    // SAFETY: The default allocator is selected with null, `terminal` is a valid out-pointer,
    // and both terminal dimensions are non-zero.
    let result = unsafe {
        ghostty_vt_sys::ghostty_terminal_new(
            ptr::null(),
            &mut terminal,
            ghostty_vt_sys::GhosttyTerminalOptions {
                cols: 5,
                rows: 2,
                max_scrollback: 10,
            },
        )
    };

    assert_eq!(result, ghostty_vt_sys::GHOSTTY_SUCCESS);
    assert!(!terminal.is_null());

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
