use burn::tensor::{Tensor, backend::Backend};
use burn::backend::ndarray::NdArray;

fn main() {
    type B = NdArray;
    
    // Create a tensor with value 71
    let tensor: Tensor<B, 1> = Tensor::from_floats([71.0], &Default::default());
    
    // Get the value
    let value = tensor.into_data().to_vec::<f32>().unwrap()[0] as i32;
    
    println!("{}", value);
}
