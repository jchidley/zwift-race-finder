#!/bin/bash

set -e

echo "🚴 Initializing Zwift Race Finder repository..."

git init

git add .

git commit -m "feat: initial commit of Zwift Race Finder

- CLI tool to find Zwift races by duration and racing score
- Route-aware duration estimation with elevation factors
- SQLite database for route data and race history
- Import functionality for ZwiftPower race results
- Regression testing framework using actual race data
- CI/CD workflow with GitHub Actions
- Dual MIT/Apache-2.0 licensing"

echo ""
echo "✅ Repository initialized!"
echo ""
echo "Next steps:"
echo "1. Create a new repository on GitHub: https://github.com/new"
echo "   - Name: zwift-racing"
echo "   - Keep it private or public as you prefer"
echo "   - Don't initialize with README, .gitignore, or license"
echo ""
echo "2. Add the remote and push:"
echo "   git remote add origin https://github.com/jchidley/zwift-racing.git"
echo "   git branch -M main"
echo "   git push -u origin main"
echo ""
echo "3. Check GitHub Actions at:"
echo "   https://github.com/jchidley/zwift-racing/actions"