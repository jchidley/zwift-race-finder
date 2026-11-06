import assert from 'node:assert/strict';
import { test } from 'node:test';
import { evaluatePredictions } from '../tools/analysis/evaluate_predictions.mjs';

const race = (id, actual, minutes) => ({ target: { result_id: id, actual_seconds: actual }, B0: { raw_minutes: minutes, rounded_minutes: Math.round(minutes) } });

test('errors use raw prediction and measured subsecond duration', () => {
    const report = evaluatePredictions([race('1', 2532.517, 42.55529037689975), race('2', 2400.125, 39.75)]);
    const first = report.races[0].predictions.B0;
    assert.ok(Math.abs(first.signed_error_seconds - 20.80042261398512) < 1e-9);
    assert.equal(first.absolute_error_seconds, first.signed_error_seconds);
    assert.equal(report.races[1].predictions.B0.signed_error_seconds, -15.125);
    assert.ok(Math.abs(report.summary.B0.mae_seconds - (20.80042261398512 + 15.125) / 2) < 1e-9);
});

test('window decisions use raw values at both inclusive boundaries', () => {
    const report = evaluatePredictions([race('1', 2400, 39.99), race('2', 3600, 60.01), race('3', 2400, 40), race('4', 3600, 60)]);
    assert.deepEqual(report.summary.B0.windows['40-60'], { suitable_shown: 2, suitable_missed: 2, unsuitable_included: 0, unavailable_suitable: 0 });
});

test('rounded fields cannot affect errors or selection', () => {
    const row = race('1', 2399.99, 40.001);
    const before = evaluatePredictions([row]).summary;
    row.B0.rounded_minutes = 900;
    assert.deepEqual(evaluatePredictions([row]).summary, before);
    assert.equal(before.B0.windows['40-60'].unsuitable_included, 1);
});

test('unavailability is counted without inventing zero error', () => {
    const row = { target: { result_id: '1', actual_seconds: 2500 }, B0: { unavailable: 'No geometry' } };
    const summary = evaluatePredictions([row]).summary.B0;
    assert.equal(summary.mae_seconds, null);
    assert.equal(summary.supported, 0);
    assert.equal(summary.windows['40-60'].unavailable_suitable, 1);
});

test('reject empty, duplicate, invalid, inconsistent or rounded-only inputs', () => {
    assert.throws(() => evaluatePredictions([]));
    assert.throws(() => evaluatePredictions([race('1', 2500, 40), race('1', 2500, 40)]));
    for (const invalid of [NaN, Infinity, -1, 0, '2500']) {
        assert.throws(() => evaluatePredictions([race('1', invalid, 40)]));
        assert.throws(() => evaluatePredictions([race('1', 2500, invalid)]));
    }
    assert.throws(() => evaluatePredictions([{ target: { result_id: '1', actual_seconds: 2500 }, B0: { rounded_minutes: 40 } }]));
    const row = race('2', 2500, 40); row.R1 = { raw_minutes: 40 };
    assert.throws(() => evaluatePredictions([race('1', 2500, 40), row]));
});
