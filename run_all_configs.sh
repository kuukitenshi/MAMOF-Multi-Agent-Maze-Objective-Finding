#!/usr/bin/env bash

EXEC=./target/debug/mamof

OUTPUT_DIR=./results/samples
rm -rf "$OUTPUT_DIR/*"
mkdir -p "$OUTPUT_DIR"

# 20 seeds
SEEDS=(3 1234 1337 4242 5678 9001 9876 13579 24680 31415 78910 \
        101112 111213 112358 123456 161803 202122 271828 445566 778899)

declare -a configs=(
    "2 8"
    "5 8"
    "32 64"
)

for cfg in "${configs[@]}"; do
   read -r AGENTS MAP <<< "$cfg"

   DIR="a${AGENTS}_m${MAP}"
   mkdir -p "$OUTPUT_DIR/$DIR"

   for SEED in "${SEEDS[@]}"; do
        #config 1 - nothing
        $EXEC --headless -n "$AGENTS" -m "$MAP" --simulation-speed x128 -s "$SEED" \
        --disable-position-sharing \
        --disable-goal-sharing \
        --disable-map-sharing \
        --disable-agent-guiding \
        -o "$OUTPUT_DIR/$DIR/config1_seed${SEED}.json"

        #config 2 - position
        $EXEC --headless -n "$AGENTS" -m "$MAP" --simulation-speed x128 -s "$SEED" \
        --disable-goal-sharing \
        --disable-map-sharing \
        --disable-agent-guiding \
        -o "$OUTPUT_DIR/$DIR/config2_seed${SEED}.json"

        #config 3 - position + goal
        $EXEC --headless -n "$AGENTS" -m "$MAP" --simulation-speed x128 -s "$SEED" \
        --disable-map-sharing \
        --disable-agent-guiding \
        -o "$OUTPUT_DIR/$DIR/config3_seed${SEED}.json"

        #config 4 - position + goal + map
        $EXEC --headless -n "$AGENTS" -m "$MAP" --simulation-speed x128 -s "$SEED" \
        --disable-agent-guiding \
        -o "$OUTPUT_DIR/$DIR/config4_seed${SEED}.json"

        #config 5 - all
        $EXEC --headless -n "$AGENTS" -m "$MAP" --simulation-speed x128 -s "$SEED" \
        -o "$OUTPUT_DIR/$DIR/config5_seed${SEED}.json"
   done
done