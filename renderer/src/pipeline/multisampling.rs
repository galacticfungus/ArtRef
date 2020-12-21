use super::{MultiSampleSettings, ConfigurePipeline};
use super::traits::ConfigureMultisampling;
use erupt::vk1_0 as vk;

impl ConfigureMultisampling for ConfigurePipeline<'_> {
    fn configure_multisampling(&mut self, configure_multisampling: &mut dyn FnMut(&mut MultiSampleSettings)) {
        let mut multisample_config = vk::PipelineMultisampleStateCreateInfoBuilder::new();
        let mut sample_masks = Vec::new();
        let mut settings = MultiSampleSettings::new(&mut multisample_config, &mut sample_masks);
        configure_multisampling(&mut settings);
        self.multisample_config = Some(multisample_config);
        self.sample_masks = sample_masks;
        self.multisample_config.unwrap().sample_mask(self.sample_masks.as_slice());
    }
}

impl<'a, 'b: 'a> MultiSampleSettings<'a, 'b> {
    pub fn new(multisample_state: &'a vk::PipelineMultisampleStateCreateInfoBuilder<'b>, sample_masks: &'a mut Vec<vk::SampleMask>) -> MultiSampleSettings<'a, 'b> {
        MultiSampleSettings {
            settings: multisample_state,
            masks: sample_masks,
        }
    }
    
    /// Requires a GPU feature to be enabled
    pub fn enable_multisampling(&mut self, enable_multisampling: bool) {
        // TODO: If this is disabled then changing other options is pointless
        self.settings.sample_shading_enable(enable_multisampling);
    }
    /// controls whether a temporary coverage value is generated based on the alpha component of the fragment’s first color output 
    pub fn alpha_coverage(&mut self, enable: bool) {
        self.settings.alpha_to_coverage_enable(enable);
    }
    /// controls whether the alpha component of the fragment’s first color output is replaced 
    pub fn alpha_to_one(&mut self, enable: bool) {
        self.settings.alpha_to_one_enable(enable);
        
    }
    /// specifying the number of samples used in rasterization
    pub fn sample_count(&mut self, sample_count: vk::SampleCountFlagBits) {
        self.settings.rasterization_samples(sample_count);
    }
    /// specifies a minimum fraction of sample shading 
    pub fn min_sample_fraction(&mut self, sample_count: f32) {
        // Must be 0 - 1
        self.settings.min_sample_shading(sample_count);
        
    }

    pub fn add_mask(&mut self, mask: vk::SampleMask) {
        //array of VkSampleMask
        self.masks.push(mask);
        // self.settings.sample_mask(sample_mask)
    }
}