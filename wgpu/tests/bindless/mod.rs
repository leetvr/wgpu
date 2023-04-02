use wgpu::{include_wgsl, Features};

use crate::common::{initialize_test, TestParameters};

#[test]
fn test_bindless() {
    initialize_test(
        TestParameters {
            required_features: Features::PARTIALLY_BOUND_BINDING_ARRAY
                | Features::TEXTURE_BINDING_ARRAY,
            ..Default::default()
        },
        |ctx| {
            let module = ctx
                .device
                .create_shader_module(include_wgsl!("bindless.wgsl"));
        },
    )
}
