# mdmd - Mastodon RSS to Markdown files

Plan:

- [ ] Fetch "last" post URL from a URL (e.g.., in github) `/{instance-name}/id.txt`
- [x] Read RSS feed
- [x] Find "more recent" posts in feed
- [x] Fetch media
- [x] Convert to markdown
- [ ] Write to github (if credentials supplied)
   - [ ] Fetches latest github revision
   - [ ] Commit markdown 
   - [ ] ...and media files to github
   - [ ] Updates github with latest URL
- [ ] Clean up local files
