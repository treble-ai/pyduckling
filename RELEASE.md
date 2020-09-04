To release a new version of pyduckling:
1. git fetch upstream && git checkout upstream/master
2. Close milestone on GitHub
3. git clean -xfdi
4. Update CHANGELOG.md with loghub
5. git add -A && git commit -m "Update Changelog"
6. Update release version in ``Cargo.toml`` (set release version, remove 'dev0')
7. git add -A && git commit -m "Release vX.X.X"
8. git tag -a vX.X.X -m "Release vX.X.X"
9. git push upstream master
10. git push upstream --tags
11. Wait for GitHub Actions to produce the wheels
12. Download the wheels locally for Linux and Mac
13. twine upload dist/*
14. Update development version in ``Cargo.toml`` (add '-dev0' and increment minor, see [1](#explanation))
15. git add -A && git commit -m "Back to work"
16. git push upstream master


[<a name="explanation">1</a>] We need to append '-dev0', as Cargo does not support the '.dev0'
syntax.
