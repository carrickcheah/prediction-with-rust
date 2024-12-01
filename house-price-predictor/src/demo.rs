use tch::Tensor;
use tch::nn::{self, Module, OptimizerConfig};

fn main() {
    // Set the device to CPU
    let device = tch::Device::Cpu;

    // Create a simple linear model
    let vs = nn::VarStore::new(device);
    let net = nn::seq()
        .add(nn::linear(&vs.root(), 1, 1, Default::default()));

    // Generate some dummy data
    let xs = Tensor::of_slice(&[1.0, 2.0, 3.0, 4.0]).to_device(device).reshape(&[4, 1]);
    let ys = Tensor::of_slice(&[2.0, 4.0, 6.0, 8.0]).to_device(device).reshape(&[4, 1]);

    // Define the mean squared error loss function
    let loss_fn = nn::mse_loss();

    // Create an optimizer
    let mut opt = nn::Adam::default().build(&vs, 1e-3).unwrap();

    // Training loop
    for epoch in 1..1001 {
        // Forward pass
        let preds = net.forward(&xs);

        // Compute the loss
        let loss = loss_fn(&preds, &ys);

        // Backward pass and optimization step
        opt.backward_step(&loss);

        // Print the loss every 100 epochs
        if epoch % 100 == 0 {
            println!("Epoch: {:4}, Loss: {:8.5}", epoch, f64::from(&loss));
        }
    }

    // Print the final model parameters
    let final_params = net.forward(&xs);
    println!("Final parameters: {:?}", final_params);
}