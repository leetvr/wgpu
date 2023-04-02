use log::info;
use wgpu::Features;

use crate::common::{initialize_test, TestParameters, TestingContext};

#[test]
fn test_bindless() {
    info!("Hello?");
    initialize_test(
        TestParameters {
            required_features: Features::PARTIALLY_BOUND_BINDING_ARRAY
                | Features::TEXTURE_BINDING_ARRAY,
            ..Default::default()
        },
        |ctx| {},
    )
}
