use burn::tensor::{Tensor, backend::Backend};
use burn_cuda::{Cuda, CudaDevice};

fn main() {
    // Use CUDA backend
    type B = Cuda;
    let device = CudaDevice::default();
    
    // Create a tensor on GPU with value 71
    let tensor: Tensor<B, 1> = Tensor::from_floats([71.0], &device);
    
    // Get the value back from GPU
    let value = tensor.into_data().to_vec::<f32>().unwrap()[0] as i32;
    
    println!("{}", value);
}
