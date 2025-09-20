
.PHONY: run diagram diagram-verbose

# Run the Rust application
run:
	cargo run

# Generate the ns-r diagram with standard logging
diagram:
	python3 visualization/visualize.py

# Generate the ns-r diagram with verbose (debug) logging
diagram-verbose:
	python3 visualization/visualize.py -v

