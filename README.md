# mdmd - Mastodon RSS to Markdown files

Plan:

- [x] Fetch "last" post URL from a URL (e.g.., in github) `/{instance-name}/id.txt`
- [ ] Fetch "last" GUI from git directly (as publishing takes time)
- [x] Read local RSS feed file
- [ ] Fetch remote RSS feed
- [x] Find "more recent" posts in feed
- [x] Fetch media
- [x] Convert to markdown
- [ ] Write to github (if credentials supplied)
   - [x] Fetches latest github revision
   - [x] Commit markdown 
   - [x] ...and media files to github
   - [x] Updates github id.txt with latest URL
- [ ] Clean up local files

# Useful links

- https://docs.github.com/en/graphql/reference/input-objects#filechanges
- https://docs.github.com/en/graphql
- https://docs.github.com/en/graphql/overview/resource-limitations
