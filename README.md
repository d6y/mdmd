# mdmd - Mastodon to Markdown

Plan:

```
Fetches "last" post URL from a URL (e.g.., in github)
    /{instance-name}/id.txt

Reads an RSS feed
    Fetch media
    Convert each to markdown

If github credentials are supplied:
    Fetches latest github revision
    Commit markdown and media files to github
    Updates github with latest URL

Clean up local files
```

Write params:

- image path prefix
- image folder (in git or locally?)
- markdown folder (in git or locally?)
- git creds
- URL for instance id
- path in git to update instance id

