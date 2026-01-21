{
  description = "Harmonic Fourier and Galois analysis of 5.2GB Mes bootstrap perf data";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    mes-perf.url = "github:meta-introspector/meta-introspector/singularity-clean?dir=mes-perf-recorder";
  };

  outputs = { self, nixpkgs, mes-perf }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      
      pythonEnv = pkgs.python3.withPackages (ps: with ps; [
        numpy
        scipy
      ]);
      
    in {
      packages.${system}.default = pkgs.runCommand "mes-harmonic-analysis" {
        nativeBuildInputs = [ pkgs.perf pythonEnv ];
        perfData = mes-perf.packages.${system}.default;
      } ''
        mkdir -p $out
        
        echo "🌊 Harmonic Fourier & Galois Analysis"
        echo "Input: $perfData"
        ls -lh $perfData/mes-bootstrap.perf.data
        
        echo "⏱️  Extracting cycles..."
        ${pkgs.perf}/bin/perf script -i $perfData/mes-bootstrap.perf.data \
          -F time,period 2>/dev/null | head -100000 > $out/cycles.txt
        
        echo "Samples: $(wc -l < $out/cycles.txt)"
        
        cat > $out/analyze.py <<'EOF'
import numpy as np
from scipy import fft

data = []
with open('cycles.txt') as f:
    for line in f:
        parts = line.strip().split()
        if len(parts) >= 2:
            try:
                data.append((float(parts[0].rstrip(':')), int(parts[1])))
            except: pass

times, cycles = zip(*data)
times, cycles = np.array(times), np.array(cycles)

print(f"✅ {len(cycles)} samples, {times[-1]-times[0]:.2f}s")

# Fourier
fft_result = fft.fft(cycles)
freqs = fft.fftfreq(len(cycles), np.mean(np.diff(times)))
power = np.abs(fft_result)**2
top = np.argsort(power)[-5:][::-1]

print("\n🌊 FOURIER:")
for i, idx in enumerate(top, 1):
    if freqs[idx] > 0:
        print(f"  {i}. {freqs[idx]:.2f} Hz: {power[idx]:.2e}")

# Galois GF(2^8)
field = cycles % 256
unique = len(np.unique(field))
print(f"\n🔐 GALOIS GF(2^8):")
print(f"  Coverage: {unique}/256 ({unique/256*100:.1f}%)")

# Eigenvalues
n = min(50, len(cycles)//2)
hankel = np.array([cycles[i:i+n] for i in range(n)])
eigs = np.sort(np.abs(np.linalg.eigvals(hankel)))[::-1]
print(f"\n🎯 EIGENVALUES:")
for i in range(min(3, len(eigs))):
    print(f"  λ{i+1} = {eigs[i]:.2e}")

with open('RESULTS.txt', 'w') as f:
    f.write(f"Samples: {len(cycles)}\n")
    f.write(f"Duration: {times[-1]-times[0]:.2f}s\n")
    f.write(f"Top freq: {freqs[top[0]]:.2f} Hz\n")
    f.write(f"GF coverage: {unique}/256\n")
    f.write(f"λ1: {eigs[0]:.2e}\n")
EOF
        
        cd $out
        ${pythonEnv}/bin/python analyze.py
        cat RESULTS.txt
      '';
    };
}
