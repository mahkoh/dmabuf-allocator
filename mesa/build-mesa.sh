#!/bin/bash
set -euo pipefail

cd "$(dirname "$0")"

options=(
    -Dbuildtype=release
    -Db_ndebug=true
    -Dplatforms=
    -Dglx=disabled
    -Degl=auto
    -Dgbm=enabled
    -Dgles1=disabled
    -Dgles2=disabled
    -Dopengl=true
    -Dgallium-drivers=radeonsi,iris,zink,llvmpipe,asahi,panfrost,v3d,virgl,etnaviv,lima,vc4
    -Dgallium-va=disabled
    -Dvulkan-drivers=intel,amd,nouveau
    -Dvideo-codecs=
    -Dtools=
    -Dbuild-tests=false
    -Dintel-rt=disabled
    -Dpanfrost-rust=false
    -Damdgpu-virtio=true
    -Dintel-virtio-experimental=true
    -Dpanfrost-virtio=true
)

export CC=clang CXX=clang++

build=builddir
stamp=$build/testgen-options
configuration="CC=$CC CXX=$CXX ${options[*]}"
if [ ! -f "$build/build.ninja" ]; then
    meson setup "$build" mesa "${options[@]}"
    printf '%s' "$configuration" > "$stamp"
elif [ "$(cat "$stamp" 2>/dev/null)" != "$configuration" ]; then
    meson setup --wipe "$build" mesa "${options[@]}"
    printf '%s' "$configuration" > "$stamp"
fi

targets=("$@")
if [ $# -eq 0 ]; then
    arch=$(uname -m)
    while IFS== read -r _ target; do
        targets+=("${target/\$ARCH/$arch}")
    done < testgen-targets
fi

ninja -C "$build" "${targets[@]}"
