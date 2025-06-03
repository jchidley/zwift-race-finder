// Deep dive into Next.js data to find route information
// Run this in the browser console on https://www.zwift.com/uk/events/view/4967256

console.log('=== Extracting Full Next.js Data ===');
const nextData = JSON.parse(document.getElementById('__NEXT_DATA__').textContent);
console.log('Full Next.js data:', nextData);

// Function to search for values in nested objects
const searchInObject = (obj, searchTerm, path = '') => {
    const results = [];
    
    const search = (currentObj, currentPath) => {
        for (let key in currentObj) {
            const newPath = currentPath ? `${currentPath}.${key}` : key;
            
            if (currentObj[key] === searchTerm || 
                (typeof currentObj[key] === 'string' && currentObj[key].includes(searchTerm))) {
                results.push({
                    path: newPath,
                    value: currentObj[key],
                    parent: currentPath
                });
            }
            
            if (typeof currentObj[key] === 'object' && currentObj[key] !== null && !Array.isArray(currentObj[key])) {
                search(currentObj[key], newPath);
            } else if (Array.isArray(currentObj[key])) {
                currentObj[key].forEach((item, index) => {
                    if (typeof item === 'object' && item !== null) {
                        search(item, `${newPath}[${index}]`);
                    }
                });
            }
        }
    };
    
    search(obj, path);
    return results;
};

console.log('\n=== Searching for Route ID ===');
const routeResults = searchInObject(nextData, '3379779247');
routeResults.forEach(result => {
    console.log(`Found route ID at: ${result.path}`);
    console.log('Context:', result);
});

console.log('\n=== Searching for Distance Values ===');
// Search for various distance representations
const distanceSearches = [23.5, '23.5', 23500, '23500', 'distance', 'Distance'];
distanceSearches.forEach(term => {
    const results = searchInObject(nextData, term);
    if (results.length > 0) {
        console.log(`\nFound "${term}":`);
        results.forEach(r => console.log(`  at: ${r.path}`));
    }
});

console.log('\n=== Searching for Route Name ===');
const routeNameResults = searchInObject(nextData, 'Three Village Loop');
routeNameResults.forEach(result => {
    console.log(`Found route name at: ${result.path}`);
});

console.log('\n=== Looking for Apollo Cache ===');
// Next.js often uses Apollo for GraphQL
if (nextData.props && nextData.props.__APOLLO_STATE__) {
    console.log('Found Apollo state:', nextData.props.__APOLLO_STATE__);
} else {
    console.log('No Apollo state found');
}

console.log('\n=== Checking for Route-related Keys ===');
// Look for any keys that might contain route data
const findKeysContaining = (obj, searchTerms, path = '') => {
    const results = [];
    
    const search = (currentObj, currentPath) => {
        for (let key in currentObj) {
            const lowerKey = key.toLowerCase();
            if (searchTerms.some(term => lowerKey.includes(term))) {
                results.push({
                    key: key,
                    path: currentPath ? `${currentPath}.${key}` : key,
                    value: currentObj[key]
                });
            }
            
            if (typeof currentObj[key] === 'object' && currentObj[key] !== null) {
                search(currentObj[key], currentPath ? `${currentPath}.${key}` : key);
            }
        }
    };
    
    search(obj, path);
    return results;
};

const routeKeys = findKeysContaining(nextData, ['route', 'distance', 'elevation', 'lap']);
console.log('Found route-related keys:', routeKeys);

console.log('\n=== Checking Window for Zwift Data ===');
// Look for Zwift-specific globals
const zwiftGlobals = Object.keys(window).filter(key => 
    key.toLowerCase().includes('zwift') || 
    key.toLowerCase().includes('route') ||
    key.toLowerCase().includes('event')
);
console.log('Zwift-related window properties:', zwiftGlobals);

// Check if route data might be loaded asynchronously
console.log('\n=== Checking for Dynamic Data Loading ===');
if (window.__remixContext) {
    console.log('Found Remix context:', window.__remixContext);
}
if (window.__remixRouteModules) {
    console.log('Found Remix route modules:', window.__remixRouteModules);
}