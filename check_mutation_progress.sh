#!/bin/bash
# ABOUTME: Monitor mutation testing progress with new directory structure
# Usage: ./check_mutation_progress.sh

set -euo pipefail
IFS=$'\n\t'

# Script directory for relative paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MUTATION_BASE="${SCRIPT_DIR}/mutation_results"

echo "=== Mutation Testing Progress ==="
echo

# Check if any mutation processes are running
if pgrep -f "cargo.mutants" > /dev/null 2>&1; then
    echo "✓ Mutation testing is currently running"
    echo
    echo "Active processes:"
    ps aux | grep -E "cargo.mutants|cargo mutants" | grep -v grep
    echo
else
    echo "✗ No mutation testing processes found"
    # Check for PID file in current run
    if [ -f "${MUTATION_BASE}/current/mutation.pid" ]; then
        PID=$(cat "${MUTATION_BASE}/current/mutation.pid")
        if ! ps -p "$PID" > /dev/null 2>&1; then
            echo "  (Process $PID has finished)"
        fi
    fi
    echo
fi

# Check current run directory
if [ -L "${MUTATION_BASE}/current" ] && [ -d "${MUTATION_BASE}/current" ]; then
    CURRENT_RUN=$(readlink -f "${MUTATION_BASE}/current")
    echo "Current run: $(basename "$CURRENT_RUN")"
    echo
    
    # Check main log file
    LOG="${CURRENT_RUN}/mutation_run.log"
    if [ -f "$LOG" ]; then
        echo "=== Recent Activity ==="
        tail -n 20 "$LOG"
        echo
    fi
    
    # Check mutants.out directory within the run
    MUTANTS_OUT="${CURRENT_RUN}/mutants.out"
    if [ -d "$MUTANTS_OUT" ]; then
        echo "=== Mutation Testing Statistics ==="
        
        # Count outcomes from files
        if [ -f "${MUTANTS_OUT}/caught.txt" ]; then
            CAUGHT=$(wc -l < "${MUTANTS_OUT}/caught.txt" 2>/dev/null || echo 0)
        else
            CAUGHT=0
        fi
        
        if [ -f "${MUTANTS_OUT}/missed.txt" ]; then
            MISSED=$(wc -l < "${MUTANTS_OUT}/missed.txt" 2>/dev/null || echo 0)
        else
            MISSED=0
        fi
        
        if [ -f "${MUTANTS_OUT}/timeout.txt" ]; then
            TIMEOUT=$(wc -l < "${MUTANTS_OUT}/timeout.txt" 2>/dev/null || echo 0)
        else
            TIMEOUT=0
        fi
        
        if [ -f "${MUTANTS_OUT}/unviable.txt" ]; then
            UNVIABLE=$(wc -l < "${MUTANTS_OUT}/unviable.txt" 2>/dev/null || echo 0)
        else
            UNVIABLE=0
        fi
        
        TOTAL=$((CAUGHT + MISSED + TIMEOUT + UNVIABLE))
        
        echo "Progress:"
        echo "  Caught (killed by tests): $CAUGHT"
        echo "  Missed (survived tests): $MISSED"
        echo "  Timeout: $TIMEOUT"
        echo "  Unviable (won't compile): $UNVIABLE"
        echo "  Total processed: $TOTAL"
        
        # Calculate percentage if we have results
        if [ "$TOTAL" -gt 0 ]; then
            COVERAGE=$(awk "BEGIN {printf \"%.1f\", ($CAUGHT / ($CAUGHT + $MISSED)) * 100}")
            echo
            echo "Mutation coverage: ${COVERAGE}%"
        fi
        
        # Check if testing is complete
        if [ -f "${MUTANTS_OUT}/outcomes.json" ]; then
            if grep -q '"phase":"test"' "${MUTANTS_OUT}/outcomes.json" && ! pgrep -f "cargo.mutants" > /dev/null 2>&1; then
                echo
                echo "Status: COMPLETED"
            fi
        fi
        
        # Show recent missed mutants
        if [ -f "${MUTANTS_OUT}/missed.txt" ] && [ "$MISSED" -gt 0 ]; then
            echo
            echo "=== Recent Missed Mutants ==="
            tail -n 5 "${MUTANTS_OUT}/missed.txt"
        fi
    else
        echo "Waiting for mutation testing to generate results..."
    fi
    
    echo
    echo "View detailed results:"
    echo "  cat ${MUTANTS_OUT}/missed.txt     # Mutations that survived"
    echo "  cat ${MUTANTS_OUT}/caught.txt     # Mutations caught by tests"
    echo "  less ${MUTANTS_OUT}/outcomes.json # Full JSON results"
    echo "  tail -f ${LOG}                    # Follow progress"
    
else
    echo "No active mutation testing run found."
    echo
    
    # List recent runs
    if [ -d "$MUTATION_BASE" ]; then
        echo "Recent runs:"
        ls -lt "$MUTATION_BASE" | grep "^d" | grep "run_" | head -5
    fi
    
    echo
    echo "To start a new run: ./run_mutation_testing.sh"
fi