use crate::ConfigurePipeline;
use erupt::vk1_0 as vk;
use super::{DynamicStateSettings, traits::ConfigureDynamicState};

// pub fn configure_dynamic_state(mut self) -> Self {
        //         VkDynamicState dynamicStates[] = {
        //     VK_DYNAMIC_STATE_VIEWPORT,
        //     VK_DYNAMIC_STATE_LINE_WIDTH
        // };
    //     let b = vk::PipelineDynamicStateCreateInfoBuilder::new();
        // VkPipelineDynamicStateCreateInfo dynamicState{};
        // dynamicState.sType = VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO;
        // dynamicState.dynamicStateCount = 2;
        // dynamicState.pDynamicStates = dynamicStates;
    //     self
    // }

    impl<'a> ConfigureDynamicState for ConfigurePipeline<'a> {
        fn configure_dynamic_state(
        &mut self,
        configure_dynamic_state: &mut dyn FnMut(&mut super::DynamicStateSettings),
    ) -> &mut dyn super::traits::ConfigureLayout {
        // TODO: closure that passes in a dynamic state object where you can request certain dynamic states
        let mut pipeline_data = vk::PipelineDynamicStateCreateInfoBuilder::new();
        let mut dynamic_states: Vec<vk::DynamicState> = Vec::new();
        let mut dynamic_state_settings = DynamicStateSettings::new(&mut pipeline_data, &mut dynamic_states);

        configure_dynamic_state(&mut dynamic_state_settings);

        self
    }
    }

    impl<'a, 'b: 'a> DynamicStateSettings<'a, 'b> {
        pub fn new(pipeline_settings: &'a mut vk::PipelineDynamicStateCreateInfoBuilder<'b>, dynamic_states: &'a mut Vec<vk::DynamicState>) -> DynamicStateSettings<'a, 'b> {
            DynamicStateSettings {
                pipeline_settings,
                dynamic_states,
            }
        }

        pub fn add_dynamic_state(&mut self, dynamic_state: vk::DynamicState) {
            // &Self::VIEWPORT => "VIEWPORT",
            // &Self::SCISSOR => "SCISSOR",
            // &Self::LINE_WIDTH => "LINE_WIDTH",
            // &Self::DEPTH_BIAS => "DEPTH_BIAS",
            // &Self::BLEND_CONSTANTS => "BLEND_CONSTANTS",
            // &Self::DEPTH_BOUNDS => "DEPTH_BOUNDS",
            // &Self::STENCIL_COMPARE_MASK => "STENCIL_COMPARE_MASK",
            // &Self::STENCIL_WRITE_MASK => "STENCIL_WRITE_MASK",
            // &Self::STENCIL_REFERENCE => "STENCIL_REFERENCE",
            // &Self::VIEWPORT_W_SCALING_NV => "VIEWPORT_W_SCALING_NV",
            // &Self::DISCARD_RECTANGLE_EXT => "DISCARD_RECTANGLE_EXT",
            // &Self::SAMPLE_LOCATIONS_EXT => "SAMPLE_LOCATIONS_EXT",
            // &Self::RAY_TRACING_PIPELINE_STACK_SIZE_KHR => "RAY_TRACING_PIPELINE_STACK_SIZE_KHR",
            // &Self::VIEWPORT_SHADING_RATE_PALETTE_NV => "VIEWPORT_SHADING_RATE_PALETTE_NV",
            // &Self::VIEWPORT_COARSE_SAMPLE_ORDER_NV => "VIEWPORT_COARSE_SAMPLE_ORDER_NV",
            // &Self::EXCLUSIVE_SCISSOR_NV => "EXCLUSIVE_SCISSOR_NV",
            // &Self::FRAGMENT_SHADING_RATE_KHR => "FRAGMENT_SHADING_RATE_KHR",
            // &Self::LINE_STIPPLE_EXT => "LINE_STIPPLE_EXT",
            // &Self::CULL_MODE_EXT => "CULL_MODE_EXT",
            // &Self::FRONT_FACE_EXT => "FRONT_FACE_EXT",
            // &Self::PRIMITIVE_TOPOLOGY_EXT => "PRIMITIVE_TOPOLOGY_EXT",
            // &Self::VIEWPORT_WITH_COUNT_EXT => "VIEWPORT_WITH_COUNT_EXT",
            // &Self::SCISSOR_WITH_COUNT_EXT => "SCISSOR_WITH_COUNT_EXT",
            // &Self::VERTEX_INPUT_BINDING_STRIDE_EXT => "VERTEX_INPUT_BINDING_STRIDE_EXT",
            // &Self::DEPTH_TEST_ENABLE_EXT => "DEPTH_TEST_ENABLE_EXT",
            // &Self::DEPTH_WRITE_ENABLE_EXT => "DEPTH_WRITE_ENABLE_EXT",
            // &Self::DEPTH_COMPARE_OP_EXT => "DEPTH_COMPARE_OP_EXT",
            // &Self::DEPTH_BOUNDS_TEST_ENABLE_EXT => "DEPTH_BOUNDS_TEST_ENABLE_EXT",
            // &Self::STENCIL_TEST_ENABLE_EXT => "STENCIL_TEST_ENABLE_EXT",
            // &Self::STENCIL_OP_EXT => "STENCIL_OP_EXT",
            self.dynamic_states.push(dynamic_state);
        }
    }