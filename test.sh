#!/bin/bash

# Clear previous tasks.json for a clean start
rm -f tasks.json

echo "=== Test: Add Tasks ==="
cargo run -- add "First Task"
cargo run -- add "Second Task"
cargo run -- add "Third Task"

echo -e "\n=== Test: List All Tasks ==="
cargo run -- list

echo -e "\n=== Test: Update Description of Task ID 2 ==="
cargo run -- update 2 "Updated Second Task Description"
cargo run -- list

echo -e "\n=== Test: Mark Task ID 1 as In Progress ==="
cargo run -- mark-in-progress 1
cargo run -- list inprogress

echo -e "\n=== Test: Mark Task ID 2 as Completed ==="
cargo run -- mark-completed 2
cargo run -- list completed

echo -e "\n=== Test: List Only Pending Tasks ==="
cargo run -- list pending

echo -e "\n=== Test: Delete Task ID 3 ==="
cargo run -- delete 3
cargo run -- list

echo -e "\n=== Final State of tasks.json ==="
cat tasks.json
