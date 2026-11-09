use super::*;

fn create_ext(regions: Option<Vec<Region>>, pat_index: Option<u32>) -> Call {
    Call::I915GemCreateExt {
        size: 4096,
        flags: 1,
        regions,
        pat_index,
    }
}

const SYSTEM: Region = Region {
    memory_class: 0,
    memory_instance: 0,
};
const DEVICE: Region = Region {
    memory_class: 1,
    memory_instance: 0,
};

#[test]
fn calls_are_shown_with_their_fields() {
    let call = Call::AmdgpuGemCreate {
        size: 262144,
        alignment: 4096,
        domains: 4,
        flags: 0x84,
    };
    let text = "amdgpu_gem_create size=262144 alignment=4096 domains=0x4 flags=0x84";
    assert_eq!(call.to_string(), text);
    assert_eq!(text.parse(), Ok(call));
}

#[test]
fn absent_fields_are_not_shown() {
    let text = "i915_gem_create_ext size=4096 flags=0x1";
    assert_eq!(create_ext(None, None).to_string(), text);
    assert_eq!(text.parse(), Ok(create_ext(None, None)));
    let text = "i915_gem_create_ext size=4096 flags=0x1 regions=1:0,0:0 pat_index=3";
    let call = create_ext(Some(vec![DEVICE, SYSTEM]), Some(3));
    assert_eq!(call.to_string(), text);
    assert_eq!(text.parse(), Ok(call));
    let metadata = Call::AmdgpuGemMetadata {
        flags: 0,
        tiling_info: 0x10,
        data: Vec::new(),
    };
    let text = "amdgpu_gem_metadata flags=0x0 tiling_info=0x10";
    assert_eq!(metadata.to_string(), text);
    assert_eq!(text.parse(), Ok(metadata));
}

#[test]
fn metadata_words_are_hexadecimal() {
    let metadata = Call::AmdgpuGemMetadata {
        flags: 0,
        tiling_info: 0,
        data: vec![1, 0xdead_beef],
    };
    let text = "amdgpu_gem_metadata flags=0x0 tiling_info=0x0 data=00000001,deadbeef";
    assert_eq!(metadata.to_string(), text);
    assert_eq!(text.parse(), Ok(metadata));
}

#[test]
fn fields_are_listed_in_order() {
    let fields = create_ext(None, Some(3)).fields();
    let keys: Vec<&str> = fields.iter().map(|field| field.key).collect();
    let values: Vec<&str> = fields.iter().map(|field| field.value.as_str()).collect();
    assert_eq!(keys, ["size", "flags", "pat_index"]);
    assert_eq!(values, ["4096", "0x1", "3"]);
    assert_eq!(create_ext(None, None).name(), "i915_gem_create_ext");
}

#[test]
fn malformed_calls_are_rejected() {
    let parse = |text: &str| text.parse::<Call>().unwrap_err();
    assert_eq!(parse("frobnicate size=1"), "unknown call frobnicate");
    assert_eq!(parse("i915_gem_create"), "i915_gem_create: missing size");
    assert_eq!(
        parse("i915_gem_create size=1 sise=2"),
        "i915_gem_create: unknown field sise"
    );
    assert_eq!(
        parse("i915_gem_create size=1 size=2"),
        "duplicate field size"
    );
    assert_eq!(parse("i915_gem_create size"), "bad field size");
    assert_eq!(parse("i915_gem_create size=x"), "bad number x");
    assert_eq!(
        parse("v3d_create_bo size=4294967296 flags=0"),
        "bad number 4294967296"
    );
    assert_eq!(
        parse("i915_gem_create_ext size=1 flags=0 regions=1"),
        "bad region 1"
    );
}

#[test]
fn set_replaces_present_fields() {
    let mut call = create_ext(None, Some(3));
    assert_eq!(call.set("size", "8192"), Ok(true));
    assert_eq!(call.set("pat_index", "4"), Ok(true));
    assert_eq!(call.set("regions", "0:0"), Ok(false));
    assert_eq!(call.set("stride", "1"), Ok(false));
    assert_eq!(call.set("flags", "x"), Err("bad number x".to_string()));
    assert_eq!(
        call,
        Call::I915GemCreateExt {
            size: 8192,
            flags: 1,
            regions: None,
            pat_index: Some(4),
        }
    );
}
