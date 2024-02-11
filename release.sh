#!/bin/zsh
# Requires 
# - cargo build (install via cargo)
# - git-cliff (install via cargo)
# - jq (install via package manager) 

echo "Bumping Version"
cargo bump $1
if [[ $? != 0 ]]
then
  echo "Version could not be bumped"
  exit 1
fi

cargo build 
if [[ $? != 0 ]]
then
  echo "Could not build new version"
  exit 1
fi
VERSION=$(cargo read-manifest |  jq -r '.version')

git add Cargo.toml Cargo.lock 
git commit -n -m "build: Bumped version to ${VERSION} :bookmark:"

echo "Updating changelog"
git cliff -o CHANGELOG.md
if [[ $? != 0 ]]
then
  echo "Generating changelog failed. Exiting..."
  exit 1
fi
git add CHANGELOG.md
git commit -n -m "chore: Updated CHANGELOG.md :memo:"

git tag ${VERSION}

echo "Pushing"
git push

echo "Pushing tag"
git push --tag