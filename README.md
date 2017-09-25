# Metarhia Protocol

[![Travis Build Status][travis-badge]][travis-url]
[![AppVeyor Build Status][appveyor-badge]][appveyor-url]

## Contributing

Please adhere to [Conventional Commits][] styleguide for commit messages (`npm
install` creates a Git hook that lints your commit messages, and they are also
checked on CI, but please write them properly beforehand so that they don't get
rejected.  If that happens locally while committing, though, don't worry, your
commit message isn't lost, you can still find it in `.git/COMMIT_EDITMSG`).

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

[Conventional Commits]: https://conventionalcommits.org
[LICENSE]: LICENSE
[appveyor-badge]: https://ci.appveyor.com/api/projects/status/wuffvoyxtplk1hvd?svg=true
[appveyor-url]: https://ci.appveyor.com/project/metarhia/protocol
[travis-badge]: https://travis-ci.org/metarhia/protocol.svg?branch=master
[travis-url]: https://travis-ci.org/metarhia/protocol
