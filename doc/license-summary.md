# Cando-RS License Summary

**Document Purpose**: Executive summary for project and team leaders  
**Date**: February 2025  
**Full Details**: See [LICENSE-THIRD-PARTY.txt](LICENSE-THIRD-PARTY.txt)

---

## Quick Summary

✅ **Project is fully compliant for commercial and proprietary use**  
✅ **No licensing conflicts or GPL dependencies detected**  
✅ **All 529 external dependencies are compatible**

---

## Project License

**Cando-RS is dual-licensed:**

- **MIT License** - https://opensource.org/licenses/MIT
- **Apache License 2.0** - https://www.apache.org/licenses/LICENSE-2.0

Users may choose either license. This provides maximum flexibility for both open-source and commercial use.

---

## Dependency Overview

| Category                      | Count | License Types               |
| ----------------------------- | ----- | --------------------------- |
| **Internal Workspace Crates** | 21    | MIT OR Apache-2.0 (all)     |
| **External Dependencies**     | 529   | Various permissive licenses |
| **Total Packages**            | 551   | -                           |

---

## License Distribution (External Dependencies)

| License Category                         | Count | Status        |
| ---------------------------------------- | ----- | ------------- |
| Permissive (MIT, Apache, BSD, ISC, etc.) | 493   | ✅ Compatible |
| Weak Copyleft (MPL-2.0)                  | 3     | ✅ Compatible |
| Public Domain / Unlicense                | 16    | ✅ Compatible |
| Unicode Data Files                       | 21    | ✅ Compatible |
| Special/Compound Licenses                | 7     | ✅ Compatible |

### Top License Types

- **MIT OR Apache-2.0**: 259 crates (49%)
- **MIT**: 124 crates (23%)
- **MIT/Apache-2.0**: 31 crates (6%)
- **Unicode-3.0**: 18 crates (3%)
- **Apache-2.0 OR MIT**: 17 crates (3%)
- **Apache-2.0**: 10 crates (2%)
- **Other permissive**: 70 crates (13%)

---

## Copyleft Analysis

### Weak Copyleft (MPL-2.0) - 3 Crates

The project includes 3 dependencies under MPL-2.0 (Mozilla Public License 2.0):

1. `colored` v3.1.1 - Terminal color output
2. `option-ext` v0.2.0 - Option type extensions
3. _(one more in detailed report)_

**Impact**: MPL-2.0 is a **file-level copyleft** license, not project-level:

- ✅ Compatible with MIT/Apache-2.0 dual licensing
- ✅ Modifications to MPL files must remain MPL-2.0
- ✅ The overall project remains MIT OR Apache-2.0
- ✅ Does not affect commercial or proprietary use

### LGPL Dual-Licensed - 1 Crate

- `r-efi` v5.3.0 - Offers **MIT OR Apache-2.0** option

**Impact**: Since this crate offers MIT/Apache-2.0, we use that option:

- ✅ No LGPL obligations apply to this project

### Strong Copyleft (GPL)

- **Zero GPL dependencies** ✅

---

## Compliance Assessment

### ✅ Commercial Use

- **Permitted**: All licenses allow commercial use
- **Restrictions**: None

### ✅ Proprietary Modifications

- **Permitted**: All licenses allow proprietary modifications
- **Restrictions**: MPL-2.0 files require source disclosure if modified (3 crates)

### ✅ Distribution

- **Permitted**: Free distribution under MIT or Apache-2.0
- **Requirements**: Include copyright notices and license text

### ✅ Patent Protection

- **Apache-2.0**: Provides explicit patent grant
- **MIT**: No explicit patent clause (common for MIT)

---

## Special Licenses

A small number of dependencies use specialized licenses:

- **Unicode-3.0** (18 crates): Permissive license for Unicode data
- **ISC** (2 crates): Similar to MIT, highly permissive
- **BSD-3-Clause** (4 crates): Permissive with attribution requirement
- **BSL-1.0** (2 crates): Boost Software License, permissive
- **CC0-1.0** (2 crates): Public domain dedication
- **WTFPL** (1 crate): "Do What The F\*\*\* You Want To" - extremely permissive
- **bzip2-1.0.6** (1 crate): BSD-style license

**All are compatible with commercial use.**

---

## Recommendations

### For Legal Review

1. ✅ Project license is standard dual MIT/Apache-2.0
2. ✅ All dependencies use well-known, permissive licenses
3. ✅ No GPL or AGPL dependencies present
4. ⚠️ Be aware of 3 MPL-2.0 dependencies (file-level copyleft)

### For Distribution

1. Include this LICENSE-SUMMARY.md and LICENSE-THIRD-PARTY.txt
2. Include MIT and/or Apache-2.0 license text
3. Include copyright notices for dependencies (see detailed report)
4. No special obligations for binary distribution

### For Contributions

1. All contributions must be compatible with MIT OR Apache-2.0
2. Contributors implicitly agree to dual-license their contributions
3. Consider requiring a Contributor License Agreement (CLA) for clarity

---

## Internal Workspace Crates (Project Components)

All 21 internal crates are dual-licensed **MIT OR Apache-2.0**:

1. can-log-analyzer
2. candump-rs
3. cansend-rs
4. count-hvpc-signals
5. dump-messages
6. emp-simulator
7. hvpc-simulator
8. j1939-simulator
9. monitor-can
10. cando-can-monitor
11. cando-cfg
12. cando-codegen
13. cando-config
14. cando-core
15. cando-messages
16. cando-meta
17. cando-simulator-common
18. cando-webui
19. rust-can-util
20. rust-websocket-query
21. udc-simulator

---

## Conclusion

**The Cando-RS project is fully compliant for commercial use with no licensing conflicts.**

- ✅ Dual MIT/Apache-2.0 licensing provides maximum flexibility
- ✅ All 529 external dependencies are compatible
- ✅ No strong copyleft (GPL) dependencies
- ✅ Safe for proprietary and commercial deployment
- ✅ Only 3 weak copyleft (MPL-2.0) dependencies with minimal impact

**No legal blockers identified for commercial or proprietary use.**

---

## References

- **Detailed Inventory**: [LICENSE-THIRD-PARTY.txt](LICENSE-THIRD-PARTY.txt) (958 lines, all packages listed)
- **MIT License**: https://opensource.org/licenses/MIT
- **Apache-2.0 License**: https://www.apache.org/licenses/LICENSE-2.0
- **MPL-2.0 License**: https://www.mozilla.org/en-US/MPL/2.0/
- **SPDX License List**: https://spdx.org/licenses/
- **Project Repository**: https://github.com/suykerbuyk/cando-rs

---

## Document Generation

This summary was automatically generated from Cargo.toml metadata using:

```bash
cargo metadata --format-version 1 --all-features
```

**Last Updated**: February 2025  
**For Questions**: Contact project maintainers
