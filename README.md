# mdmd - Mastodon RSS to Markdown files

Plan:

- [x] Fetch "last" post URL from a URL (e.g.., in github) `/{instance-name}/id.txt`
- [x] Fetch "last" GUI from git directly (as publishing takes time)
- [x] Read local RSS feed file
- [x] Fetch remote RSS feed
- [x] Find "more recent" posts in feed
- [x] Fetch media
- [x] Add argument for controlling number of items to fetch per run
- [x] Convert to markdown
- [x] Write to github (if credentials supplied)
   - [x] Fetches latest github revision
   - [x] Commit markdown 
   - [x] ...and media files to github
   - [x] Updates github id.txt with latest URL
- [x] Clean up local files (use tmpdir)
- [x] Add docker build
- [ ] Deploy

# Useful links

- https://docs.github.com/en/graphql/reference/input-objects#filechanges
- https://docs.github.com/en/graphql
- https://docs.github.com/en/graphql/overview/resource-limitations

# Docker build

```
docker build -t mdmd .
```

Example run:

```
docker run -it -e GITHUB_TOKEN=$GITHUB_TOKEN -e GITHUB_REPO=$GITHUB_REPO -e RUST_LOG=$RUST_LOG  --rm --name running-mdmd mdmd
```

# Environment

```
export GITHUB_TOKEN=???
export GITHUB_REPO=d6y/richard.dallaway.com
export RUST_LOG=INFO,rustls=off
```

...but see `cargo run -- -help` for options.
