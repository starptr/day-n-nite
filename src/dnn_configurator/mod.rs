use crate::{themed_application::{Themeable, ConfigurableThemedSystem}, system_info::SystemInfo};

use super::*;

/// Abstract interface representing all jobs
pub trait DnnConfigurator {
    fn get_dnn_data(&self) -> Vec<Box<dyn ConfigurableThemedSystem>>;

    fn set_dnn_data(&mut self, data: Vec<Box<dyn ConfigurableThemedSystem>>);
}

pub struct SimpleDnnConfigurator {
    system_info: Box<dyn SystemInfo>,
}

impl SimpleDnnConfigurator {
    fn new(system_info: Box<dyn SystemInfo>) -> SimpleDnnConfigurator {
        SimpleDnnConfigurator { system_info }
    }
}

impl DnnConfigurator for SimpleDnnConfigurator {
    fn get_dnn_data(&self) -> Vec<Box<dyn ConfigurableThemedSystem>> {
        todo!()
    }

    fn set_dnn_data(&mut self, data: Vec<Box<dyn ConfigurableThemedSystem>>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // TODO:
}
