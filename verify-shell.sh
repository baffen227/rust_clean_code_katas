#!/usr/bin/env bash

echo "🔍 Nix Shell Environment Healthcheck"
echo "====================================="
echo

PASS=0
FAIL=0

echo "=== Shell Environment ==="
echo -n "  Testing shell entry... "
if timeout 10 nix-shell --run "echo 'OK'" >/dev/null 2>&1; then
    echo "✓ PASS"
    PASS=$((PASS + 1))
else
    echo "✗ FAIL"
    FAIL=$((FAIL + 1))
fi
echo

echo "=== Rust Toolchain ==="
echo -n "  Testing Rust version... "
RUST_VERSION=$(timeout 10 nix-shell --run "rustc --version" 2>/dev/null || echo "")
if echo "$RUST_VERSION" | grep -q "nightly"; then
    echo "✓ PASS"
    echo "    Info: $RUST_VERSION"
    PASS=$((PASS + 1))
else
    echo "✗ FAIL"
    FAIL=$((FAIL + 1))
fi

echo -n "  Testing cargo... "
if timeout 10 nix-shell --run "cargo --version" >/dev/null 2>&1; then
    echo "✓ PASS"
    PASS=$((PASS + 1))
else
    echo "✗ FAIL"
    FAIL=$((FAIL + 1))
fi

echo -n "  Testing clippy... "
if timeout 10 nix-shell --run "clippy-driver --version" >/dev/null 2>&1; then
    echo "✓ PASS"
    PASS=$((PASS + 1))
else
    echo "✗ FAIL"
    FAIL=$((FAIL + 1))
fi

echo -n "  Testing rustfmt... "
if timeout 10 nix-shell --run "rustfmt --version" >/dev/null 2>&1; then
    echo "✓ PASS"
    PASS=$((PASS + 1))
else
    echo "✗ FAIL"
    FAIL=$((FAIL + 1))
fi
echo

echo "=== Required Tools ==="
echo -n "  Testing cargo-make... "
if timeout 10 nix-shell --run "cargo make --version" >/dev/null 2>&1; then
    MAKE_VERSION=$(timeout 10 nix-shell --run "cargo make --version" 2>/dev/null)
    echo "✓ PASS"
    echo "    Info: $MAKE_VERSION"
    PASS=$((PASS + 1))
else
    echo "✗ FAIL"
    FAIL=$((FAIL + 1))
fi
echo

echo "=== Unstable Utilities ==="
echo -n "  Testing claude-code... "
if timeout 10 nix-shell --run "claude --version" >/dev/null 2>&1; then
    echo "✓ PASS"
    PASS=$((PASS + 1))
else
    echo "✗ FAIL"
    FAIL=$((FAIL + 1))
fi

echo -n "  Testing gemini-cli... "
if timeout 10 nix-shell --run "gemini --version" >/dev/null 2>&1; then
    echo "✓ PASS"
    PASS=$((PASS + 1))
else
    echo "✗ FAIL"
    FAIL=$((FAIL + 1))
fi

echo -n "  Testing kiro... "
if timeout 10 nix-shell --run "kiro --version" >/dev/null 2>&1; then
    echo "✓ PASS"
    PASS=$((PASS + 1))
else
    echo "✗ FAIL"
    FAIL=$((FAIL + 1))
fi

echo -n "  Testing zed-editor... "
if timeout 10 nix-shell --run "zeditor --version" >/dev/null 2>&1; then
    echo "✓ PASS"
    PASS=$((PASS + 1))
else
    echo "✗ FAIL"
    FAIL=$((FAIL + 1))
fi
echo

echo "=== Development Workflow ==="
echo -n "  Testing cargo check... "
if timeout 30 nix-shell --run "cargo check" >/dev/null 2>&1; then
    echo "✓ PASS"
    echo "    Info: Project compiles successfully"
    PASS=$((PASS + 1))
else
    echo "✗ FAIL"
    FAIL=$((FAIL + 1))
fi
echo

echo "=== Summary ==="
echo "Tests passed: $PASS"
echo "Tests failed: $FAIL"

if [ $FAIL -eq 0 ]; then
    echo
    echo "🎉 All tests passed! Environment is ready for development."
    exit 0
else
    echo
    echo "⚠️  Some tests failed. Check the issues above."
    exit 1
fi