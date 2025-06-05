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
    LOG="mutation_logs/full_run.log"
    if [ -f "$LOG" ]; then
        echo
        echo "--- Mutation Testing Progress ---"
        # Show last 20 lines for better context
        tail -n 20 "$LOG"
        echo
        
        # Extract progress information
        if grep -q "Found .* mutants" "$LOG"; then
            TOTAL=$(grep "Found .* mutants" "$LOG" | tail -1 | grep -oE "[0-9]+ mutants" | grep -oE "[0-9]+")
            echo "Total mutants: $TOTAL"
        fi
        
        # Count different outcomes based on the actual log format
        # Format is: STATUS  src/file.rs:line:col: description
        KILLED=$(grep -cE "^(KILLED|ok)" "$LOG" 2>/dev/null | tr -d '[:space:]' || echo "0")
        SURVIVED=$(grep -cE "^SURVIVED" "$LOG" 2>/dev/null | tr -d '[:space:]' || echo "0")
        TIMEOUT=$(grep -cE "^TIMEOUT" "$LOG" 2>/dev/null | tr -d '[:space:]' || echo "0")
        UNVIABLE=$(grep -cE "^UNVIABLE" "$LOG" 2>/dev/null | tr -d '[:space:]' || echo "0")
        
        # Ensure values are numeric
        KILLED=${KILLED:-0}
        SURVIVED=${SURVIVED:-0}
        TIMEOUT=${TIMEOUT:-0}
        UNVIABLE=${UNVIABLE:-0}
        
        echo "Progress:"
        echo "  Killed: $KILLED"
        echo "  Survived: $SURVIVED"
        echo "  Timeout: $TIMEOUT"
        echo "  Unviable: $UNVIABLE"
        echo "  Total processed: $((KILLED + SURVIVED + TIMEOUT + UNVIABLE))"
        
        # Check if completed
        if grep -q "mutants tested" "$LOG" || grep -q "Finished" "$LOG"; then
            echo
            echo "Status: COMPLETED"
            grep -A 5 "mutants tested" "$LOG" | tail -10
        fi
        
        # Show current/recent activity
        echo
        echo "Recent activity:"
        grep -E "^(KILLED|SURVIVED|TIMEOUT|UNVIABLE|ok)" "$LOG" | tail -5
    else
        echo "Log file not found. Waiting for mutation testing to start..."
    fi
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