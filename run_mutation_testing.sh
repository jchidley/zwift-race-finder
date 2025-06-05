#!/bin/bash
# Script to run mutation testing in background with progress monitoring

# Create a log directory
mkdir -p mutation_logs

# Run mutation testing on each module
echo "Starting mutation testing on refactored modules..."

# Option 1: Run with nohup (survives terminal closure)
nohup cargo mutants --file src/models.rs --timeout 60 > mutation_logs/models.log 2>&1 &
echo "Models mutation testing PID: $!"

nohup cargo mutants --file src/category.rs --timeout 60 > mutation_logs/category.log 2>&1 &
echo "Category mutation testing PID: $!"

nohup cargo mutants --file src/parsing.rs --timeout 60 > mutation_logs/parsing.log 2>&1 &
echo "Parsing mutation testing PID: $!"

nohup cargo mutants --file src/cache.rs --timeout 60 > mutation_logs/cache.log 2>&1 &
echo "Cache mutation testing PID: $!"

# Option 2: Run all modules at once in background
# nohup cargo mutants --timeout 120 > mutation_logs/full_run.log 2>&1 &

echo "Mutation testing started in background. Check progress with:"
echo "  tail -f mutation_logs/*.log"
echo "  ps aux | grep cargo-mutants"
echo "  htop (press F4 and search for 'mutants')"