# Security Policy

## Supported Versions

Security fixes target the latest released version of `rpath`.

## Reporting a Vulnerability

Please report suspected vulnerabilities privately to:

```text
me@byjonas.dev
```

Do not open a public issue for vulnerabilities involving shell injection,
profile writing, registry edits, PATH hijacking, or unsafe environment mutation.

Please include:

- Affected platform and shell
- `rpath --version`
- Reproduction steps
- Expected and actual behavior
- Whether the issue requires local access

## Security Principles

- No runtime network calls.
- No system-wide environment mutation during refresh.
- No shell commands emitted when the environment plan has hard errors.
- Back up profile files before wrapper installation changes them.
- Keep registry and service integrations under user scope where possible.
