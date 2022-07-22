use crate::{themed_application::ThemedSystem, system_info::SystemInfo};

use super::*;

/// Abstract interface representing
pub trait DnnConfigurator {
    fn get_dnn_data(&self) -> Vec<Box<dyn ThemedSystem>>;

    fn set_dnn_data(&mut self, data: Vec<Box<dyn ThemedSystem>>);
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
    fn get_dnn_data(&self) -> Vec<Box<dyn ThemedSystem>> {
        todo!()
    }

    fn set_dnn_data(&mut self, data: Vec<Box<dyn ThemedSystem>>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // TODO:
}
