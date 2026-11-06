import { readFileSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

export const WINDOWS = [[20, 40], [40, 50], [50, 70], [75, 105], [40, 60]];

function positive(value, label) {
    if (typeof value !== 'number' || !Number.isFinite(value) || value <= 0) {
        throw new Error(`${label} must be a finite positive number`);
    }
    return value;
}

export function evaluatePredictions(records) {
    if (!Array.isArray(records) || records.length === 0) {
        throw new Error('At least one measured race and prediction are required');
    }
    const models = Object.keys(records[0]).filter(key => {
        const value = records[0][key];
        return value && typeof value === 'object' && ('raw_minutes' in value || 'unavailable' in value);
    }).sort();
    if (!models.length) throw new Error('No raw model predictions found');
    const seen = new Set();
    const races = records.map(record => {
        const target = record.target;
        if (!target?.result_id || seen.has(target.result_id)) {
            throw new Error('Each race must have a unique source result_id');
        }
        seen.add(target.result_id);
        const actual = positive(target.actual_seconds, 'target.actual_seconds');
        const predictions = {};
        const recordModels = Object.keys(record).filter(key => {
            const value = record[key];
            return value && typeof value === 'object' && ('raw_minutes' in value || 'unavailable' in value);
        }).sort();
        if (JSON.stringify(recordModels) !== JSON.stringify(models)) {
            throw new Error('Every race must contain the same models');
        }
        for (const model of models) {
            const prediction = record[model];
            if ('unavailable' in prediction) {
                if ('raw_minutes' in prediction || typeof prediction.unavailable !== 'string' || !prediction.unavailable.trim()) {
                    throw new Error(`${model}: ambiguous or missing unavailability reason`);
                }
                predictions[model] = { unavailable: prediction.unavailable };
                continue;
            }
            const seconds = positive(positive(prediction.raw_minutes, `${model}.raw_minutes`) * 60, 'predicted_seconds');
            const error = seconds - actual;
            predictions[model] = {
                predicted_seconds: seconds,
                signed_error_seconds: error,
                absolute_error_seconds: Math.abs(error),
            };
        }
        return { target, predictions };
    });
    const summary = {};
    for (const model of models) {
        const supported = races.filter(race => !('unavailable' in race.predictions[model]));
        const mean = key => supported.length
            ? supported.reduce((sum, race) => sum + race.predictions[model][key], 0) / supported.length
            : null;
        const windows = {};
        for (const [lo, hi] of WINDOWS) {
            const actualMatch = race => lo * 60 <= race.target.actual_seconds && race.target.actual_seconds <= hi * 60;
            const predictedMatch = race => {
                const seconds = race.predictions[model].predicted_seconds;
                return seconds !== undefined && lo * 60 <= seconds && seconds <= hi * 60;
            };
            windows[`${lo}-${hi}`] = {
                suitable_shown: races.filter(r => actualMatch(r) && predictedMatch(r)).length,
                suitable_missed: races.filter(r => actualMatch(r) && !predictedMatch(r)).length,
                unsuitable_included: races.filter(r => !actualMatch(r) && predictedMatch(r)).length,
                unavailable_suitable: races.filter(r => actualMatch(r) && 'unavailable' in r.predictions[model]).length,
            };
        }
        summary[model] = {
            targets: races.length,
            supported: supported.length,
            unavailable: races.length - supported.length,
            mae_seconds: mean('absolute_error_seconds'),
            mean_signed_error_seconds: mean('signed_error_seconds'),
            windows,
        };
    }
    return { precision: 'unrounded predictions and measured seconds; positive error means predicted too long', races, summary };
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
    const [input, output, ...extra] = process.argv.slice(2);
    if (!input || !output || extra.length) {
        throw new Error('Usage: node tools/analysis/evaluate_predictions.mjs INPUT.jsonl OUTPUT.json (new private output path)');
    }
    const records = readFileSync(input, 'utf8').split(/\r?\n/).filter(line => line.trim()).map(line => JSON.parse(line));
    const report = evaluatePredictions(records);
    writeFileSync(output, `${JSON.stringify(report, null, 2)}\n`, { flag: 'wx', mode: 0o600 });
    console.log(JSON.stringify(report.summary, null, 2));
}
