#!/bin/bash

# Fast build script supporting codex-rs, zeus-rs, and code-rs
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

show_usage() {
  cat << EOF
Usage: ./build-fast.sh [OPTIONS]

Options:
  DETERMINISTIC=1                     Add -C debuginfo=0; promotes to release-prod unless DETERMINISTIC_FORCE_RELEASE=0
  DETERMINISTIC_FORCE_RELEASE=0|1     Keep dev-fast (0) or switch to release-prod (1, default)
  DETERMINISTIC_NO_UUID=1             macOS only: strip LC_UUID on final executables
  BUILD_FAST_BINS="zeus zeus-tui"      Override bins to build (space or comma separated)
  --workspace codex|zeus|code|both    Select workspace to build (default: zeus)

Examples:
  ./build-fast.sh
  ./build-fast.sh --workspace both
  ./build-fast.sh --workspace code DETERMINISTIC=1
  BUILD_FAST_BINS="my-bin other-bin" ./build-fast.sh
  ./build-fast.sh --no-cargo-check  (skip cargo check for faster builds)
EOF
}

for arg in "$@"; do
  if [ "$arg" = "--help" ] || [ "$arg" = "-h" ]; then
    show_usage
    exit 0
  fi
  if [[ "$arg" == "--workspace" ]]; then
    WORKSPACE_CHOICE_NEXT=true
    continue
  fi
  if [ "$WORKSPACE_CHOICE_NEXT" = true ]; then
    WORKSPACE_CHOICE="$arg"
    WORKSPACE_CHOICE_NEXT=false
    continue
  fi
  if [[ "$arg" == "--no-cargo-check" ]]; then
    NO_CARGO_CHECK=true
    continue
  fi
  if [[ "$arg" == *"="* ]]; then
    export "$arg"
    continue
  fi
done

if [ -z "$WORKSPACE_CHOICE" ]; then
  WORKSPACE_CHOICE="zeus"
fi

if [ "$WORKSPACE_CHOICE" = "both" ]; then
  for workspace in codex zeus code; do
    if [ -d "$SCRIPT_DIR/${workspace}-rs" ]; then
      "$0" --workspace "$workspace"
    fi
  done
  exit $?
fi

case "$WORKSPACE_CHOICE" in
  codex|codex-rs)
    WORKSPACE_DIR="codex-rs"
    DEFAULT_CRATE_PREFIX="codex"
    ;;
  zeus|zeus-rs)
    WORKSPACE_DIR="zeus-rs"
    DEFAULT_CRATE_PREFIX="zeus"
    ;;
  code|code-rs)
    WORKSPACE_DIR="code-rs"
    DEFAULT_CRATE_PREFIX="code"
    ;;
  *)
    echo "Error: Unknown workspace '${WORKSPACE_CHOICE}'. Use codex, zeus, code, or both." >&2
    exit 1
    ;;
esac

CRATE_PREFIX="${DEFAULT_CRATE_PREFIX:-}"

WORKSPACE_PATH="${SCRIPT_DIR}/${WORKSPACE_DIR}"
if [ ! -d "$WORKSPACE_PATH" ]; then
  echo "Error: Workspace directory '${WORKSPACE_PATH}' not found." >&2
  exit 1
fi

cd "$WORKSPACE_PATH"

PROFILE="${PROFILE:-dev-fast}"

if [ "${DETERMINISTIC:-}" = "1" ]; then
  if [ "${DETERMINISTIC_FORCE_RELEASE:-1}" != "0" ]; then
    PROFILE="release-prod"
  fi
fi

if [ "$NO_CARGO_CHECK" != "true" ]; then
  echo "Running cargo check..."
  cargo check --profile "$PROFILE" "$@" 2>&1 | tail -20
fi

case "$(uname)" in
  Darwin)
    RUSTFLAGS="${RUSTFLAGS} -C target-cpu=native"
    if [ "${DETERMINISTIC:-}" = "1" ]; then
      RUSTFLAGS="${RUSTFLAGS} -C debuginfo=0"
    fi
    if [ "${DETERMINISTIC_NO_UUID:-}" = "1" ]; then
      RUSTFLAGS="${RUSTFLAGS} -Z unstable-options -C strip=debuginfo"
    fi
    export RUSTFLAGS
    ;;
  Linux)
    RUSTFLAGS="${RUSTFLAGS} -C target-cpu=native"
    if [ "${DETERMINISTIC:-}" = "1" ]; then
      RUSTFLAGS="${RUSTFLAGS} -C debuginfo=0"
    fi
    export RUSTFLAGS
    ;;
esac

BUILD_CMD="cargo build --profile $PROFILE"
if [ -n "${BUILD_FAST_BINS:-}" ]; then
  for bin in ${BUILD_FAST_BINS//,/ }; do
    BUILD_CMD="$BUILD_CMD --bin $bin"
  done
fi

if [ "${VERBOSE:-}" = "1" ]; then
  set -x
  "$BUILD_CMD"
  set +x
elif [ "${VERY_VERBOSE:-}" = "1" ]; then
  "$BUILD_CMD"
else
  "$BUILD_CMD" 2>&1 | tail -20
fi

BUILD_PATH="target/$PROFILE"

case "$(uname)" in
  Darwin)
    BINARIES=(
      "$BUILD_PATH/code"
      "$BUILD_PATH/code-tui"
      "$BUILD_PATH/zeus"
      "$BUILD_PATH/zeus-tui"
      "$BUILD_PATH/codex"
      "$BUILD_PATH/codex-tui"
    )
    for bin in "${BINARIES[@]}"; do
      if [ -f "$bin" ]; then
        ls -lh "$bin"
        if [ "${DETERMINISTIC_NO_UUID:-}" = "1" ]; then
          echo "Stripping LC_UUID from $bin..."
          xcrun vtool -remove LC_UUID "$bin" -o "$bin" 2>/dev/null || true
        fi
      fi
    done
    ;;
  Linux)
    BINARIES=(
      "$BUILD_PATH/code"
      "$BUILD_PATH/code-tui"
      "$BUILD_PATH/zeus"
      "$BUILD_PATH/zeus-tui"
      "$BUILD_PATH/codex"
      "$BUILD_PATH/codex-tui"
    )
    for bin in "${BINARIES[@]}"; do
      if [ -f "$bin" ]; then
        ls -lh "$bin"
        file "$bin" 2>/dev/null || true
      fi
    done
    ;;
esac

case "$WORKSPACE_CHOICE" in
  codex|codex-rs)
    if [ -f "$BUILD_PATH/codex" ] || [ -f "$BUILD_PATH/codex-tui" ]; then
      echo "✅ Codex build complete"
    else
      echo "⚠️  Warning: Expected binaries not found in $BUILD_PATH"
    fi
    ;;
  zeus|zeus-rs)
    if [ -f "$BUILD_PATH/zeus" ] || [ -f "$BUILD_PATH/zeus-tui" ]; then
      echo "✅ Zeus build complete"
    else
      echo "⚠️  Warning: Expected binaries not found in $BUILD_PATH"
    fi
    ;;
  code|code-rs)
    if [ -f "$BUILD_PATH/code" ] || [ -f "$BUILD_PATH/code-tui" ]; then
      echo "✅ Code build complete"
    else
      echo "⚠️  Warning: Expected binaries not found in $BUILD_PATH"
    fi
    ;;
esac

if [ "${VERBOSE_WORKSPACE_PATH:-}" = "1" ]; then
  echo "Workspace path: $WORKSPACE_PATH"
  echo "Build path: $BUILD_PATH"
fi
