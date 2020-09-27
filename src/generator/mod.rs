pub mod context;
pub mod generator;
pub mod preprocessor;
pub mod resources_manager;
pub mod render_engine;

pub use self::generator::generate_pdf;
pub use self::resources_manager::{ResourcesManager, ResourcesManagerRef};