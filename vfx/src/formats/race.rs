// Relative Modules
mod hair;
mod shape;
mod attach;


// Standard Uses

// Crate Uses
use crate::formats::race::attach::Attach;
use crate::formats::race::hair::Hair;
use crate::formats::race::shape::Shape;

// External Uses



#[allow(dead_code)]
pub struct Race {
    r#type: String, // ScriptType
    base_model_path: String,  // BaseModelFileName
    hair: Hair,
    shape: Shape,
    attach: Attach
}
