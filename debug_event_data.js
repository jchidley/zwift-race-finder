// Examine the actual event data structure
// Run this in the browser console on https://www.zwift.com/uk/events/view/4967256

console.log('=== Examining Event Data Structure ===');
const nextData = JSON.parse(document.getElementById('__NEXT_DATA__').textContent);
const eventData = nextData.props.initialState.events.event.event;

console.log('Full event data:', eventData);

console.log('\n=== Key Event Fields ===');
console.log('Event ID:', eventData.id);
console.log('Name:', eventData.name);
console.log('Route ID:', eventData.routeId);
console.log('Distance in meters:', eventData.distanceInMeters);
console.log('Laps:', eventData.laps);
console.log('Category Enforcement:', eventData.categoryEnforcement);
console.log('Event Type:', eventData.eventType);

console.log('\n=== Subgroups (Categories) ===');
eventData.eventSubgroups.forEach((subgroup, i) => {
    console.log(`\nSubgroup ${i}:`);
    console.log('  Name:', subgroup.name);
    console.log('  Label:', subgroup.subgroupLabel);
    console.log('  Distance:', subgroup.distanceInMeters);
    console.log('  Laps:', subgroup.laps);
    console.log('  Range Access Label:', subgroup.rangeAccessLabel);
});

console.log('\n=== Looking for Route Details ===');
// Check if there's a routes lookup table
const state = nextData.props.initialState;
if (state.routes) {
    console.log('Found routes data:', state.routes);
}

// Check for route in other locations
const routeId = eventData.routeId;
console.log(`\nSearching for route ${routeId} details...`);

// Look through all state keys
Object.keys(state).forEach(key => {
    if (state[key] && typeof state[key] === 'object') {
        const str = JSON.stringify(state[key]);
        if (str.includes(routeId) && key !== 'events') {
            console.log(`Found route ID in state.${key}`);
        }
    }
});

console.log('\n=== Extracting Distance from Description ===');
// Since distance is in description, let's parse it
const description = eventData.description;
const stage4Match = description.match(/Stage 4:.*?Distance:\s*([\d.]+)\s*km/s);
if (stage4Match) {
    console.log('Parsed distance from description:', stage4Match[1], 'km');
}

// Extract all stage info
const stagePattern = /Stage (\d+):.*?Route:\s*([^\n]+).*?Laps:\s*(\d+).*?Distance:\s*([\d.]+)\s*km.*?Elevation:\s*(\d+)\s*m/gs;
let match;
console.log('\nAll stages from description:');
while ((match = stagePattern.exec(description)) !== null) {
    console.log(`Stage ${match[1]}: ${match[2]}`);
    console.log(`  Laps: ${match[3]}, Distance: ${match[4]} km, Elevation: ${match[5]} m`);
}

console.log('\n=== Where does 23.5 come from? ===');
// The webpage shows 23.5 km but the API has 0
// Let's check if it's calculated from laps
console.log('Event laps:', eventData.laps);
console.log('If route was 11.75 km per lap:', eventData.laps * 11.75, 'km total');

// Or maybe it's hardcoded in the UI
console.log('\nChecking UI elements for hardcoded values...');
const distanceElements = document.querySelectorAll('.group-value');
distanceElements.forEach(el => {
    if (el.textContent.includes('23.5')) {
        console.log('Found distance element:', el);
        console.log('Parent structure:', el.parentElement.parentElement.innerHTML);
    }
});