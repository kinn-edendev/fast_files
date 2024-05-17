# Fast Files

## File directory storage UI for fast access and sorting 

Features include:
- Saving filesystem endpoints
- Settings default actions on files based on file type
- Adding endpoints to favorites
- Sorting by alphabetical, last modified order, and filetype

### Example output

\> fast\_files

Fast Files - File directory storage UI for fast access and sorting

\[l\] - List saved directories

\[o\] - Open file

\[n\] - New Directory

\[R\] - Refresh saved directories

\[d\] - Default opening process

\[q\] - Quit

\> l

Listing Saved Directories\.\.\.
1. shopping\_list\.txt
2. watchlist\.txt
3. to\_do\.txt
4. design.md
5. hashcat.potfile

\[\#\] - Open file number
\[s\] - Change sort (current: last modified)
\[d\] - Delete directory
\[r\] - Return to main menu
\[q\] - Quit

\> 1

Opening shopping\_list\.txt with vim\.\.\.

\> fast\_files

Fast Files - File directory storage UI for fast access and sorting

Select Action
\[l\] - List saved directories
\[o\] - Open file
\[n\] - New Directory
\[R\] - Refresh saved directories
\[d\] - Default opening process
\[q\] - Quit

\> l

Listing Saved Directories\.\.\.
1. shopping\_list\.txt
2. watchlist\.txt
3. to\_do\.txt
4. design.md
5. hashcat.potfile

\[\#\] - Open file number
\[s\] - Change sort \(current: last modified\)
\[d\] - Delete directory
\[r\] - Return to main menu
\[q\] - Quit

\> s

Listing Saved Directories\.\.\.
1. design.md
2. hashcat.potfile
3. shopping\_list\.txt
4. to\_do\.txt
5. watchlist\.txt

\[\#\] - Open file number
\[s\] - Change sort \(current: alphabetical, descending\)
\[d\] - Delete directory
\[r\] - Return to main menu
\[q\] - Quit

\> s

Listing Saved Directories\.\.\.
1. watchlist\.txt
2. to\_do\.txt
3. shopping\_list\.txt
4. hashcat.potfile
5. design.md

\[\#\] - Open file number
\[s\] - Change sort \(current: alphabetical, ascending\)
\[d\] - Delete directory
\[r\] - Return to main menu
\[q\] - Quit

\> s

Listing Saved Directories\.\.\.
1. hashcat.potfile
2. design.md
3. shopping\_list\.txt
4. to\_do\.txt
5. watchlist\.txt

\[\#\] - Open file number
\[s\] - Change sort \(current: filetype, descending\)
\[d\] - Delete directory
\[r\] - Return to main menu
\[q\] - Quit

\> s

Listing Saved Directories\.\.\.
1. hashcat.potfile
2. design.md
3. watchlist\.txt
4. to\_do\.txt
5. shopping\_list\.txt

\[\#\] - Open file number
\[s\] - Change sort \(current: filetype, ascending\)
\[d\] - Delete directory
\[r\] - Return to main menu
\[q\] - Quit

\> d

Select directory number or file name to delete\.\.\.

\> to\_do\.txt

Deleting to\_do\.txt from registry\.\.\. \(fast\_files does not truly delete files\)

Listing Saved Directories\.\.\.
1. hashcat.potfile
2. design.md
3. watchlist\.txt
4. shopping\_list\.txt

\[\#\] - Open file number
\[s\] - Change sort \(current: filetype, ascending\)
\[d\] - Delete directory
\[r\] - Return to main menu
\[q\] - Quit

\> d 4

Deleting shopping\_list\.txt from registry\.\.\. \(fast\_files does not truly delete files\)

Listing Saved Directories\.\.\.
1. hashcat.potfile
2. design.md
3. watchlist\.txt

\[\#\] - Open file number
\[s\] - Change sort \(current: filetype, ascending\)
\[d\] - Delete directory
\[r\] - Return to main menu
\[q\] - Quit

\> d hashcat.potfile

Deleting hashcat\.potfile from registry\.\.\. \(fast\_files does not truly delete files\)

\> d hashcat.potfilea

No file hashcat\.potfile found in registry.

Listing Saved Directories\.\.\.
1. hashcat.potfile
2. design.md
3. watchlist\.txt

\[\#\] - Open file number
\[s\] - Change sort \(current: filetype, ascending\)
\[d\] - Delete directory
\[r\] - Return to main menu
\[q\] - Quit

\> r

Fast Files - File directory storage UI for fast access and sorting

Select Action
\[l\] - List saved directories
\[o\] - Open File
\[n\] - New Directory
\[R\] - Refresh saved directories
\[d\] - Default opening process
\[q\] - Quit

\> n

Please enter new filepath\.\.\.

\> ~/Documents/important/taxes\.txt

taxes\.txt added to saved directories...

Select Action
\[l\] - List saved directories
\[o\] - Open File
\[n\] - New Directory
\[R\] - Refresh saved directories
\[d\] - Default opening process
\[q\] - Quit

\> n ~/Web/design/index.html

index\.html added to saved directories\.\.\.
New filetype discovered, what process would you like to open "html" files with?

\> nano

"html" file will now be opened with "nano" by default, use "d" from the main menu to change.

Select Action
\[l\] - List saved directories
\[o\] - Open File
\[n\] - New Directory
\[R\] - Refresh saved directories
\[d\] - Default opening process
\[q\] - Quit

\> r

Refreshing registry... \(2 files updated\)

Select Action
\[l\] - List saved directories
\[o\] - Open File
\[n\] - New Directory
\[R\] - Refresh saved directories
\[p\] - Default opening process
\[q\] - Quit

\> p

Listing saved opening actions\.\.\.
1. txt -\> vim
2. potfile -\> vim
3. md -\> vim
4. html -\> nano

\[\#\] - Change action for #
\[d\] - Delete action
\[r\] - Return to main menu
\[q\] - Quit

