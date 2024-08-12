#!/bin/bash

# Set up pre-commit hook
cat << 'EOF' > .git/hooks/pre-commit
#!/bin/sh

# Run cargo fmt
cargo fmt --check
if [ $? -ne 0 ]; then
  echo "Cargo fmt check failed."
  exit 1
fi

# Run cargo clippy
cargo clippy --no-deps --all-features --release -- -Dwarnings
if [ $? -ne 0 ]; then
  echo "Clippy check failed."
  exit 1
fi
EOF

chmod +x .git/hooks/pre-commit

# Set up pre-push hook
cat << 'EOF' > .git/hooks/pre-push
#!/bin/sh

# Run release build of the binary
cargo leptos build --release --lib-features release-lib --bin-features release-bin
EOF

chmod +x .git/hooks/pre-push

