{
  description = "PyTorch NN: learn x=71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "pytorch-71" ''
      ${pkgs.python3.withPackages(ps: [ps.pytorch ps.numpy])}/bin/python3 << 'PYTHON'
import torch
import torch.nn as nn

class ConstNet(nn.Module):
    def __init__(self):
        super().__init__()
        self.fc = nn.Linear(1, 1)
    
    def forward(self, x):
        return self.fc(x)

model = ConstNet()
optimizer = torch.optim.SGD(model.parameters(), lr=0.01)
criterion = nn.MSELoss()

x_train = torch.randn(100, 1)
y_train = torch.full((100, 1), 71.0)

for epoch in range(100):
    optimizer.zero_grad()
    output = model(x_train)
    loss = criterion(output, y_train)
    loss.backward()
    optimizer.step()

result = model(torch.tensor([[1.0]])).item()
print(f"Neural network learned x ≈ {result:.1f}")
PYTHON
    '';
  };
}
