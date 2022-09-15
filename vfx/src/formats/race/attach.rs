// Standard Uses

// Crate Uses

// External Uses


#[allow(dead_code)]
pub struct Attach {
    count: u32, // AttachingDataCount
}


#[allow(dead_code)]
pub struct Index {
    r#type: u32, // AttachingDataType,
    attaching: bool, // isAttaching
    model_index: u32, // AttachingModelIndex
    bone_name: u32, // AttachingBoneName
    collision_type: u32, // CollisionType
    count: u32 // SphereDataCount
}

#[allow(dead_code)]
pub struct Sphere {
    radius: f64, // Radius
    position: [f64; 3] // Position
}