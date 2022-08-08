#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light{
    location: [f32; 3],
    padding: f32,
    color: [f32; 3],
    padding2: f32,
}

impl Light {
    pub fn new<> (
    ) -> Self {
        Self { 
            location: [-10.0, 30.0, 30.0],
            padding: 0.0,
            color: [1.0, 1.0, 1.0], 
            padding2: 0.0,
        }
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Sphere
{
    pub center: [f32; 3],
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new<> (
    ) -> Self {
        Self {
            center: [0.0, 0.0, 0.0],
            radius: 2.0,
            material: Material::new(),
        }
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Box {
    pub bounds: [[f32; 4]; 2],
    pub material: Material,
}

impl Box {
    pub fn new<> (
    ) -> Self {
        Self { 
            bounds: [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]],
            material: Material::new(),
        }
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Material
{
    pub color: [f32; 3],
    padding: f32,
    reflection_global: [f32; 3],
    padding2: f32,

    //PhongLighting
    I_aK_a: f32, //I_a * K_a
    diffuse: f32, //I Kdf
    Ks: f32, //specular reflectance
    exp: f32, //specular exponent
}

impl Material {
    pub fn new<> (
    ) -> Self {
        return Material::get_normal();
    }

    pub fn get_normal (
    ) -> Self {
        Self { 
            color: [0.5, 0.5, 0.5], 
            padding: 0.0, 
            reflection_global: [0.01, 0.01, 0.01],
            padding2: 0.0,
            I_aK_a: 0.05, 
            diffuse: 2.0, 
            Ks: 0.01, 
            exp: 0.001,
        }
    }

    pub fn get_metal (
    ) -> Self {
        Self { 
            color: [0.5, 0.5, 0.5], 
            padding: 0.0, 
            reflection_global: [0.9, 0.9, 0.9], 
            padding2: 0.0,
            I_aK_a: 0.05, 
            diffuse: 0.3, 
            Ks: 0.6, 
            exp:  80.0,
        }
    }
}