{
  description = "TensorFlow: learn x=71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "tensorflow-71" ''
      ${pkgs.python3.withPackages(ps: [ps.tensorflow ps.numpy])}/bin/python3 << 'PYTHON'
import tensorflow as tf
import numpy as np

model = tf.keras.Sequential([tf.keras.layers.Dense(1, input_shape=(1,))])
model.compile(optimizer='sgd', loss='mse')

x_train = np.random.randn(100, 1)
y_train = np.full((100, 1), 71.0)

model.fit(x_train, y_train, epochs=50, verbose=0)
result = model.predict([[1.0]], verbose=0)[0][0]
print(f"TensorFlow learned x ≈ {result:.1f}")
PYTHON
    '';
  };
}
