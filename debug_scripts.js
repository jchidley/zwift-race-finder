// Debug script to find where Zwift webpage gets route data
// Run this in the browser console on https://www.zwift.com/uk/events/view/4967256

console.log('=== Checking for page state data ===');
console.log('window.__INITIAL_STATE__:', window.__INITIAL_STATE__);
console.log('window.__NEXT_DATA__:', window.__NEXT_DATA__);

// Get Next.js data
const nextDataElement = document.getElementById('__NEXT_DATA__');
if (nextDataElement) {
    const nextData = JSON.parse(nextDataElement.textContent);
    console.log('Next.js pageProps:', nextData.props.pageProps);
    
    // Look for route data
    if (nextData.props.pageProps.routes) {
        console.log('Found routes data:', nextData.props.pageProps.routes);
    }
}

// Search for route ID in window
console.log('\n=== Searching for route ID ===');
const routeId = '3379779247';
for (let key in window) {
    if (window[key] && typeof window[key] === 'object') {
        try {
            let str = JSON.stringify(window[key]);
            if (str && str.includes(routeId)) {
                console.log(`Found routeId in window.${key}`);
            }
        } catch (e) {
            // Skip circular references
        }
    }
}

// Find where distance is displayed
console.log('\n=== Finding distance elements ===');
const distanceText = '23.5';
document.querySelectorAll('*').forEach(el => {
    if (el.textContent && el.textContent.includes(distanceText) && el.children.length === 0) {
        console.log('Distance found in:', el);
        console.log('Parent element:', el.parentElement);
        console.log('Parent classes:', el.parentElement.className);
    }
});

// Look for API calls in performance entries
console.log('\n=== Recent API calls ===');
performance.getEntriesByType('resource').forEach(entry => {
    if (entry.name.includes('/api/') || entry.name.includes('route')) {
        console.log('API call:', entry.name);
    }
});

// Check for route mapping
console.log('\n=== Checking for route mappings ===');
const routeIds = ['3379779247']; // Three Village Loop
const searchText = JSON.stringify(window);
if (searchText.includes('Three Village Loop')) {
    console.log('Route name "Three Village Loop" found in page data');
}

// Try to find React component data
console.log('\n=== Looking for React components ===');
const findReactFiber = (el) => {
    for (const key in el) {
        if (key.startsWith('__reactInternalInstance') || key.startsWith('__reactFiber')) {
            return el[key];
        }
    }
    return null;
};

// Find elements containing our data
const elements = Array.from(document.querySelectorAll('*')).filter(el => 
    el.textContent && (
        el.textContent.includes('23.5') || 
        el.textContent.includes('160-270') ||
        el.textContent.includes('Three Village Loop')
    )
);

elements.forEach((el, i) => {
    const fiber = findReactFiber(el);
    if (fiber && fiber.memoizedProps) {
        console.log(`Element ${i} React props:`, fiber.memoizedProps);
    }
});