# Algorithm Examples - Design of Experiments

This document demonstrates the four maze generation algorithms using a Design of Experiments (DOE) approach. All parameters except the algorithm are held constant to clearly show how each algorithm affects the maze structure and solution path.

## Experimental Design

### Fixed Parameters
- **Seed**: 12345 (for reproducibility)
- **Maze Size**: 50×50 cells (medium)
- **Image Size**: 100×100 pixels (cell_size = 2)
- **Complexity Values**: 0.0, 0.5, 1.0

### Variable Parameter
- **Algorithm**: Four different maze generation algorithms

### Experimental Matrix

| Algorithm | Complexity 0.0 | Complexity 0.5 | Complexity 1.0 |
|-----------|----------------|----------------|----------------|
| Recursive Backtracking | ✓ | ✓ | ✓ |
| Kruskal | ✓ | ✓ | ✓ |
| Prim | ✓ | ✓ | ✓ |
| Aldous-Broder | ✓ | ✓ | ✓ |

**Total Combinations**: 4 algorithms × 3 complexity values = 12 mazes

## Results by Complexity Level

### Complexity 0.0

At minimum complexity, the algorithms show their baseline characteristics:

#### Recursive Backtracking
![Recursive Backtracking - Complexity 0.0](.img/recursive_backtracking_0.0_solved.png)

#### Kruskal
![Kruskal - Complexity 0.0](.img/kruskal_0.0_solved.png)

#### Prim
![Prim - Complexity 0.0](.img/prim_0.0_solved.png)

#### Aldous-Broder
![Aldous-Broder - Complexity 0.0](.img/aldous_broder_0.0_solved.png)

---

### Complexity 0.5

At medium complexity, the algorithms demonstrate balanced maze characteristics:

#### Recursive Backtracking
![Recursive Backtracking - Complexity 0.5](.img/recursive_backtracking_0.5_solved.png)

#### Kruskal
![Kruskal - Complexity 0.5](.img/kruskal_0.5_solved.png)

#### Prim
![Prim - Complexity 0.5](.img/prim_0.5_solved.png)

#### Aldous-Broder
![Aldous-Broder - Complexity 0.5](.img/aldous_broder_0.5_solved.png)

---

### Complexity 1.0

At maximum complexity, the algorithms show their most complex maze structures:

#### Recursive Backtracking
![Recursive Backtracking - Complexity 1.0](.img/recursive_backtracking_1.0_solved.png)

#### Kruskal
![Kruskal - Complexity 1.0](.img/kruskal_1.0_solved.png)

#### Prim
![Prim - Complexity 1.0](.img/prim_1.0_solved.png)

#### Aldous-Broder
![Aldous-Broder - Complexity 1.0](.img/aldous_broder_1.0_solved.png)

---

## Algorithm Characteristics

### Recursive Backtracking
- **Pattern**: Long, winding paths with deep branches
- **Solution Path**: Typically longer, more circuitous routes
- **Complexity Effect**: More pronounced branching at higher complexity

### Kruskal
- **Pattern**: More uniform distribution of paths
- **Solution Path**: Generally shorter, more direct routes
- **Complexity Effect**: Maintains uniform structure across complexity levels

### Prim
- **Pattern**: Tree-like structure growing from a single point
- **Solution Path**: Moderate length, follows tree branches
- **Complexity Effect**: Tree structure becomes more complex with higher complexity

### Aldous-Broder
- **Pattern**: Random walk creates varied path distributions
- **Solution Path**: Variable length depending on random walk pattern
- **Complexity Effect**: Random nature makes complexity effects less predictable

## Observations

1. **Solution Path Length**: Different algorithms produce mazes with varying solution path lengths even with the same seed and complexity, demonstrating their inherent structural differences.

2. **Path Patterns**: Each algorithm creates distinct visual patterns:
   - Recursive Backtracking: Long corridors with dead ends
   - Kruskal: More evenly distributed paths
   - Prim: Centralized tree structure
   - Aldous-Broder: More random, less predictable patterns

3. **Complexity Impact**: The complexity parameter affects each algorithm differently, with some showing more dramatic changes than others across the 0.0 to 1.0 range.

4. **Reproducibility**: Using the same seed (12345) ensures that any differences observed are due to the algorithm itself, not random variation.

## Generating These Examples

To regenerate these examples with the same parameters:

```bash
# Ensure the binary is built
make release

# Generate all combinations
for algo in recursive_backtracking kruskal prim aldous_broder; do
  for complexity in 0.0 0.5 1.0; do
    ./target/release/maze_generator \
      --config config_doe.toml \
      --algorithm "$algo" \
      --complexity "$complexity" \
      --width 50 \
      --height 50 \
      --seed 12345 \
      --output ".img/${algo}_${complexity}.png"
  done
done
```

Note: The `config_doe.toml` file contains `cell_size = 2` to produce 100×100 pixel images for 50×50 cell mazes.

