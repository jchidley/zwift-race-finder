# OCR Mutation Testing Comprehensive Plan

## Date: 2025-01-10 07:45:00

## Understanding the Reality

### Time Investment
- **Full codebase**: 3+ hours (last run: 3h 7m for 1051 mutations)
- **OCR module only**: Estimated 30-60 minutes
- **Background execution**: Yes, using nohup
- **Monitoring**: check_mutation_progress.sh every 10-15 minutes

### Complexity Acceptance
1. **Code Movement**: OCR code may move during the run
2. **Line Number Drift**: Even small changes shift mutation locations
3. **Mapping Required**: Must track function movements post-run
4. **No Shortcuts**: This IS difficult and long-winded but necessary

## Execution Plan

### Phase 1: Pre-Flight Setup (10 minutes)
1. **Git Tag Current State**
   ```bash
   git tag mutation-ocr-start-20250110
   git status  # Ensure clean working tree
   ```

2. **Verify OCR Module Structure**
   - List all OCR-related files
   - Count approximate lines of code
   - Identify test files that will catch mutations

3. **Configure for OCR Focus**
   - Create targeted mutation config
   - Limit scope to OCR files only

### Phase 2: Launch Background Mutation Testing (5 minutes)
1. **Create OCR-Specific Run Script**
   ```bash
   #!/bin/bash
   # run_ocr_mutation_testing.sh
   
   TIMESTAMP=$(date +%Y%m%d_%H%M%S)
   OUTPUT_DIR="mutation_results/ocr_run_${TIMESTAMP}"
   mkdir -p "$OUTPUT_DIR"
   
   # Run mutation testing ONLY on OCR files
   nohup cargo mutants \
       --file src/ocr_compact.rs \
       --file src/ocr_constants.rs \
       --file src/ocr_image_processing.rs \
       --file src/ocr_ocrs.rs \
       --file src/ocr_parallel.rs \
       --file src/ocr_regex.rs \
       --output "$OUTPUT_DIR" \
       --jobs 8 \
       --timeout 180 \
       > "${OUTPUT_DIR}/mutation_run.log" 2>&1 &
   
   PID=$!
   echo "OCR Mutation testing started with PID: $PID"
   echo "$PID" > "${OUTPUT_DIR}/mutation.pid"
   ln -sfn "$OUTPUT_DIR" "mutation_results/ocr_current"
   ```

2. **Launch and Monitor**
   - Execute the script
   - Note the PID
   - Initial progress check

### Phase 3: Active Monitoring (30-60 minutes)
1. **Regular Progress Checks**
   ```bash
   # Every 10 minutes
   ./check_mutation_progress.sh
   
   # Watch specific metrics
   watch -n 30 'grep -c "MISSED\|CAUGHT" mutation_results/ocr_current/mutation_run.log'
   ```

2. **Document Progress**
   - Create progress log with timestamps
   - Note any errors or timeouts
   - Track mutation counts

### Phase 4: Post-Run Analysis (45 minutes)
1. **Capture Final Results**
   ```bash
   # Copy results before any code changes
   cp -r mutation_results/ocr_current /tmp/ocr_mutations_backup
   ```

2. **Code Movement Mapping**
   ```bash
   # Check for any code changes during run
   git diff mutation-ocr-start-20250110
   
   # If changes occurred, create mapping document
   ```

3. **Categorize Missed Mutations**
   - Group by mutation type (arithmetic, comparison, boolean)
   - Identify patterns (e.g., all boundary checks missing)
   - Prioritize by impact on OCR accuracy

### Phase 5: Targeted Test Creation (90 minutes)
1. **High-Priority Mutations** (30 minutes)
   - Arithmetic in parse functions
   - Comparison operators in validation
   - Boolean logic in name detection

2. **Medium-Priority Mutations** (30 minutes)
   - String manipulation boundaries
   - Regex pattern mutations
   - Threshold value changes

3. **Low-Priority Mutations** (30 minutes)
   - Logging/debug code
   - Error message formatting
   - Performance optimizations

### Phase 6: Verification Run (30 minutes)
1. **Re-run Mutation Testing**
   - Same scope as before
   - Compare caught vs missed
   - Document improvements

## Handling Common Challenges

### If Code Moves During Testing
1. Use git diff to identify changes
2. Create mapping table:
   ```
   Old Location → New Location
   ocr_compact.rs:123 → ocr_parsing.rs:145
   ```
3. Apply mutations to new locations

### If Testing Takes Too Long
1. Continue monitoring in background
2. Work on other tasks
3. Check progress hourly
4. Let it complete overnight if needed

### If Many Mutations Survive
1. Remember: 70% rule applies
2. Focus on business-critical paths
3. Don't chase 100% mutation coverage
4. Document why mutations were ignored

## Success Criteria

### Quantitative
- Mutation score for critical functions > 75%
- All arithmetic operations in parsing tested
- All boundary conditions verified
- No timeouts in OCR tests

### Qualitative
- Confidence in OCR accuracy validation
- Tests catch real parsing errors
- Clear documentation of decisions
- Reproducible process for future runs

## Timeline

**Total Time**: 4-5 hours (mostly waiting)
- Setup: 15 minutes
- Monitoring: 30-60 minutes (intermittent)
- Analysis: 45 minutes
- Test Writing: 90 minutes
- Verification: 30 minutes

## Key Commands Reference

```bash
# Start OCR mutation testing
./run_ocr_mutation_testing.sh

# Monitor progress
./check_mutation_progress.sh
tail -f mutation_results/ocr_current/mutation_run.log

# View results
cat mutation_results/ocr_current/mutants.out/missed.txt
cat mutation_results/ocr_current/mutants.out/caught.txt
jq '.summary' mutation_results/ocr_current/mutants.out/outcomes.json

# Stop if needed
kill $(cat mutation_results/ocr_current/mutation.pid)
```

## Commitment

This plan accepts:
1. ✅ The time investment (4-5 hours total)
2. ✅ The complexity of matching mutations to moved code
3. ✅ The necessity of comprehensive testing
4. ✅ Running in background while doing other work
5. ✅ The difficulty of the task

No shortcuts. No abandonment. Complete execution.