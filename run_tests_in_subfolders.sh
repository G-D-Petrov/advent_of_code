#!/bin/bash

set -e

# Find all Cargo.toml files in subfolders
for cargo_toml in $(find . -mindepth 2 -name 'Cargo.toml'); do
  # Get the directory of the Cargo.toml file
  project_dir=$(dirname "$cargo_toml")

  # Change into the project directory
  pushd "$project_dir" > /dev/null

  # Build the project
  echo "Building $project_dir"
  cargo build --verbose

  # Run the tests
  echo "Running tests in $project_dir"
  cargo test --verbose

  # Change back to the root directory
  popd > /dev/null
done
