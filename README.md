### Design

```
UI <--> DATABASE <--> INDEXER 
           ^
 ⠀         |
        Watcher
 ```

- UI
    - Query
    - Selection
    - Database

- Database
    - Radix Tree
    
    A wrapper around the database for threading reasons?
    I don't really want to hold the metadata since it's slow to get,
    maybe the UI can just ask for it?
    It's not like I'm showing all the items at once.
    How would you order by filesize then?

    Only get the metadata of files currently rendered by the UI
    that way there are only 30 or so calls, this may be fast enough to work
- Watcher

    https://github.com/notify-rs/notify

    https://github.com/facebook/watchmano

    Append files to the database when new files are made.

    Delete files to the database when they are deleted.