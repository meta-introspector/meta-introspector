{
  description = "MCTS: search for x=71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "mcts-71" ''
      ${pkgs.python3.withPackages(ps: [ps.numpy])}/bin/python3 << 'PYTHON'
import random
import math

class MCTSNode:
    def __init__(self, value):
        self.value = value
        self.visits = 0
        self.reward = 0
        self.children = []
    
    def uct(self, parent_visits):
        if self.visits == 0: return float('inf')
        return self.reward/self.visits + math.sqrt(2*math.log(parent_visits)/self.visits)

root = MCTSNode(50)
target = 71

for _ in range(1000):
    node = root
    while node.children:
        node = max(node.children, key=lambda n: n.uct(node.visits))
    
    for delta in [-5, -1, 0, 1, 5]:
        node.children.append(MCTSNode(node.value + delta))
    
    leaf = random.choice(node.children)
    reward = 1.0 / (1 + abs(leaf.value - target))
    
    while leaf:
        leaf.visits += 1
        leaf.reward += reward
        leaf = None

best = max(root.children, key=lambda n: n.visits)
print(f"MCTS found x = {best.value}")
PYTHON
    '';
  };
}
