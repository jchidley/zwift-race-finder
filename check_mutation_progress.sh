#!/bin/bash
# Script to check mutation testing progress

echo "=== Mutation Testing Progress ==="
echo

# Check if any mutation processes are running
if pgrep -f "cargo-mutants" > /dev/null; then
    echo "✓ Mutation testing is currently running"
    echo
    echo "Active processes:"
    ps aux | grep -E "cargo-mutants|cargo mutants" | grep -v grep
    echo
else
    echo "✗ No mutation testing processes found"
    echo
fi

# Check log files
if [ -d mutation_logs ]; then
    echo "=== Log Files Status ==="
    for log in mutation_logs/*.log; do
        if [ -f "$log" ]; then
            echo
            echo "--- $(basename $log) ---"
            # Show last 10 lines and any summary info
            tail -n 10 "$log"
            
            # Check for completion markers
            if grep -q "Found .* mutants" "$log"; then
                echo "Mutants found: $(grep "Found .* mutants" "$log" | tail -1)"
            fi
            if grep -q "Killed:" "$log"; then
                echo "Progress: $(grep -c "Killed:" "$log") mutants tested"
            fi
            if grep -q "Final" "$log"; then
                echo "Status: COMPLETED"
                grep "Final" "$log"
            fi
        fi
    done
else
    echo "No mutation_logs directory found. Run ./run_mutation_testing.sh first."
fi

# Check mutants.out directory for detailed results
if [ -d mutants.out ]; then
    echo
    echo "=== Detailed Results ==="
    echo "Check mutants.out/ directory for:"
    echo "  - mutants.out/outcomes.json (detailed results)"
    echo "  - mutants.out/mutants.log (full log)"
    echo "  - mutants.out/*/build.log (individual build logs)"
fi