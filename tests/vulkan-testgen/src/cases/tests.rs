use super::*;

fn traced(handle: Option<u32>, call: &str) -> Traced {
    Traced {
        handle,
        call: call.parse().unwrap(),
    }
}

#[test]
fn the_exported_calls_are_those_of_the_last_exported_handle() {
    let calls = vec![
        traced(Some(1), "i915_gem_create size=4096"),
        traced(Some(1), "prime_handle_to_fd flags=0x80002"),
        traced(Some(2), "i915_gem_create size=8192"),
        traced(None, "i915_gem_create size=1"),
        traced(Some(2), "prime_handle_to_fd flags=0x80002"),
    ];
    let exported: Vec<String> = exported_calls(calls)
        .iter()
        .map(|call| call.to_string())
        .collect();
    assert_eq!(
        exported,
        [
            "i915_gem_create size=8192",
            "prime_handle_to_fd flags=0x80002"
        ]
    );
}

#[test]
#[should_panic(expected = "no export recorded")]
fn a_trace_without_an_export_is_refused() {
    exported_calls(vec![traced(Some(1), "i915_gem_create size=4096")]);
}

#[test]
fn the_vulkan_flags_are_the_nine_relaxations() {
    let flags = vulkan_flags();
    for flag in VULKAN_FLAGS {
        assert!(flags.contains(flag), "{flag:?}");
    }
    assert!(!flags.contains(Flag::Linear));
    assert!(!flags.contains(Flag::ExplicitSync));
}

#[test]
fn no_flags_are_the_default() {
    assert_eq!(flags(&[]), Flags::default());
}
