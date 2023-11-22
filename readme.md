# Kladez API

[![Uptime Badge][uptime-badge]][uptime-badge]
\
[![Test Results Badge][tests-badge]][tests-url]
[![Coverage Badge][coverage-badge]][coverage-url]
[![Unsafe Forbidden Badge][unsafe-forbidden-badge]][unsafe-forbidden-url]
\
[![License Badge][license-badge]](license.txt)

## Contributing

Project uses [`pnpm`][pnpm], [`commitlint`][commitlint] and [`lefthook`][lefthook] for Git hooks.\
To install all additional development dependencies [install Node.js][install-nodejs] v20.9.0 and run:
```sh
corepack enable pnpm
corepack prepare pnpm@8.10.5 --activate
pnpm install
```

## License

This project is licensed under the MIT License.\
See the [license.txt](license.txt) file for details.


[uptime-badge]: https://img.shields.io/uptimerobot/ratio/m795031693-ba4eb19f16952b0092e4f7e6

[tests-badge]: https://github.com/kladez/api/workflows/tests/badge.svg
[tests-url]: https://dev.azure.com/kladez/OpenSource/_build/latest?definitionId=98&view=ms.vss-test-web.build-test-results-tab

[coverage-badge]: https://img.shields.io/azure-devops/coverage/kladez/api/main
[coverage-url]: https://codecov.io/gh/kladez/api

[unsafe-forbidden-badge]: https://img.shields.io/badge/unsafe-forbidden-success
[unsafe-forbidden-url]: https://github.com/rust-secure-code/safety-dance

[license-badge]: https://img.shields.io/github/license/kladez/api

[commitlint]: https://commitlint.js.org
[lefthook]: https://github.com/evilmartians/lefthook
[pnpm]: https://pnpm.io
[install-nodejs]: https://nodejs.org/en/learn/getting-started/how-to-install-nodejs
