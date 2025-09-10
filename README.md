ghai: advancements to the gh command
====================================

ghai-my-issues:  My preferred output for browsing issues.  Clean and easy to navigate from the
terminal.

ghai-process-notifications:  AI-powered notification processor that uses policies to automatically
decide whether GitHub notifications should be marked as read or kept unread. Features comment 
tracking since last read, customizable decision policies, and a --mark-read-by-default option.

## Example Notification Policy

The notification processor uses policy files to make decisions. Here's an example policy:

```
If the issue or pull request was closed after it was last read or it was never read, output {"action": "mark-unread"}
If the issue or pull request has a comment after it was last read, output {"action": "mark-unread"}
If the issue or pull request has no comments, output {"action": "mark-read"}
If the issue or pull request is somehow associated with @rescrv, output {"action": "mark-unread"}
```

This policy keeps notifications unread for recently closed items, items with new comments, items with
no comments at all, and items associated with @rescrv, while marking everything else as read.
