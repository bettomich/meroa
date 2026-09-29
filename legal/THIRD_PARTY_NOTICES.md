# MEROA 0.1.0 third-party notices

## Scope and verification method

This inventory is limited to components used by the Windows V1 artifact:

- frontend runtime packages resolved by `package-lock.json` (`npm ls --omit=dev`);
- the target-specific, non-dev Rust dependency closure resolved from `Cargo.lock` with `cargo tree --locked --offline --target x86_64-pc-windows-msvc --edges normal`;
- the two font assets emitted by the production frontend build.

License expressions below were read from the exact installed npm package metadata or the exact local crate `Cargo.toml` for the locked version. For the Rust entries, the project reference is the versioned crates.io source archive. No separate `NOTICE*` file was present in the locally resolved Windows-target crate directories. Preserve the applicable upstream license text and any copyright notice supplied by that source archive when redistributing.

`OR` license expressions are upstream dual-license choices; a distributor must comply with one selected option. `AND` expressions require compliance with each listed license.

## Frontend runtime

| Name | Version | License | Required copyright / notice | Source / project reference |
| --- | --- | --- | --- | --- |
| @tauri-apps/api | 2.11.1 | Apache-2.0 OR MIT | `Copyright (c) 2017 - Present Tauri Apps Contributors`; preserve selected license text | https://www.npmjs.com/package/@tauri-apps/api/v/2.11.1 |
| @tauri-apps/plugin-autostart | 2.5.1 | MIT OR Apache-2.0 | `2019-2022, The Tauri Programme in the Commons Conservancy`; preserve selected license text | https://www.npmjs.com/package/@tauri-apps/plugin-autostart/v/2.5.1 |
| react | 19.2.8 | MIT | `Copyright (c) Meta Platforms, Inc. and affiliates.`; preserve MIT text | https://www.npmjs.com/package/react/v/19.2.8 |
| react-dom | 19.2.8 | MIT | `Copyright (c) Meta Platforms, Inc. and affiliates.`; preserve MIT text | https://www.npmjs.com/package/react-dom/v/19.2.8 |
| scheduler | 0.27.0 | MIT | `Copyright (c) Meta Platforms, Inc. and affiliates.`; preserve MIT text | https://www.npmjs.com/package/scheduler/v/0.27.0 |

## Rust dependency closure for Windows x86_64

| Name | Version | License | Source / project reference |
| --- | --- | --- | --- |
| adler2 | 2.0.1 | 0BSD OR MIT OR Apache-2.0 | https://crates.io/crates/adler2/2.0.1 |
| aho-corasick | 1.1.5 | Unlicense OR MIT | https://crates.io/crates/aho-corasick/1.1.5 |
| alloc-no-stdlib | 2.0.4 | BSD-3-Clause | https://crates.io/crates/alloc-no-stdlib/2.0.4 |
| alloc-stdlib | 0.2.4 | BSD-3-Clause | https://crates.io/crates/alloc-stdlib/0.2.4 |
| anyhow | 1.0.104 | MIT OR Apache-2.0 | https://crates.io/crates/anyhow/1.0.104 |
| auto-launch | 0.5.0 | MIT | https://crates.io/crates/auto-launch/0.5.0 |
| base64 | 0.22.1 | MIT OR Apache-2.0 | https://crates.io/crates/base64/0.22.1 |
| bit-set | 0.8.0 | Apache-2.0 OR MIT | https://crates.io/crates/bit-set/0.8.0 |
| bit-vec | 0.8.0 | Apache-2.0 OR MIT | https://crates.io/crates/bit-vec/0.8.0 |
| bitflags | 1.3.2 | MIT/Apache-2.0 | https://crates.io/crates/bitflags/1.3.2 |
| bitflags | 2.13.1 | MIT OR Apache-2.0 | https://crates.io/crates/bitflags/2.13.1 |
| block-buffer | 0.10.4 | MIT OR Apache-2.0 | https://crates.io/crates/block-buffer/0.10.4 |
| brotli | 8.0.4 | BSD-3-Clause AND MIT | https://crates.io/crates/brotli/8.0.4 |
| brotli-decompressor | 5.0.3 | BSD-3-Clause/MIT | https://crates.io/crates/brotli-decompressor/5.0.3 |
| byteorder | 1.5.0 | Unlicense OR MIT | https://crates.io/crates/byteorder/1.5.0 |
| bytes | 1.12.1 | MIT | https://crates.io/crates/bytes/1.12.1 |
| camino | 1.2.5 | MIT OR Apache-2.0 | https://crates.io/crates/camino/1.2.5 |
| cargo_metadata | 0.19.2 | MIT | https://crates.io/crates/cargo_metadata/0.19.2 |
| cargo-platform | 0.1.9 | MIT OR Apache-2.0 | https://crates.io/crates/cargo-platform/0.1.9 |
| cfb | 0.7.3 | MIT | https://crates.io/crates/cfb/0.7.3 |
| cfg-if | 1.0.4 | MIT OR Apache-2.0 | https://crates.io/crates/cfg-if/1.0.4 |
| cookie | 0.18.2 | MIT OR Apache-2.0 | https://crates.io/crates/cookie/0.18.2 |
| cpufeatures | 0.2.17 | MIT OR Apache-2.0 | https://crates.io/crates/cpufeatures/0.2.17 |
| crc32fast | 1.5.1 | MIT OR Apache-2.0 | https://crates.io/crates/crc32fast/1.5.1 |
| crossbeam-channel | 0.5.16 | MIT OR Apache-2.0 | https://crates.io/crates/crossbeam-channel/0.5.16 |
| crossbeam-utils | 0.8.22 | MIT OR Apache-2.0 | https://crates.io/crates/crossbeam-utils/0.8.22 |
| crypto-common | 0.1.7 | MIT OR Apache-2.0 | https://crates.io/crates/crypto-common/0.1.7 |
| cssparser | 0.36.0 | MPL-2.0 | https://crates.io/crates/cssparser/0.36.0 |
| cssparser-macros | 0.6.1 | MPL-2.0 | https://crates.io/crates/cssparser-macros/0.6.1 |
| ctor | 0.8.0 | Apache-2.0 OR MIT | https://crates.io/crates/ctor/0.8.0 |
| ctor-proc-macro | 0.0.7 | Apache-2.0 OR MIT | https://crates.io/crates/ctor-proc-macro/0.0.7 |
| darling | 0.23.0 | MIT | https://crates.io/crates/darling/0.23.0 |
| darling_core | 0.23.0 | MIT | https://crates.io/crates/darling_core/0.23.0 |
| darling_macro | 0.23.0 | MIT | https://crates.io/crates/darling_macro/0.23.0 |
| deranged | 0.5.8 | MIT OR Apache-2.0 | https://crates.io/crates/deranged/0.5.8 |
| derive_more | 2.1.1 | MIT | https://crates.io/crates/derive_more/2.1.1 |
| derive_more-impl | 2.1.1 | MIT | https://crates.io/crates/derive_more-impl/2.1.1 |
| digest | 0.10.7 | MIT OR Apache-2.0 | https://crates.io/crates/digest/0.10.7 |
| dirs | 6.0.0 | MIT OR Apache-2.0 | https://crates.io/crates/dirs/6.0.0 |
| dirs-sys | 0.5.0 | MIT OR Apache-2.0 | https://crates.io/crates/dirs-sys/0.5.0 |
| displaydoc | 0.2.7 | MIT OR Apache-2.0 | https://crates.io/crates/displaydoc/0.2.7 |
| dom_query | 0.27.0 | MIT | https://crates.io/crates/dom_query/0.27.0 |
| dpi | 0.1.2 | Apache-2.0 AND MIT | https://crates.io/crates/dpi/0.1.2 |
| dtoa | 1.0.11 | MIT OR Apache-2.0 | https://crates.io/crates/dtoa/1.0.11 |
| dtoa-short | 0.3.5 | MPL-2.0 | https://crates.io/crates/dtoa-short/0.3.5 |
| dunce | 1.0.5 | CC0-1.0 OR MIT-0 OR Apache-2.0 | https://crates.io/crates/dunce/1.0.5 |
| dyn-clone | 1.0.20 | MIT OR Apache-2.0 | https://crates.io/crates/dyn-clone/1.0.20 |
| equivalent | 1.0.2 | Apache-2.0 OR MIT | https://crates.io/crates/equivalent/1.0.2 |
| erased-serde | 0.4.10 | MIT OR Apache-2.0 | https://crates.io/crates/erased-serde/0.4.10 |
| fastrand | 2.5.0 | Apache-2.0 OR MIT | https://crates.io/crates/fastrand/2.5.0 |
| fdeflate | 0.3.7 | MIT OR Apache-2.0 | https://crates.io/crates/fdeflate/0.3.7 |
| flate2 | 1.1.10 | MIT OR Apache-2.0 | https://crates.io/crates/flate2/1.1.10 |
| fnv | 1.0.7 | Apache-2.0 / MIT | https://crates.io/crates/fnv/1.0.7 |
| foldhash | 0.2.0 | Zlib | https://crates.io/crates/foldhash/0.2.0 |
| form_urlencoded | 1.2.2 | MIT OR Apache-2.0 | https://crates.io/crates/form_urlencoded/1.2.2 |
| generic-array | 0.14.7 | MIT | https://crates.io/crates/generic-array/0.14.7 |
| getrandom | 0.3.4 | MIT OR Apache-2.0 | https://crates.io/crates/getrandom/0.3.4 |
| getrandom | 0.4.3 | MIT OR Apache-2.0 | https://crates.io/crates/getrandom/0.4.3 |
| glob | 0.3.4 | MIT OR Apache-2.0 | https://crates.io/crates/glob/0.3.4 |
| hashbrown | 0.12.3 | MIT OR Apache-2.0 | https://crates.io/crates/hashbrown/0.12.3 |
| hashbrown | 0.17.1 | MIT OR Apache-2.0 | https://crates.io/crates/hashbrown/0.17.1 |
| heck | 0.5.0 | MIT OR Apache-2.0 | https://crates.io/crates/heck/0.5.0 |
| html5ever | 0.38.0 | MIT OR Apache-2.0 | https://crates.io/crates/html5ever/0.38.0 |
| http | 1.5.0 | MIT OR Apache-2.0 | https://crates.io/crates/http/1.5.0 |
| ico | 0.5.0 | MIT | https://crates.io/crates/ico/0.5.0 |
| icu_collections | 2.3.0 | Unicode-3.0 | https://crates.io/crates/icu_collections/2.3.0 |
| icu_locale_core | 2.3.0 | Unicode-3.0 | https://crates.io/crates/icu_locale_core/2.3.0 |
| icu_normalizer | 2.3.0 | Unicode-3.0 | https://crates.io/crates/icu_normalizer/2.3.0 |
| icu_normalizer_data | 2.3.0 | Unicode-3.0 | https://crates.io/crates/icu_normalizer_data/2.3.0 |
| icu_properties | 2.3.0 | Unicode-3.0 | https://crates.io/crates/icu_properties/2.3.0 |
| icu_properties_data | 2.3.0 | Unicode-3.0 | https://crates.io/crates/icu_properties_data/2.3.0 |
| icu_provider | 2.3.1 | Unicode-3.0 | https://crates.io/crates/icu_provider/2.3.1 |
| ident_case | 1.0.1 | MIT/Apache-2.0 | https://crates.io/crates/ident_case/1.0.1 |
| idna | 1.1.0 | MIT OR Apache-2.0 | https://crates.io/crates/idna/1.1.0 |
| idna_adapter | 1.2.2 | Apache-2.0 OR MIT | https://crates.io/crates/idna_adapter/1.2.2 |
| indexmap | 1.9.3 | Apache-2.0 OR MIT | https://crates.io/crates/indexmap/1.9.3 |
| indexmap | 2.14.1 | Apache-2.0 OR MIT | https://crates.io/crates/indexmap/2.14.1 |
| infer | 0.19.0 | MIT | https://crates.io/crates/infer/0.19.0 |
| itoa | 1.0.18 | MIT OR Apache-2.0 | https://crates.io/crates/itoa/1.0.18 |
| json-patch | 3.0.1 | MIT/Apache-2.0 | https://crates.io/crates/json-patch/3.0.1 |
| jsonptr | 0.6.3 | MIT OR Apache-2.0 | https://crates.io/crates/jsonptr/0.6.3 |
| keyboard-types | 0.7.0 | MIT OR Apache-2.0 | https://crates.io/crates/keyboard-types/0.7.0 |
| libc | 0.2.189 | MIT OR Apache-2.0 | https://crates.io/crates/libc/0.2.189 |
| litemap | 0.8.3 | Unicode-3.0 | https://crates.io/crates/litemap/0.8.3 |
| lock_api | 0.4.14 | MIT OR Apache-2.0 | https://crates.io/crates/lock_api/0.4.14 |
| log | 0.4.34 | MIT OR Apache-2.0 | https://crates.io/crates/log/0.4.34 |
| markup5ever | 0.38.0 | MIT OR Apache-2.0 | https://crates.io/crates/markup5ever/0.38.0 |
| memchr | 2.8.3 | Unlicense OR MIT | https://crates.io/crates/memchr/2.8.3 |
| mime | 0.3.17 | MIT OR Apache-2.0 | https://crates.io/crates/mime/0.3.17 |
| miniz_oxide | 0.8.9 | MIT OR Zlib OR Apache-2.0 | https://crates.io/crates/miniz_oxide/0.8.9 |
| miniz_oxide | 0.9.1 | MIT OR Zlib OR Apache-2.0 | https://crates.io/crates/miniz_oxide/0.9.1 |
| mio | 1.2.2 | MIT | https://crates.io/crates/mio/1.2.2 |
| muda | 0.19.3 | Apache-2.0 OR MIT | https://crates.io/crates/muda/0.19.3 |
| new_debug_unreachable | 1.0.6 | MIT | https://crates.io/crates/new_debug_unreachable/1.0.6 |
| num-conv | 0.2.2 | MIT OR Apache-2.0 | https://crates.io/crates/num-conv/0.2.2 |
| once_cell | 1.21.4 | MIT OR Apache-2.0 | https://crates.io/crates/once_cell/1.21.4 |
| option-ext | 0.2.0 | MPL-2.0 | https://crates.io/crates/option-ext/0.2.0 |
| parking_lot | 0.12.5 | MIT OR Apache-2.0 | https://crates.io/crates/parking_lot/0.12.5 |
| parking_lot_core | 0.9.12 | MIT OR Apache-2.0 | https://crates.io/crates/parking_lot_core/0.9.12 |
| percent-encoding | 2.3.2 | MIT OR Apache-2.0 | https://crates.io/crates/percent-encoding/2.3.2 |
| phf | 0.13.1 | MIT | https://crates.io/crates/phf/0.13.1 |
| phf_generator | 0.13.1 | MIT | https://crates.io/crates/phf_generator/0.13.1 |
| phf_macros | 0.13.1 | MIT | https://crates.io/crates/phf_macros/0.13.1 |
| phf_shared | 0.13.1 | MIT | https://crates.io/crates/phf_shared/0.13.1 |
| pin-project-lite | 0.2.17 | Apache-2.0 OR MIT | https://crates.io/crates/pin-project-lite/0.2.17 |
| plist | 1.10.0 | MIT | https://crates.io/crates/plist/1.10.0 |
| png | 0.17.16 | MIT OR Apache-2.0 | https://crates.io/crates/png/0.17.16 |
| potential_utf | 0.1.6 | Unicode-3.0 | https://crates.io/crates/potential_utf/0.1.6 |
| powerfmt | 0.2.0 | MIT OR Apache-2.0 | https://crates.io/crates/powerfmt/0.2.0 |
| precomputed-hash | 0.1.1 | MIT | https://crates.io/crates/precomputed-hash/0.1.1 |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 | https://crates.io/crates/proc-macro2/1.0.107 |
| quick-xml | 0.41.0 | MIT | https://crates.io/crates/quick-xml/0.41.0 |
| quote | 1.0.47 | MIT OR Apache-2.0 | https://crates.io/crates/quote/1.0.47 |
| raw-window-handle | 0.6.2 | MIT OR Apache-2.0 OR Zlib | https://crates.io/crates/raw-window-handle/0.6.2 |
| regex | 1.13.1 | MIT OR Apache-2.0 | https://crates.io/crates/regex/1.13.1 |
| regex-automata | 0.4.18 | MIT OR Apache-2.0 | https://crates.io/crates/regex-automata/0.4.18 |
| regex-syntax | 0.8.11 | MIT OR Apache-2.0 | https://crates.io/crates/regex-syntax/0.8.11 |
| rustc-hash | 2.1.3 | Apache-2.0 OR MIT | https://crates.io/crates/rustc-hash/2.1.3 |
| same-file | 1.0.6 | Unlicense/MIT | https://crates.io/crates/same-file/1.0.6 |
| schemars | 0.8.22 | MIT | https://crates.io/crates/schemars/0.8.22 |
| schemars_derive | 0.8.22 | MIT | https://crates.io/crates/schemars_derive/0.8.22 |
| scopeguard | 1.2.0 | MIT OR Apache-2.0 | https://crates.io/crates/scopeguard/1.2.0 |
| selectors | 0.36.1 | MPL-2.0 | https://crates.io/crates/selectors/0.36.1 |
| semver | 1.0.28 | MIT OR Apache-2.0 | https://crates.io/crates/semver/1.0.28 |
| serde | 1.0.229 | MIT OR Apache-2.0 | https://crates.io/crates/serde/1.0.229 |
| serde_core | 1.0.229 | MIT OR Apache-2.0 | https://crates.io/crates/serde_core/1.0.229 |
| serde_derive | 1.0.229 | MIT OR Apache-2.0 | https://crates.io/crates/serde_derive/1.0.229 |
| serde_derive_internals | 0.29.1 | MIT OR Apache-2.0 | https://crates.io/crates/serde_derive_internals/0.29.1 |
| serde_json | 1.0.151 | MIT OR Apache-2.0 | https://crates.io/crates/serde_json/1.0.151 |
| serde_repr | 0.1.21 | MIT OR Apache-2.0 | https://crates.io/crates/serde_repr/0.1.21 |
| serde_spanned | 1.1.1 | MIT OR Apache-2.0 | https://crates.io/crates/serde_spanned/1.1.1 |
| serde_with | 3.22.0 | MIT OR Apache-2.0 | https://crates.io/crates/serde_with/3.22.0 |
| serde_with_macros | 3.22.0 | MIT OR Apache-2.0 | https://crates.io/crates/serde_with_macros/3.22.0 |
| serde-untagged | 0.1.9 | MIT OR Apache-2.0 | https://crates.io/crates/serde-untagged/0.1.9 |
| serialize-to-javascript | 0.1.2 | MIT OR Apache-2.0 | https://crates.io/crates/serialize-to-javascript/0.1.2 |
| serialize-to-javascript-impl | 0.1.2 | MIT OR Apache-2.0 | https://crates.io/crates/serialize-to-javascript-impl/0.1.2 |
| servo_arc | 0.4.3 | MIT OR Apache-2.0 | https://crates.io/crates/servo_arc/0.4.3 |
| sha2 | 0.10.9 | MIT OR Apache-2.0 | https://crates.io/crates/sha2/0.10.9 |
| simd-adler32 | 0.3.10 | MIT | https://crates.io/crates/simd-adler32/0.3.10 |
| siphasher | 1.0.3 | MIT/Apache-2.0 | https://crates.io/crates/siphasher/1.0.3 |
| smallvec | 1.15.2 | MIT OR Apache-2.0 | https://crates.io/crates/smallvec/1.15.2 |
| socket2 | 0.6.5 | MIT OR Apache-2.0 | https://crates.io/crates/socket2/0.6.5 |
| softbuffer | 0.4.8 | MIT OR Apache-2.0 | https://crates.io/crates/softbuffer/0.4.8 |
| stable_deref_trait | 1.2.1 | MIT OR Apache-2.0 | https://crates.io/crates/stable_deref_trait/1.2.1 |
| string_cache | 0.9.0 | MIT OR Apache-2.0 | https://crates.io/crates/string_cache/0.9.0 |
| strsim | 0.11.1 | MIT | https://crates.io/crates/strsim/0.11.1 |
| syn | 2.0.119 | MIT OR Apache-2.0 | https://crates.io/crates/syn/2.0.119 |
| syn | 3.0.4 | MIT OR Apache-2.0 | https://crates.io/crates/syn/3.0.4 |
| synstructure | 0.13.2 | MIT | https://crates.io/crates/synstructure/0.13.2 |
| tao | 0.35.3 | Apache-2.0 | https://crates.io/crates/tao/0.35.3 |
| tauri | 2.11.5 | Apache-2.0 OR MIT | https://crates.io/crates/tauri/2.11.5 |
| tauri-codegen | 2.6.3 | Apache-2.0 OR MIT | https://crates.io/crates/tauri-codegen/2.6.3 |
| tauri-macros | 2.6.3 | Apache-2.0 OR MIT | https://crates.io/crates/tauri-macros/2.6.3 |
| tauri-plugin-autostart | 2.5.1 | Apache-2.0 OR MIT | https://crates.io/crates/tauri-plugin-autostart/2.5.1 |
| tauri-plugin-single-instance | 2.4.5 | Apache-2.0 OR MIT | https://crates.io/crates/tauri-plugin-single-instance/2.4.5 |
| tauri-runtime | 2.11.3 | Apache-2.0 OR MIT | https://crates.io/crates/tauri-runtime/2.11.3 |
| tauri-runtime-wry | 2.11.4 | Apache-2.0 OR MIT | https://crates.io/crates/tauri-runtime-wry/2.11.4 |
| tauri-utils | 2.9.3 | Apache-2.0 OR MIT | https://crates.io/crates/tauri-utils/2.9.3 |
| tendril | 0.5.1 | MIT OR Apache-2.0 | https://crates.io/crates/tendril/0.5.1 |
| thiserror | 1.0.69 | MIT OR Apache-2.0 | https://crates.io/crates/thiserror/1.0.69 |
| thiserror | 2.0.20 | MIT OR Apache-2.0 | https://crates.io/crates/thiserror/2.0.20 |
| thiserror-impl | 1.0.69 | MIT OR Apache-2.0 | https://crates.io/crates/thiserror-impl/1.0.69 |
| thiserror-impl | 2.0.20 | MIT OR Apache-2.0 | https://crates.io/crates/thiserror-impl/2.0.20 |
| time | 0.3.55 | MIT OR Apache-2.0 | https://crates.io/crates/time/0.3.55 |
| time-core | 0.1.9 | MIT OR Apache-2.0 | https://crates.io/crates/time-core/0.1.9 |
| time-macros | 0.2.32 | MIT OR Apache-2.0 | https://crates.io/crates/time-macros/0.2.32 |
| tinystr | 0.8.4 | Unicode-3.0 | https://crates.io/crates/tinystr/0.8.4 |
| tokio | 1.53.1 | MIT | https://crates.io/crates/tokio/1.53.1 |
| toml | 1.1.4+spec-1.1.0 | MIT OR Apache-2.0 | https://crates.io/crates/toml/1.1.4+spec-1.1.0 |
| toml_datetime | 1.1.1+spec-1.1.0 | MIT OR Apache-2.0 | https://crates.io/crates/toml_datetime/1.1.1+spec-1.1.0 |
| toml_parser | 1.1.3+spec-1.1.0 | MIT OR Apache-2.0 | https://crates.io/crates/toml_parser/1.1.3+spec-1.1.0 |
| toml_writer | 1.1.2+spec-1.1.0 | MIT OR Apache-2.0 | https://crates.io/crates/toml_writer/1.1.2+spec-1.1.0 |
| tracing | 0.1.44 | MIT | https://crates.io/crates/tracing/0.1.44 |
| tracing-attributes | 0.1.31 | MIT | https://crates.io/crates/tracing-attributes/0.1.31 |
| tracing-core | 0.1.36 | MIT | https://crates.io/crates/tracing-core/0.1.36 |
| tray-icon | 0.24.2 | MIT OR Apache-2.0 | https://crates.io/crates/tray-icon/0.24.2 |
| typeid | 1.0.3 | MIT OR Apache-2.0 | https://crates.io/crates/typeid/1.0.3 |
| typenum | 1.20.1 | MIT OR Apache-2.0 | https://crates.io/crates/typenum/1.20.1 |
| unic-char-property | 0.9.0 | MIT/Apache-2.0 | https://crates.io/crates/unic-char-property/0.9.0 |
| unic-char-range | 0.9.0 | MIT/Apache-2.0 | https://crates.io/crates/unic-char-range/0.9.0 |
| unic-common | 0.9.0 | MIT/Apache-2.0 | https://crates.io/crates/unic-common/0.9.0 |
| unic-ucd-ident | 0.9.0 | MIT/Apache-2.0 | https://crates.io/crates/unic-ucd-ident/0.9.0 |
| unic-ucd-version | 0.9.0 | MIT/Apache-2.0 | https://crates.io/crates/unic-ucd-version/0.9.0 |
| unicode-ident | 1.0.24 | (MIT OR Apache-2.0) AND Unicode-3.0 | https://crates.io/crates/unicode-ident/1.0.24 |
| unicode-segmentation | 1.13.3 | MIT OR Apache-2.0 | https://crates.io/crates/unicode-segmentation/1.13.3 |
| url | 2.5.8 | MIT OR Apache-2.0 | https://crates.io/crates/url/2.5.8 |
| urlpattern | 0.3.0 | MIT | https://crates.io/crates/urlpattern/0.3.0 |
| utf8_iter | 1.0.4 | Apache-2.0 OR MIT | https://crates.io/crates/utf8_iter/1.0.4 |
| uuid | 1.26.0 | Apache-2.0 OR MIT | https://crates.io/crates/uuid/1.26.0 |
| walkdir | 2.5.0 | Unlicense/MIT | https://crates.io/crates/walkdir/2.5.0 |
| web_atoms | 0.2.6 | MIT OR Apache-2.0 | https://crates.io/crates/web_atoms/0.2.6 |
| webview2-com | 0.38.2 | MIT | https://crates.io/crates/webview2-com/0.38.2 |
| webview2-com-macros | 0.8.1 | MIT | https://crates.io/crates/webview2-com-macros/0.8.1 |
| webview2-com-sys | 0.38.2 | MIT | https://crates.io/crates/webview2-com-sys/0.38.2 |
| winapi | 0.3.9 | MIT/Apache-2.0 | https://crates.io/crates/winapi/0.3.9 |
| winapi-util | 0.1.11 | Unlicense OR MIT | https://crates.io/crates/winapi-util/0.1.11 |
| window-vibrancy | 0.6.0 | Apache-2.0 OR MIT | https://crates.io/crates/window-vibrancy/0.6.0 |
| windows | 0.61.3 | MIT OR Apache-2.0 | https://crates.io/crates/windows/0.61.3 |
| windows_x86_64_msvc | 0.52.6 | MIT OR Apache-2.0 | https://crates.io/crates/windows_x86_64_msvc/0.52.6 |
| windows_x86_64_msvc | 0.53.1 | MIT OR Apache-2.0 | https://crates.io/crates/windows_x86_64_msvc/0.53.1 |
| windows-collections | 0.2.0 | MIT OR Apache-2.0 | https://crates.io/crates/windows-collections/0.2.0 |
| windows-core | 0.61.2 | MIT OR Apache-2.0 | https://crates.io/crates/windows-core/0.61.2 |
| windows-future | 0.2.1 | MIT OR Apache-2.0 | https://crates.io/crates/windows-future/0.2.1 |
| windows-implement | 0.60.2 | MIT OR Apache-2.0 | https://crates.io/crates/windows-implement/0.60.2 |
| windows-interface | 0.59.3 | MIT OR Apache-2.0 | https://crates.io/crates/windows-interface/0.59.3 |
| windows-link | 0.1.3 | MIT OR Apache-2.0 | https://crates.io/crates/windows-link/0.1.3 |
| windows-link | 0.2.1 | MIT OR Apache-2.0 | https://crates.io/crates/windows-link/0.2.1 |
| windows-numerics | 0.2.0 | MIT OR Apache-2.0 | https://crates.io/crates/windows-numerics/0.2.0 |
| windows-result | 0.3.4 | MIT OR Apache-2.0 | https://crates.io/crates/windows-result/0.3.4 |
| windows-strings | 0.4.2 | MIT OR Apache-2.0 | https://crates.io/crates/windows-strings/0.4.2 |
| windows-sys | 0.59.0 | MIT OR Apache-2.0 | https://crates.io/crates/windows-sys/0.59.0 |
| windows-sys | 0.60.2 | MIT OR Apache-2.0 | https://crates.io/crates/windows-sys/0.60.2 |
| windows-sys | 0.61.2 | MIT OR Apache-2.0 | https://crates.io/crates/windows-sys/0.61.2 |
| windows-targets | 0.52.6 | MIT OR Apache-2.0 | https://crates.io/crates/windows-targets/0.52.6 |
| windows-targets | 0.53.5 | MIT OR Apache-2.0 | https://crates.io/crates/windows-targets/0.53.5 |
| windows-threading | 0.1.0 | MIT OR Apache-2.0 | https://crates.io/crates/windows-threading/0.1.0 |
| windows-version | 0.1.7 | MIT OR Apache-2.0 | https://crates.io/crates/windows-version/0.1.7 |
| winnow | 1.0.4 | MIT | https://crates.io/crates/winnow/1.0.4 |
| winreg | 0.10.1 | MIT | https://crates.io/crates/winreg/0.10.1 |
| writeable | 0.6.4 | Unicode-3.0 | https://crates.io/crates/writeable/0.6.4 |
| wry | 0.55.1 | Apache-2.0 OR MIT | https://crates.io/crates/wry/0.55.1 |
| yoke | 0.8.3 | Unicode-3.0 | https://crates.io/crates/yoke/0.8.3 |
| yoke-derive | 0.8.2 | Unicode-3.0 | https://crates.io/crates/yoke-derive/0.8.2 |
| zerofrom | 0.1.8 | Unicode-3.0 | https://crates.io/crates/zerofrom/0.1.8 |
| zerofrom-derive | 0.1.7 | Unicode-3.0 | https://crates.io/crates/zerofrom-derive/0.1.7 |
| zerotrie | 0.2.5 | Unicode-3.0 | https://crates.io/crates/zerotrie/0.2.5 |
| zerovec | 0.11.8 | Unicode-3.0 | https://crates.io/crates/zerovec/0.11.8 |
| zerovec-derive | 0.11.6 | Unicode-3.0 | https://crates.io/crates/zerovec-derive/0.11.6 |
| zmij | 1.0.23 | MIT | https://crates.io/crates/zmij/1.0.23 |

## Distributed fonts

`assets/fonts/SpaceGrotesk-Variable.ttf` and `assets/fonts/Doto-Variable.ttf` are both emitted in the production frontend output. They are the only bundled font binaries. Their copyright notices and complete SIL Open Font License 1.1 texts are preserved in [FONT_LICENSES.md](FONT_LICENSES.md), `assets/fonts/SpaceGrotesk-OFL.txt`, and `assets/fonts/Doto-OFL.txt`.
