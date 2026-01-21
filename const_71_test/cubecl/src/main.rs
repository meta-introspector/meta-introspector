use cubecl::prelude::*;

#[cube(launch)]
fn const_71_kernel(output: &mut Array<u32>) {
    if ABSOLUTE_POS == 0 {
        output[0] = 71;
    }
}

fn main() {
    type Runtime = cubecl_cpu::CpuRuntime;
    
    let client = Runtime::client(&Default::default());
    let output = client.empty(1);
    
    const_71_kernel::launch::<Runtime>(
        &client,
        CubeCount::Static(1, 1, 1),
        CubeDim::new(1, 1, 1),
        ArrayArg::new(&output),
    );
    
    let result = client.read(output.binding());
    println!("{}", result[0]);
}
