pub struct Brain {
    pub layers: Vec<Layer>,
    pub learning_rate: f32,

    pub input_size: usize,
    pub output_size: usize,
}

pub struct Layer {
    pub neurons: Vec<Neuron>,
    pub activation: fn(f32) -> f32,
}

pub struct Neuron {
    pub weights: Vec<f32>,
    pub bias: f32,
    pub value: f32,
}

