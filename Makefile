.PHONY: help build analyze-agda report baseline compare-ips test-all clean

help:
	@echo "Meta-Introspector Galois Analysis"
	@echo ""
	@echo "Phase 1 - Analysis Commands:"
	@echo "  make build         - Build all analyzers"
	@echo "  make analyze-agda  - Analyze Agda 71 perf data"
	@echo "  make report        - Generate full 71 language report"
	@echo "  make baseline      - Compare builds to baseline"
	@echo "  make compare-ips   - Analyze top IPs for Rust/Agda/Coq"
	@echo ""
	@echo "Testing:"
	@echo "  make test-all      - Test all 71 languages output '71'"
	@echo ""
	@echo "  make clean         - Clean build artifacts"

build:
	cargo build --release --bin harmonic_analyzer
	cargo build --release --bin galois_report
	cargo build --release --bin baseline_comparator
	cargo build --release --bin ip_galois
	cargo build --release --bin symbol_resolver

analyze-agda: build
	./target/release/harmonic_analyzer data/71_flakes_perf/agda_1768990025_build.perf.data

report: build
	./target/release/galois_report

baseline: build
	./target/release/baseline_comparator

compare-ips: build
	@echo "=== RUST ==="
	@./target/release/ip_galois data/71_flakes_perf/rust_1768414298_build.perf.data
	@echo ""
	@echo "=== AGDA ==="
	@./target/release/ip_galois data/71_flakes_perf/agda_1768990025_build.perf.data
	@echo ""
	@echo "=== COQ ==="
	@./target/release/ip_galois data/71_flakes_perf/coq_1768414198_build.perf.data

test-all:
	@echo "🧪 Testing all 71 languages..."
	@./test_all_71.sh

clean:
	cargo clean
