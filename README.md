# Metarhia Protocol

[![Travis Build Status][travis-badge]][travis-url]
[![AppVeyor Build Status][appveyor-badge]][appveyor-url]
[![Documentation Status][readthedocs-badge]][readthedocs-url]
[![Greenkeeper Status][greenkeeper-badge]][greenkeeper-url]

Metarhia Protocol (`mhp`) is a protocol for RPC, event streams and two-way
asynchronous data transfer that supports multiplexing and is capable of
handling network errors gracefully.

This project is covered by a [Code of Conduct](CODE_OF_CONDUCT.md).

[![NPM Status][npm-badge]][npm-url]

## Documentation

Check out our documentation at <https://mhp.readthedocs.io>.

## Contributing

Please adhere to [Conventional Commits][] styleguide for commit messages
(`npm install` creates a Git hook that lints your commit messages, and they are
also checked on CI, but please write them properly beforehand so that they don't
get rejected. If that happens locally while committing, though, don't worry,
your commit message isn't lost, you can still find it in `.git/COMMIT_EDITMSG`).

### Releasing

Collaborators can release new versions using

```console
npm run release
git push origin master --follow-tags
npm publish
```

This will update the version in `package.json` and `package-lock.json`
according to semantic versioning using commit messages to determine whether it
is a patch, minor or major release, update the changelog, tag the new version
in Git, and publish it to npm registry.

## License

MIT. See the [LICENSE][] file for details.

[conventional commits]: https://conventionalcommits.org
[license]: LICENSE
[appveyor-badge]: https://ci.appveyor.com/api/projects/status/wuffvoyxtplk1hvd?svg=true
[appveyor-url]: https://ci.appveyor.com/project/metarhia/protocol
[greenkeeper-badge]: https://badges.greenkeeper.io/metarhia/protocol.svg
[greenkeeper-url]: https://greenkeeper.io
[npm-badge]: https://nodei.co/npm/mhp.png
[npm-url]: https://npmjs.com/package/mhp
[readthedocs-badge]: https://readthedocs.org/projects/mhp/badge/?version=latest
[readthedocs-url]: https://mhp.readthedocs.io/en/latest/
[travis-badge]: https://travis-ci.org/metarhia/protocol.svg?branch=master
[travis-url]: https://travis-ci.org/metarhia/protocol
