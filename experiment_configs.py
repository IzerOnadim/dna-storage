from constraints import default_constraints, wider_gc_limits
from experiments import Parameters


# EXP 1 - DEFAULT (5 RESERVED BITS, 35% < GC < 85%)
config1 = Parameters(sequence_length=300, repetitions=20, random_seed=42)

# EXP 2 - 3 RESERVED BITS, 15% < GC < 85%
config2 = Parameters(
    reserved_bits=3,
    constraints=wider_gc_limits(),
    sequence_length=300,
    repetitions=20,
    random_seed=42,
)

# EXP 3 - 4 RESERVED BITS, 25% < GC < 75%
config3 = Parameters(
    reserved_bits=4,
    constraints=default_constraints(gc_min=0.25, gc_max=0.75),
    sequence_length=300,
    repetitions=20,
    random_seed=42,
)

# EXP 4 - SYMBOL LENGTH 5, 5 RESERVED BITS, 30% < GC 70%
config4 = Parameters(
    symbol_size=5,
    constraints=default_constraints(
        symbol_size=5,
        gc_min=0.3,
        gc_max=0.7,
        restriction_sites=[],
    ),
    sequence_length=200,
    repetitions=20,
    random_seed=42,
)