# HolidaysRu.jl

Julia bindings for the [`holidays-ru`](https://crates.io/crates/holidays-ru)
Rust library. The native module uses `jlrs`; Julia-specific dependencies are
isolated from the main Cargo workspace.

Julia 1.10–1.13 are tested in CI. Building the native module requires Rust 1.97 or newer; the Julia package uses JlrsCore 0.8.

The package currently targets source-based development. Build the native
library before loading `HolidaysRu`; prebuilt JLL artifacts are not published
yet.

```julia
using Dates
using HolidaysRu

calendar = Calendar(Tatarstan)
info = day_info(calendar, Date(2026, 11, 6))

@assert is_day_off(info)
@assert is_official(info)
```

Date ranges are half-open: the start is included and the stop is excluded.

## Development

The native library is linked against the selected minor version of Julia. If
Julia was installed with `juliaup`, set `JLRS_JULIA_DIR` to its installation
root while building:

```console
cd bindings/julia/native
JLRS_JULIA_DIR="$(julia -e 'print(dirname(Sys.BINDIR))')" cargo build

cd ..
julia --project=. -e 'using Pkg; Pkg.instantiate(); Pkg.test()'
```

If an existing local `Manifest.toml` still pins JlrsCore 0.7, run `julia --project=. -e 'using Pkg; Pkg.resolve()'` from `bindings/julia` before instantiating the updated project.

`HOLIDAYS_RU_JULIA_LIB` can be set to an explicit native-library path. By
default the package loads the debug library from `native/target/debug`.

Distribution as a registered Julia package will use per-Julia-version JLL
artifacts; building those artifacts is intentionally separate from local
development.

The native module has its own `native/Cargo.lock`. After changing versions or dependencies of the root crate, the shared bindings crate, or the native module, refresh this lockfile with `cargo update --manifest-path bindings/julia/native/Cargo.toml` from the repository root and commit it along with the root `Cargo.lock`. CI builds the native module with `--locked`.
