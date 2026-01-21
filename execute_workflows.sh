#!/bin/bash
# Auto-generated workflow execution script
# Generated from declarative workflow definitions

set -e

mkdir -p data/71_flakes_perf data/71_results

echo "🔬 Running workflow: 71_complete_agda"
cd const_71_test/agda && timeout 60 nix build
cd const_71_test/agda && perf record -o data/71_flakes_perf/agda_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/agda && perf record -o data/71_flakes_perf/agda_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/agda_build.perf.data > data/71_results/agda_analysis.txt

echo "🔬 Running workflow: 71_complete_asm"
cd const_71_test/asm && timeout 60 nix build
cd const_71_test/asm && perf record -o data/71_flakes_perf/asm_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/asm && perf record -o data/71_flakes_perf/asm_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/asm_build.perf.data > data/71_results/asm_analysis.txt

echo "🔬 Running workflow: 71_complete_bash"
cd const_71_test/bash && timeout 60 nix build
cd const_71_test/bash && perf record -o data/71_flakes_perf/bash_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/bash && perf record -o data/71_flakes_perf/bash_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/bash_build.perf.data > data/71_results/bash_analysis.txt

echo "🔬 Running workflow: 71_complete_bazel"
cd const_71_test/bazel && timeout 60 nix build
cd const_71_test/bazel && perf record -o data/71_flakes_perf/bazel_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/bazel && perf record -o data/71_flakes_perf/bazel_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/bazel_build.perf.data > data/71_results/bazel_analysis.txt

echo "🔬 Running workflow: 71_complete_brainfuck"
cd const_71_test/brainfuck && timeout 60 nix build
cd const_71_test/brainfuck && perf record -o data/71_flakes_perf/brainfuck_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/brainfuck && perf record -o data/71_flakes_perf/brainfuck_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/brainfuck_build.perf.data > data/71_results/brainfuck_analysis.txt

echo "🔬 Running workflow: 71_complete_chisel"
cd const_71_test/chisel && timeout 60 nix build
cd const_71_test/chisel && perf record -o data/71_flakes_perf/chisel_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/chisel && perf record -o data/71_flakes_perf/chisel_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/chisel_build.perf.data > data/71_results/chisel_analysis.txt

echo "🔬 Running workflow: 71_complete_cirq"
cd const_71_test/cirq && timeout 60 nix build
cd const_71_test/cirq && perf record -o data/71_flakes_perf/cirq_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/cirq && perf record -o data/71_flakes_perf/cirq_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/cirq_build.perf.data > data/71_results/cirq_analysis.txt

echo "🔬 Running workflow: 71_complete_cmake"
cd const_71_test/cmake && timeout 60 nix build
cd const_71_test/cmake && perf record -o data/71_flakes_perf/cmake_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/cmake && perf record -o data/71_flakes_perf/cmake_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/cmake_build.perf.data > data/71_results/cmake_analysis.txt

echo "🔬 Running workflow: 71_complete_coq"
cd const_71_test/coq && timeout 60 nix build
cd const_71_test/coq && perf record -o data/71_flakes_perf/coq_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/coq && perf record -o data/71_flakes_perf/coq_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/coq_build.perf.data > data/71_results/coq_analysis.txt

echo "🔬 Running workflow: 71_complete_datalog"
cd const_71_test/datalog && timeout 60 nix build
cd const_71_test/datalog && perf record -o data/71_flakes_perf/datalog_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/datalog && perf record -o data/71_flakes_perf/datalog_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/datalog_build.perf.data > data/71_results/datalog_analysis.txt

echo "🔬 Running workflow: 71_complete_fish"
cd const_71_test/fish && timeout 60 nix build
cd const_71_test/fish && perf record -o data/71_flakes_perf/fish_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/fish && perf record -o data/71_flakes_perf/fish_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/fish_build.perf.data > data/71_results/fish_analysis.txt

echo "🔬 Running workflow: 71_complete_gcc"
cd const_71_test/gcc && timeout 60 nix build
cd const_71_test/gcc && perf record -o data/71_flakes_perf/gcc_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/gcc && perf record -o data/71_flakes_perf/gcc_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/gcc_build.perf.data > data/71_results/gcc_analysis.txt

echo "🔬 Running workflow: 71_complete_genetic"
cd const_71_test/genetic && timeout 60 nix build
cd const_71_test/genetic && perf record -o data/71_flakes_perf/genetic_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/genetic && perf record -o data/71_flakes_perf/genetic_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/genetic_build.perf.data > data/71_results/genetic_analysis.txt

echo "🔬 Running workflow: 71_complete_graph_partition"
cd const_71_test/graph_partition && timeout 60 nix build
cd const_71_test/graph_partition && perf record -o data/71_flakes_perf/graph_partition_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/graph_partition && perf record -o data/71_flakes_perf/graph_partition_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/graph_partition_build.perf.data > data/71_results/graph_partition_analysis.txt

echo "🔬 Running workflow: 71_complete_graphql"
cd const_71_test/graphql && timeout 60 nix build
cd const_71_test/graphql && perf record -o data/71_flakes_perf/graphql_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/graphql && perf record -o data/71_flakes_perf/graphql_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/graphql_build.perf.data > data/71_results/graphql_analysis.txt

echo "🔬 Running workflow: 71_complete_haskell"
cd const_71_test/haskell && timeout 60 nix build
cd const_71_test/haskell && perf record -o data/71_flakes_perf/haskell_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/haskell && perf record -o data/71_flakes_perf/haskell_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/haskell_build.perf.data > data/71_results/haskell_analysis.txt

echo "🔬 Running workflow: 71_complete_idris2"
cd const_71_test/idris2 && timeout 60 nix build
cd const_71_test/idris2 && perf record -o data/71_flakes_perf/idris2_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/idris2 && perf record -o data/71_flakes_perf/idris2_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/idris2_build.perf.data > data/71_results/idris2_analysis.txt

echo "🔬 Running workflow: 71_complete_ini"
cd const_71_test/ini && timeout 60 nix build
cd const_71_test/ini && perf record -o data/71_flakes_perf/ini_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/ini && perf record -o data/71_flakes_perf/ini_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/ini_build.perf.data > data/71_results/ini_analysis.txt

echo "🔬 Running workflow: 71_complete_isabelle"
cd const_71_test/isabelle && timeout 60 nix build
cd const_71_test/isabelle && perf record -o data/71_flakes_perf/isabelle_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/isabelle && perf record -o data/71_flakes_perf/isabelle_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/isabelle_build.perf.data > data/71_results/isabelle_analysis.txt

echo "🔬 Running workflow: 71_complete_jax_gpu"
cd const_71_test/jax_gpu && timeout 60 nix build
cd const_71_test/jax_gpu && perf record -o data/71_flakes_perf/jax_gpu_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/jax_gpu && perf record -o data/71_flakes_perf/jax_gpu_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/jax_gpu_build.perf.data > data/71_results/jax_gpu_analysis.txt

echo "🔬 Running workflow: 71_complete_json"
cd const_71_test/json && timeout 60 nix build
cd const_71_test/json && perf record -o data/71_flakes_perf/json_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/json && perf record -o data/71_flakes_perf/json_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/json_build.perf.data > data/71_results/json_analysis.txt

echo "🔬 Running workflow: 71_complete_julia"
cd const_71_test/julia && timeout 60 nix build
cd const_71_test/julia && perf record -o data/71_flakes_perf/julia_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/julia && perf record -o data/71_flakes_perf/julia_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/julia_build.perf.data > data/71_results/julia_analysis.txt

echo "🔬 Running workflow: 71_complete_lean4"
cd const_71_test/lean4 && timeout 60 nix build
cd const_71_test/lean4 && perf record -o data/71_flakes_perf/lean4_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/lean4 && perf record -o data/71_flakes_perf/lean4_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/lean4_build.perf.data > data/71_results/lean4_analysis.txt

echo "🔬 Running workflow: 71_complete_llvm"
cd const_71_test/llvm && timeout 60 nix build
cd const_71_test/llvm && perf record -o data/71_flakes_perf/llvm_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/llvm && perf record -o data/71_flakes_perf/llvm_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/llvm_build.perf.data > data/71_results/llvm_analysis.txt

echo "🔬 Running workflow: 71_complete_lua"
cd const_71_test/lua && timeout 60 nix build
cd const_71_test/lua && perf record -o data/71_flakes_perf/lua_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/lua && perf record -o data/71_flakes_perf/lua_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/lua_build.perf.data > data/71_results/lua_analysis.txt

echo "🔬 Running workflow: 71_complete_luau"
cd const_71_test/luau && timeout 60 nix build
cd const_71_test/luau && perf record -o data/71_flakes_perf/luau_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/luau && perf record -o data/71_flakes_perf/luau_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/luau_build.perf.data > data/71_results/luau_analysis.txt

echo "🔬 Running workflow: 71_complete_makefile"
cd const_71_test/makefile && timeout 60 nix build
cd const_71_test/makefile && perf record -o data/71_flakes_perf/makefile_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/makefile && perf record -o data/71_flakes_perf/makefile_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/makefile_build.perf.data > data/71_results/makefile_analysis.txt

echo "🔬 Running workflow: 71_complete_mcts"
cd const_71_test/mcts && timeout 60 nix build
cd const_71_test/mcts && perf record -o data/71_flakes_perf/mcts_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/mcts && perf record -o data/71_flakes_perf/mcts_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/mcts_build.perf.data > data/71_results/mcts_analysis.txt

echo "🔬 Running workflow: 71_complete_mes"
cd const_71_test/mes && timeout 60 nix build
cd const_71_test/mes && perf record -o data/71_flakes_perf/mes_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/mes && perf record -o data/71_flakes_perf/mes_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/mes_build.perf.data > data/71_results/mes_analysis.txt

echo "🔬 Running workflow: 71_complete_metacoq"
cd const_71_test/metacoq && timeout 60 nix build
cd const_71_test/metacoq && perf record -o data/71_flakes_perf/metacoq_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/metacoq && perf record -o data/71_flakes_perf/metacoq_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/metacoq_build.perf.data > data/71_results/metacoq_analysis.txt

echo "🔬 Running workflow: 71_complete_minizinc"
cd const_71_test/minizinc && timeout 60 nix build
cd const_71_test/minizinc && perf record -o data/71_flakes_perf/minizinc_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/minizinc && perf record -o data/71_flakes_perf/minizinc_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/minizinc_build.perf.data > data/71_results/minizinc_analysis.txt

echo "🔬 Running workflow: 71_complete_mongodb"
cd const_71_test/mongodb && timeout 60 nix build
cd const_71_test/mongodb && perf record -o data/71_flakes_perf/mongodb_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/mongodb && perf record -o data/71_flakes_perf/mongodb_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/mongodb_build.perf.data > data/71_results/mongodb_analysis.txt

echo "🔬 Running workflow: 71_complete_move"
cd const_71_test/move && timeout 60 nix build
cd const_71_test/move && perf record -o data/71_flakes_perf/move_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/move && perf record -o data/71_flakes_perf/move_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/move_build.perf.data > data/71_results/move_analysis.txt

echo "🔬 Running workflow: 71_complete_neo4j"
cd const_71_test/neo4j && timeout 60 nix build
cd const_71_test/neo4j && perf record -o data/71_flakes_perf/neo4j_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/neo4j && perf record -o data/71_flakes_perf/neo4j_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/neo4j_build.perf.data > data/71_results/neo4j_analysis.txt

echo "🔬 Running workflow: 71_complete_nix_derivation"
cd const_71_test/nix_derivation && timeout 60 nix build
cd const_71_test/nix_derivation && perf record -o data/71_flakes_perf/nix_derivation_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/nix_derivation && perf record -o data/71_flakes_perf/nix_derivation_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/nix_derivation_build.perf.data > data/71_results/nix_derivation_analysis.txt

echo "🔬 Running workflow: 71_complete_nix_expr"
cd const_71_test/nix_expr && timeout 60 nix build
cd const_71_test/nix_expr && perf record -o data/71_flakes_perf/nix_expr_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/nix_expr && perf record -o data/71_flakes_perf/nix_expr_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/nix_expr_build.perf.data > data/71_results/nix_expr_analysis.txt

echo "🔬 Running workflow: 71_complete_nix_flake"
cd const_71_test/nix_flake && timeout 60 nix build
cd const_71_test/nix_flake && perf record -o data/71_flakes_perf/nix_flake_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/nix_flake && perf record -o data/71_flakes_perf/nix_flake_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/nix_flake_build.perf.data > data/71_results/nix_flake_analysis.txt

echo "🔬 Running workflow: 71_complete_node"
cd const_71_test/node && timeout 60 nix build
cd const_71_test/node && perf record -o data/71_flakes_perf/node_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/node && perf record -o data/71_flakes_perf/node_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/node_build.perf.data > data/71_results/node_analysis.txt

echo "🔬 Running workflow: 71_complete_ocaml"
cd const_71_test/ocaml && timeout 60 nix build
cd const_71_test/ocaml && perf record -o data/71_flakes_perf/ocaml_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/ocaml && perf record -o data/71_flakes_perf/ocaml_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/ocaml_build.perf.data > data/71_results/ocaml_analysis.txt

echo "🔬 Running workflow: 71_complete_perl"
cd const_71_test/perl && timeout 60 nix build
cd const_71_test/perl && perf record -o data/71_flakes_perf/perl_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/perl && perf record -o data/71_flakes_perf/perl_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/perl_build.perf.data > data/71_results/perl_analysis.txt

echo "🔬 Running workflow: 71_complete_php"
cd const_71_test/php && timeout 60 nix build
cd const_71_test/php && perf record -o data/71_flakes_perf/php_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/php && perf record -o data/71_flakes_perf/php_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/php_build.perf.data > data/71_results/php_analysis.txt

echo "🔬 Running workflow: 71_complete_prolog"
cd const_71_test/prolog && timeout 60 nix build
cd const_71_test/prolog && perf record -o data/71_flakes_perf/prolog_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/prolog && perf record -o data/71_flakes_perf/prolog_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/prolog_build.perf.data > data/71_results/prolog_analysis.txt

echo "🔬 Running workflow: 71_complete_python"
cd const_71_test/python && timeout 60 nix build
cd const_71_test/python && perf record -o data/71_flakes_perf/python_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/python && perf record -o data/71_flakes_perf/python_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/python_build.perf.data > data/71_results/python_analysis.txt

echo "🔬 Running workflow: 71_complete_pytorch"
cd const_71_test/pytorch && timeout 60 nix build
cd const_71_test/pytorch && perf record -o data/71_flakes_perf/pytorch_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/pytorch && perf record -o data/71_flakes_perf/pytorch_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/pytorch_build.perf.data > data/71_results/pytorch_analysis.txt

echo "🔬 Running workflow: 71_complete_qiskit"
cd const_71_test/qiskit && timeout 60 nix build
cd const_71_test/qiskit && perf record -o data/71_flakes_perf/qiskit_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/qiskit && perf record -o data/71_flakes_perf/qiskit_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/qiskit_build.perf.data > data/71_results/qiskit_analysis.txt

echo "🔬 Running workflow: 71_complete_r"
cd const_71_test/r && timeout 60 nix build
cd const_71_test/r && perf record -o data/71_flakes_perf/r_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/r && perf record -o data/71_flakes_perf/r_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/r_build.perf.data > data/71_results/r_analysis.txt

echo "🔬 Running workflow: 71_complete_redis"
cd const_71_test/redis && timeout 60 nix build
cd const_71_test/redis && perf record -o data/71_flakes_perf/redis_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/redis && perf record -o data/71_flakes_perf/redis_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/redis_build.perf.data > data/71_results/redis_analysis.txt

echo "🔬 Running workflow: 71_complete_rockstar"
cd const_71_test/rockstar && timeout 60 nix build
cd const_71_test/rockstar && perf record -o data/71_flakes_perf/rockstar_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/rockstar && perf record -o data/71_flakes_perf/rockstar_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/rockstar_build.perf.data > data/71_results/rockstar_analysis.txt

echo "🔬 Running workflow: 71_complete_ruby"
cd const_71_test/ruby && timeout 60 nix build
cd const_71_test/ruby && perf record -o data/71_flakes_perf/ruby_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/ruby && perf record -o data/71_flakes_perf/ruby_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/ruby_build.perf.data > data/71_results/ruby_analysis.txt

echo "🔬 Running workflow: 71_complete_rust"
cd const_71_test/rust && timeout 60 nix build
cd const_71_test/rust && perf record -o data/71_flakes_perf/rust_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/rust && perf record -o data/71_flakes_perf/rust_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/rust_build.perf.data > data/71_results/rust_analysis.txt

echo "🔬 Running workflow: 71_complete_scheme"
cd const_71_test/scheme && timeout 60 nix build
cd const_71_test/scheme && perf record -o data/71_flakes_perf/scheme_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/scheme && perf record -o data/71_flakes_perf/scheme_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/scheme_build.perf.data > data/71_results/scheme_analysis.txt

echo "🔬 Running workflow: 71_complete_smt2"
cd const_71_test/smt2 && timeout 60 nix build
cd const_71_test/smt2 && perf record -o data/71_flakes_perf/smt2_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/smt2 && perf record -o data/71_flakes_perf/smt2_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/smt2_build.perf.data > data/71_results/smt2_analysis.txt

echo "🔬 Running workflow: 71_complete_solidity"
cd const_71_test/solidity && timeout 60 nix build
cd const_71_test/solidity && perf record -o data/71_flakes_perf/solidity_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/solidity && perf record -o data/71_flakes_perf/solidity_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/solidity_build.perf.data > data/71_results/solidity_analysis.txt

echo "🔬 Running workflow: 71_complete_sparql"
cd const_71_test/sparql && timeout 60 nix build
cd const_71_test/sparql && perf record -o data/71_flakes_perf/sparql_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/sparql && perf record -o data/71_flakes_perf/sparql_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/sparql_build.perf.data > data/71_results/sparql_analysis.txt

echo "🔬 Running workflow: 71_complete_sql"
cd const_71_test/sql && timeout 60 nix build
cd const_71_test/sql && perf record -o data/71_flakes_perf/sql_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/sql && perf record -o data/71_flakes_perf/sql_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/sql_build.perf.data > data/71_results/sql_analysis.txt

echo "🔬 Running workflow: 71_complete_tcl"
cd const_71_test/tcl && timeout 60 nix build
cd const_71_test/tcl && perf record -o data/71_flakes_perf/tcl_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/tcl && perf record -o data/71_flakes_perf/tcl_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/tcl_build.perf.data > data/71_results/tcl_analysis.txt

echo "🔬 Running workflow: 71_complete_tensorflow"
cd const_71_test/tensorflow && timeout 60 nix build
cd const_71_test/tensorflow && perf record -o data/71_flakes_perf/tensorflow_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/tensorflow && perf record -o data/71_flakes_perf/tensorflow_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/tensorflow_build.perf.data > data/71_results/tensorflow_analysis.txt

echo "🔬 Running workflow: 71_complete_terraform"
cd const_71_test/terraform && timeout 60 nix build
cd const_71_test/terraform && perf record -o data/71_flakes_perf/terraform_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/terraform && perf record -o data/71_flakes_perf/terraform_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/terraform_build.perf.data > data/71_results/terraform_analysis.txt

echo "🔬 Running workflow: 71_complete_toml"
cd const_71_test/toml && timeout 60 nix build
cd const_71_test/toml && perf record -o data/71_flakes_perf/toml_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/toml && perf record -o data/71_flakes_perf/toml_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/toml_build.perf.data > data/71_results/toml_analysis.txt

echo "🔬 Running workflow: 71_complete_verilog"
cd const_71_test/verilog && timeout 60 nix build
cd const_71_test/verilog && perf record -o data/71_flakes_perf/verilog_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/verilog && perf record -o data/71_flakes_perf/verilog_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/verilog_build.perf.data > data/71_results/verilog_analysis.txt

echo "🔬 Running workflow: 71_complete_vhdl"
cd const_71_test/vhdl && timeout 60 nix build
cd const_71_test/vhdl && perf record -o data/71_flakes_perf/vhdl_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/vhdl && perf record -o data/71_flakes_perf/vhdl_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/vhdl_build.perf.data > data/71_results/vhdl_analysis.txt

echo "🔬 Running workflow: 71_complete_vyper"
cd const_71_test/vyper && timeout 60 nix build
cd const_71_test/vyper && perf record -o data/71_flakes_perf/vyper_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/vyper && perf record -o data/71_flakes_perf/vyper_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/vyper_build.perf.data > data/71_results/vyper_analysis.txt

echo "🔬 Running workflow: 71_complete_xml"
cd const_71_test/xml && timeout 60 nix build
cd const_71_test/xml && perf record -o data/71_flakes_perf/xml_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/xml && perf record -o data/71_flakes_perf/xml_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/xml_build.perf.data > data/71_results/xml_analysis.txt

echo "🔬 Running workflow: 71_complete_yaml"
cd const_71_test/yaml && timeout 60 nix build
cd const_71_test/yaml && perf record -o data/71_flakes_perf/yaml_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/yaml && perf record -o data/71_flakes_perf/yaml_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/yaml_build.perf.data > data/71_results/yaml_analysis.txt

echo "🔬 Running workflow: 71_complete_z3"
cd const_71_test/z3 && timeout 60 nix build
cd const_71_test/z3 && perf record -o data/71_flakes_perf/z3_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/z3 && perf record -o data/71_flakes_perf/z3_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/z3_build.perf.data > data/71_results/z3_analysis.txt

echo "🔬 Running workflow: 71_complete_zsh"
cd const_71_test/zsh && timeout 60 nix build
cd const_71_test/zsh && perf record -o data/71_flakes_perf/zsh_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/zsh && perf record -o data/71_flakes_perf/zsh_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/zsh_build.perf.data > data/71_results/zsh_analysis.txt

echo "🔬 Running workflow: 71_complete_asm_aarch64"
cd const_71_test/asm_aarch64 && timeout 60 nix build
cd const_71_test/asm_aarch64 && perf record -o data/71_flakes_perf/asm_aarch64_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/asm_aarch64 && perf record -o data/71_flakes_perf/asm_aarch64_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/asm_aarch64_build.perf.data > data/71_results/asm_aarch64_analysis.txt

echo "🔬 Running workflow: 71_complete_asm_mips"
cd const_71_test/asm_mips && timeout 60 nix build
cd const_71_test/asm_mips && perf record -o data/71_flakes_perf/asm_mips_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/asm_mips && perf record -o data/71_flakes_perf/asm_mips_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/asm_mips_build.perf.data > data/71_results/asm_mips_analysis.txt

echo "🔬 Running workflow: 71_complete_asm_riscv"
cd const_71_test/asm_riscv && timeout 60 nix build
cd const_71_test/asm_riscv && perf record -o data/71_flakes_perf/asm_riscv_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/asm_riscv && perf record -o data/71_flakes_perf/asm_riscv_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/asm_riscv_build.perf.data > data/71_results/asm_riscv_analysis.txt

echo "🔬 Running workflow: 71_complete_asm_wasm"
cd const_71_test/asm_wasm && timeout 60 nix build
cd const_71_test/asm_wasm && perf record -o data/71_flakes_perf/asm_wasm_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/asm_wasm && perf record -o data/71_flakes_perf/asm_wasm_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/asm_wasm_build.perf.data > data/71_results/asm_wasm_analysis.txt

echo "🔬 Running workflow: 71_complete_asm_x86_64"
cd const_71_test/asm_x86_64 && timeout 60 nix build
cd const_71_test/asm_x86_64 && perf record -o data/71_flakes_perf/asm_x86_64_build.perf.data -F 99 -g nix build
nix-collect-garbage
cd const_71_test/asm_x86_64 && perf record -o data/71_flakes_perf/asm_x86_64_rebuild.perf.data -F 99 -g nix build --rebuild --no-substitute
./target/release/harmonic_analyzer data/71_flakes_perf/asm_x86_64_build.perf.data > data/71_results/asm_x86_64_analysis.txt

