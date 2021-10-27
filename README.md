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
        - Paths
    - File Name
    - Metadata
        - File size
        - Last changed
        - Is directory
    
    A wrapper around the database for threading reasons?

- Indexer

    Writes database to file.
    Why have indexer and database?

- Watcher

    https://github.com/notify-rs/notify

    https://github.com/facebook/watchmano

    Append files to the database when new files are made.

    Delete files to the database when they are deleted.