PROJECT_NAME := maze_generator
CARGO := cargo
TARGET_RELEASE := target/release/${PROJECT_NAME}
TARGET_DEBUG := target/debug/${PROJECT_NAME}

# Output directory
IMG_DIR := .img

# Output files
MAZE_PNG := ${IMG_DIR}/maze.png
MAZE_SOLVED_PNG := ${IMG_DIR}/maze_solved.png

.PHONY: release debug clean maze all-algorithms all-mazes benchmark help images regenerate-images regenerate-images-push \
	size-examples size-example-50 size-example-100 size-example-200 size-example-500 size-example-1000 \
	complexity-examples algorithm-examples serve-docs

# Build targets
release: ${TARGET_RELEASE}

${TARGET_RELEASE}:
	${CARGO} build --release

debug: ${TARGET_DEBUG}

${TARGET_DEBUG}:
	${CARGO} build

# Benchmark targets
benchmark:
	${CARGO} bench

benchmark-quick:
	${CARGO} bench --bench maze_generation -- --sample-size 10

# Default maze generation (uses config.toml) - always generates solved version
maze: ${TARGET_RELEASE} ${MAZE_PNG} ${MAZE_SOLVED_PNG}

${MAZE_PNG}: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --output ${MAZE_PNG}

${MAZE_SOLVED_PNG}: ${TARGET_RELEASE} ${MAZE_PNG}
	@echo "Solved version already generated with maze"

# Algorithm-specific targets (always generate solved versions)
maze-recursive-backtracking: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --algorithm recursive_backtracking --output ${IMG_DIR}/maze_recursive_backtracking.png

maze-kruskal: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --algorithm kruskal --output ${IMG_DIR}/maze_kruskal.png

maze-prim: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --algorithm prim --output ${IMG_DIR}/maze_prim.png

maze-aldous-broder: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --algorithm aldous_broder --output ${IMG_DIR}/maze_aldous_broder.png

# Generate all algorithms
all-algorithms: ${TARGET_RELEASE} maze-recursive-backtracking maze-kruskal maze-prim maze-aldous-broder

# Generate all maze variations
all-mazes: ${TARGET_RELEASE} maze maze-recursive-backtracking maze-kruskal maze-prim maze-aldous-broder \
	maze-small maze-medium maze-large maze-huge \
	maze-simple maze-normal maze-complex

# Generate README example images in .img directory
images-readme: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --width 25 --height 25 --output ${IMG_DIR}/README_maze.png

# Size examples (DOE - Design of Experiments)
size-example-50: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --config config_size_50.toml

size-example-100: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --config config_size_100.toml

size-example-200: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --config config_size_200.toml

size-example-500: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --config config_size_500.toml

size-example-1000: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --config config_size_1000.toml

# Generate all size examples
size-examples: ${TARGET_RELEASE} size-example-50 size-example-100 size-example-200 size-example-500 size-example-1000

# Complexity examples (DOE - Design of Experiments)
# Generates mazes for all algorithms at all complexity levels (0.0 to 1.0 in steps of 0.1)
# Uses config_complexity_examples.toml for cell_size=2 (100x100 pixel images)
complexity-examples: ${TARGET_RELEASE}
	@mkdir -p complexity_examples
	@echo "Generating complexity examples..."
	@for algo in recursive_backtracking kruskal prim aldous_broder; do \
		for comp in 0.0 0.1 0.2 0.3 0.4 0.5 0.6 0.7 0.8 0.9 1.0; do \
			echo "  Generating $${algo} complexity $${comp}..."; \
			timeout 60 ${TARGET_RELEASE} \
				--config config_complexity_examples.toml \
				--algorithm $${algo} \
				--complexity $${comp} \
				--seed 12345 \
				--output complexity_examples/$${algo}_complexity_$${comp}.png; \
		done; \
	done
	@echo "Complexity examples generated"

# Algorithm examples (DOE - Design of Experiments)
# Generates mazes for all algorithms at complexity levels 0.0, 0.5, 1.0 with fixed seed and size
# Used for ALGORITHM_EXAMPLES.md documentation
algorithm-examples: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	@echo "Generating algorithm examples for ALGORITHM_EXAMPLES.md..."
	@for algo in recursive_backtracking kruskal prim aldous_broder wilsons recursive_division growing_tree hunt_and_kill binary_tree sidewinder eller dfs_iterative bfs recursive_backtracking_braided cellular_automata drunkards_walk random_obstacle hamiltonian voronoi; do \
		for comp in 0.0 0.5 1.0; do \
			echo "  Generating $${algo} complexity $${comp}..."; \
			timeout 60 ${TARGET_RELEASE} \
				--config config_doe.toml \
				--algorithm $${algo} \
				--complexity $${comp} \
				--width 50 \
				--height 50 \
				--seed 12345 \
				--output ${IMG_DIR}/$${algo}_$${comp}.png; \
		done; \
	done
	@echo "Algorithm examples generated"

# Regenerate all images (all mazes + README images + size examples + complexity examples + algorithm examples)
regenerate-images: ${TARGET_RELEASE} all-mazes images-readme size-examples complexity-examples algorithm-examples
	@echo "All images regenerated"

# Regenerate all images, add to git, and push
regenerate-images-push: regenerate-images
	git add ${IMG_DIR}/*.png complexity_examples/*.png
	git commit -m "Regenerate all maze images" || true
	git push

# Size presets (always generate solved versions)
maze-small: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --width 20 --height 20 --output ${IMG_DIR}/maze_small.png

maze-medium: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --width 50 --height 50 --output ${IMG_DIR}/maze_medium.png

maze-large: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --width 100 --height 100 --output ${IMG_DIR}/maze_large.png

maze-huge: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --width 200 --height 200 --output ${IMG_DIR}/maze_huge.png

# Complexity variations
maze-simple: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --complexity 0.1 --output ${IMG_DIR}/maze_simple.png

maze-normal: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --complexity 0.5 --output ${IMG_DIR}/maze_normal.png

maze-complex: ${TARGET_RELEASE}
	@mkdir -p ${IMG_DIR}
	timeout 60 ${TARGET_RELEASE} --complexity 0.9 --output ${IMG_DIR}/maze_complex.png

# Clean targets
clean:
	${CARGO} clean
	rm -f ${TARGET_RELEASE} ${TARGET_DEBUG}

clean-outputs:
	rm -rf ${IMG_DIR}

clean-all: clean clean-outputs

# Serve docs locally for GitHub Pages preview
serve-docs:
	@echo "Serving docs/ directory on http://localhost:8000"
	@echo "Press Ctrl+C to stop the server"
	@cd docs && python3 -m http.server 8000

# Help target
help:
	@echo "Maze Generator Makefile Targets:"
	@echo ""
	@echo "Build targets:"
	@echo "  release          - Build release binary"
	@echo "  debug            - Build debug binary"
	@echo "  benchmark        - Run full benchmark suite"
	@echo "  benchmark-quick  - Run quick benchmark suite"
	@echo ""
	@echo "Documentation:"
	@echo "  serve-docs       - Serve docs/ directory locally on http://localhost:8000"
	@echo ""
	@echo "Maze generation:"
	@echo "  maze             - Generate default maze with solution (.img/maze.png + .img/maze_solved.png)"
	@echo ""
	@echo "Algorithm-specific:"
	@echo "  maze-recursive-backtracking - Generate using recursive backtracking"
	@echo "  maze-kruskal                - Generate using Kruskal's algorithm"
	@echo "  maze-prim                   - Generate using Prim's algorithm"
	@echo "  maze-aldous-broder          - Generate using Aldous-Broder algorithm"
	@echo "  all-algorithms              - Generate mazes with all algorithms"
	@echo ""
	@echo "Size presets (all include solved versions):"
	@echo "  maze-small       - 20x20 maze"
	@echo "  maze-medium      - 50x50 maze"
	@echo "  maze-large       - 100x100 maze"
	@echo "  maze-huge        - 200x200 maze"
	@echo ""
	@echo "Complexity variations:"
	@echo "  maze-simple      - Complexity 0.1"
	@echo "  maze-normal      - Complexity 0.5"
	@echo "  maze-complex     - Complexity 0.9"
	@echo ""
	@echo "Size examples (DOE - fixed 50x50 maze, varying resolution):"
	@echo "  size-example-50   - 50x50 pixel image (cell_size=1)"
	@echo "  size-example-100  - 100x100 pixel image (cell_size=2)"
	@echo "  size-example-200  - 200x200 pixel image (cell_size=4)"
	@echo "  size-example-500  - 500x500 pixel image (cell_size=10)"
	@echo "  size-example-1000 - 1000x1000 pixel image (cell_size=20)"
	@echo "  size-examples     - Generate all size examples"
	@echo ""
	@echo "Complexity examples (DOE - fixed seed/size, varying complexity 0.0-1.0):"
	@echo "  complexity-examples - Generate all complexity examples for all algorithms"
	@echo ""
	@echo "Algorithm examples (DOE - fixed seed/size/complexity, varying algorithm):"
	@echo "  algorithm-examples - Generate algorithm examples for ALGORITHM_EXAMPLES.md"
	@echo ""
	@echo "Image regeneration:"
	@echo "  images-readme          - Generate README example images in .img directory"
	@echo "  regenerate-images       - Regenerate all maze images (includes size, complexity, and algorithm examples)"
	@echo "  regenerate-images-push  - Regenerate images, add to git, and push"
	@echo ""
	@echo "Clean targets:"
	@echo "  clean            - Remove build artifacts"
	@echo "  clean-outputs    - Remove all generated PNG files"
	@echo "  clean-all        - Remove both build artifacts and outputs"

