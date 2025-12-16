PROJECT_NAME := maze_generator
CARGO := cargo
TARGET_RELEASE := target/release/${PROJECT_NAME}
TARGET_DEBUG := target/debug/${PROJECT_NAME}

# Output files
MAZE_PNG := maze.png
MAZE_SOLVED_PNG := maze_solved.png

.PHONY: release debug clean maze all-algorithms all-mazes benchmark help

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
	${CARGO} bench -- --sample-size 10

# Default maze generation (uses config.toml) - always generates solved version
maze: ${TARGET_RELEASE} ${MAZE_PNG} ${MAZE_SOLVED_PNG}

${MAZE_PNG}: ${TARGET_RELEASE}
	${TARGET_RELEASE}

${MAZE_SOLVED_PNG}: ${TARGET_RELEASE} ${MAZE_PNG}
	@echo "Solved version already generated with maze"

# Algorithm-specific targets (always generate solved versions)
maze-recursive-backtracking: ${TARGET_RELEASE}
	${TARGET_RELEASE} --algorithm recursive_backtracking --output maze_recursive_backtracking.png

maze-kruskal: ${TARGET_RELEASE}
	${TARGET_RELEASE} --algorithm kruskal --output maze_kruskal.png

maze-prim: ${TARGET_RELEASE}
	${TARGET_RELEASE} --algorithm prim --output maze_prim.png

maze-aldous-broder: ${TARGET_RELEASE}
	${TARGET_RELEASE} --algorithm aldous_broder --output maze_aldous_broder.png

# Generate all algorithms
all-algorithms: ${TARGET_RELEASE} maze-recursive-backtracking maze-kruskal maze-prim maze-aldous-broder

# Generate all maze variations
all-mazes: ${TARGET_RELEASE} maze maze-recursive-backtracking maze-kruskal maze-prim maze-aldous-broder \
	maze-small maze-medium maze-large maze-huge \
	maze-simple maze-normal maze-complex

# Size presets (always generate solved versions)
maze-small: ${TARGET_RELEASE}
	${TARGET_RELEASE} --width 20 --height 20 --output maze_small.png

maze-medium: ${TARGET_RELEASE}
	${TARGET_RELEASE} --width 50 --height 50 --output maze_medium.png

maze-large: ${TARGET_RELEASE}
	${TARGET_RELEASE} --width 100 --height 100 --output maze_large.png

maze-huge: ${TARGET_RELEASE}
	${TARGET_RELEASE} --width 200 --height 200 --output maze_huge.png

# Complexity variations
maze-simple: ${TARGET_RELEASE}
	${TARGET_RELEASE} --complexity 0.1 --output maze_simple.png

maze-normal: ${TARGET_RELEASE}
	${TARGET_RELEASE} --complexity 0.5 --output maze_normal.png

maze-complex: ${TARGET_RELEASE}
	${TARGET_RELEASE} --complexity 0.9 --output maze_complex.png

# Clean targets
clean:
	${CARGO} clean
	rm -f ${TARGET_RELEASE} ${TARGET_DEBUG}

clean-outputs:
	rm -f ${MAZE_PNG} ${MAZE_SOLVED_PNG} \
		maze_*.png \
		maze_small.png maze_medium.png maze_large.png maze_huge.png \
		maze_simple.png maze_normal.png maze_complex.png

clean-all: clean clean-outputs

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
	@echo "Maze generation:"
	@echo "  maze             - Generate default maze with solution (maze.png + maze_solved.png)"
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
	@echo "Clean targets:"
	@echo "  clean            - Remove build artifacts"
	@echo "  clean-outputs    - Remove all generated PNG files"
	@echo "  clean-all        - Remove both build artifacts and outputs"

