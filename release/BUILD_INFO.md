# MEROA 0.3.0

- Build source commit: 55e5cae736451597ae4d94c364fa26e16093a878
- Build date: 2026-10-26
- Target: Windows x86_64
- Distribution artifact: `MEROA_0.3.0_x64-setup.exe`
- Installer: NSIS
- SHA-256: `4E86239A3C7A97BC92AA0B13A3451A082C2E5709C7907B7F0246767E83FA65C9`
- Authenticode: Not signed
- Validation: 20 frontend/unit tests; 21 native tests; one explicit live GitHub HTTPS metadata test; TypeScript and production frontend build; Windows release build and NSIS bundling; Chromium frontend interaction checks.
- Default skipped tests: three opt-in native tests; the GitHub HTTPS test was also run separately and passed.
- Runtime limitation: Windows installer upgrade execution and low-powered hardware performance were not manually verified.
