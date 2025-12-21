import init, { generate_maze_image, generate_maze_with_solution_image, get_algorithms } from './pkg/maze_generator.js';

let wasmInitialized = false;
let currentImageData = null;
let unsolvedImageData = null;
let solvedImageData = null;
let showingSolution = false;
let lastGenerationParams = null;
let currentSeed = null;

// Initialize WASM module
async function initWasm() {
    if (wasmInitialized) return;
    
    try {
        await init();
        wasmInitialized = true;
        populateAlgorithms();
        console.log('WASM module initialized');
    } catch (error) {
        showError('Failed to initialize WASM module: ' + error.message);
        console.error('WASM initialization error:', error);
    }
}

// Populate algorithm dropdown
function populateAlgorithms() {
    const select = document.getElementById('algorithm');
    const algorithms = get_algorithms();
    
    algorithms.forEach(alg => {
        const option = document.createElement('option');
        option.value = alg;
        option.textContent = formatAlgorithmName(alg);
        select.appendChild(option);
    });
    
    // Set default
    select.value = 'recursive_backtracking';
}

// Format algorithm name for display
function formatAlgorithmName(name) {
    return name
        .split('_')
        .map(word => word.charAt(0).toUpperCase() + word.slice(1))
        .join(' ');
}

// Update complexity display
function updateComplexityDisplay() {
    const slider = document.getElementById('complexity');
    const display = document.getElementById('complexity-value');
    const value = (parseFloat(slider.value) / 100).toFixed(2);
    display.textContent = value;
}

// Update line thickness display
function updateLineThicknessDisplay() {
    const slider = document.getElementById('line-thickness');
    const display = document.getElementById('line-thickness-value');
    const value = (parseFloat(slider.value) / 100).toFixed(2);
    display.textContent = value;
}

// Sync color picker and hex input
function syncColorInputs() {
    const colorPicker = document.getElementById('line-color');
    const hexInput = document.getElementById('line-color-hex');
    
    colorPicker.addEventListener('input', (e) => {
        hexInput.value = e.target.value;
    });
    
    hexInput.addEventListener('input', (e) => {
        const value = e.target.value;
        if (/^#[0-9A-Fa-f]{6}$/.test(value)) {
            colorPicker.value = value;
        }
    });
}

// Display image on canvas
function displayImage(imageBytes) {
    const canvas = document.getElementById('maze-canvas');
    const ctx = canvas.getContext('2d');
    
    // Create image from bytes
    const blob = new Blob([imageBytes], { type: 'image/png' });
    const url = URL.createObjectURL(blob);
    const img = new Image();
    
    img.onload = () => {
        canvas.width = img.width;
        canvas.height = img.height;
        ctx.drawImage(img, 0, 0);
        URL.revokeObjectURL(url);
        
        // Store image data for download
        currentImageData = imageBytes;
        document.getElementById('download-btn').disabled = false;
    };
    
    img.onerror = () => {
        showError('Failed to load generated image');
        URL.revokeObjectURL(url);
    };
    
    img.src = url;
}

// Display seed below the image
function displaySeed(seed) {
    const seedDisplay = document.getElementById('seed-display');
    if (seed !== null) {
        seedDisplay.textContent = `Seed: ${seed} (use this seed to reproduce this maze)`;
        seedDisplay.classList.remove('hidden');
    } else {
        seedDisplay.classList.add('hidden');
    }
}

// Download current image
function downloadImage() {
    if (!currentImageData) return;
    
    const blob = new Blob([currentImageData], { type: 'image/png' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'maze.png';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
}

// Show error message
function showError(message) {
    const errorDiv = document.getElementById('error');
    errorDiv.textContent = message;
    errorDiv.classList.remove('hidden');
    setTimeout(() => {
        errorDiv.classList.add('hidden');
    }, 5000);
}

// Hide error message
function hideError() {
    document.getElementById('error').classList.add('hidden');
}

// Show loading indicator
function showLoading() {
    document.getElementById('loading').classList.remove('hidden');
    document.getElementById('generate-btn').disabled = true;
}

// Hide loading indicator
function hideLoading() {
    document.getElementById('loading').classList.add('hidden');
    document.getElementById('generate-btn').disabled = false;
}

// Generate maze - generates both versions
async function generateMaze(formData) {
    if (!wasmInitialized) {
        showError('WASM module not initialized. Please refresh the page.');
        return;
    }
    
    showLoading();
    hideError();
    
    try {
        const width = parseInt(formData.get('width'));
        const height = parseInt(formData.get('height'));
        const algorithm = formData.get('algorithm');
        const complexity = parseFloat(formData.get('complexity')) / 100;
        const cellSize = parseInt(formData.get('cell-size'));
        const seedInput = formData.get('seed');
        let seed = null;
        let seedValue = null;
        
        // If seed provided by user, use it
        if (seedInput && seedInput !== '') {
            seedValue = parseInt(seedInput);
            seed = BigInt(seedValue);
        } else {
            // If no seed provided, generate one internally to ensure both versions use the same maze
            // Generate a random seed and convert to BigInt
            seedValue = Math.floor(Math.random() * 4294967296); // 32-bit range for compatibility
            seed = BigInt(seedValue);
            // Don't populate the input field - user wants different mazes each time
        }
        
        // Store the seed value for display
        currentSeed = seedValue;
        
        const lineColor = formData.get('line-color');
        const lineThickness = parseFloat(formData.get('line-thickness')) / 100;
        
        // Store parameters (store the actual seed used)
        lastGenerationParams = {
            width,
            height,
            algorithm,
            complexity,
            cellSize,
            seed,
            lineColor,
            lineThickness
        };
        
        // Generate maze
        unsolvedImageData = generate_maze_image(
            width,
            height,
            algorithm,
            complexity,
            seed,
            cellSize
        );
        
        solvedImageData = generate_maze_with_solution_image(
            width,
            height,
            algorithm,
            complexity,
            seed,
            cellSize,
            lineColor,
            lineThickness
        );
        
        // Start by showing unsolved version
        showingSolution = false;
        currentImageData = unsolvedImageData;
        displayImage(unsolvedImageData);
        
        // Display seed for reproducibility
        displaySeed(currentSeed);
        
        // Enable toggle button
        const toggleBtn = document.getElementById('toggle-solution-btn');
        toggleBtn.disabled = false;
        toggleBtn.textContent = 'Show Solution';
    } catch (error) {
        showError('Error generating maze: ' + error.message);
        console.error('Maze generation error:', error);
    } finally {
        hideLoading();
    }
}

// Toggle solution display - switches between stored images
function toggleSolution() {
    if (!unsolvedImageData || !solvedImageData) {
        return;
    }
    
    showingSolution = !showingSolution;
    
    if (showingSolution) {
        currentImageData = solvedImageData;
        displayImage(solvedImageData);
        document.getElementById('toggle-solution-btn').textContent = 'Hide Solution';
    } else {
        currentImageData = unsolvedImageData;
        displayImage(unsolvedImageData);
        document.getElementById('toggle-solution-btn').textContent = 'Show Solution';
    }
}

// Event listeners
document.addEventListener('DOMContentLoaded', () => {
    // Initialize WASM
    initWasm();
    
    // Form submission
    document.getElementById('maze-form').addEventListener('submit', (e) => {
        e.preventDefault();
        const formData = new FormData(e.target);
        generateMaze(formData);
    });
    
    // Complexity slider
    document.getElementById('complexity').addEventListener('input', updateComplexityDisplay);
    updateComplexityDisplay();
    
    // Line thickness slider
    document.getElementById('line-thickness').addEventListener('input', updateLineThicknessDisplay);
    updateLineThicknessDisplay();
    
    // Color inputs sync
    syncColorInputs();
    
    // Toggle solution button
    document.getElementById('toggle-solution-btn').addEventListener('click', toggleSolution);
    
    // Download button
    document.getElementById('download-btn').addEventListener('click', downloadImage);
});

