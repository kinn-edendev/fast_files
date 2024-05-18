## Example Outputs

\> fast\_files

Fast Files \- File directory storage UI for fast access and sorting

Select Action
\[l\] \- List saved directories
\[d\] \- List default opening process
\[r\] \- Refresh saved directories
\[q\] \- Quit

\> l

Listing Saved Directories\.\.\.

Select Action
\[#\] \- Open file number
\[s\] \- Change sort (current: last modified)
\[n\] \- New Directory
\[d\] \- Delete directory
\[r\] \- Return to main menu
\[q\] \- Quit

\> n

Please enter a new filepath\.\.\.

\> ~/documents/anime\.txt ~/documents/movies\.txt ~/web/server/index\.html ~/hashcat/hashcat\.potfile

anime\.txt added and saved to directories\.\.\.
New filetype discovered, what process would you like to open '\.txt' files with?
\> vim

'\.txt' files will now be opened with 'vim' by default, use 'd' from the default opening process menu to change\.

movies\.txt added and saved to directories\.\.\.

index\.html added and saved to directories\.\.\.
New filetype discovered, what process would you like to open '\.html' files with?
\> nano

'\.html' files will now be opened with 'nano' by default, use 'd' from the default opening process menu to change\.

hashcat.potfile added and saved to directories\.\.\.
New filetype discovered, what process would you like to open '\.html' files with?
\> vi
'\.potfile' files will now be opened with 'vi' by default, use 'd' from the default opening process menu to change\.

Listing Saved Directories\.\.\.
1. hashcat.potfile
2. index.html
3. anime.txt
4. movies.txt

Select Action
\[\#\] \- Open file number
\[s\] \- Change sort (current: last modified)
\[n\] \- New Directory
\[d\] \- Delete directory
\[r\] \- Return to main menu
\[q\] \- Quit

\> d

Select directory number or file name to delete\.\.\.
\> anime.txt index.html

Deleting anime\.txt from registry\.\.\. \(fast\_files does not truly delete files\) 

Deleting index\.html from registry\.\.\. \(fast\_files does not truly delete files\)

Listing Saved Directories\.\.\.
1. hashcat.potfile
2. movies.txt

Select Action
\[\#\] \- Open file number
\[s\] \- Change sort (current: last modified)
\[n\] \- New Directory
\[d\] \- Delete directory
\[r\] \- Return to main menu
\[q\] \- Quit

\> r

Fast Files \- File directory storage UI for fast access and sorting

Select Action
\[l\] \- List saved directories
\[d\] \- List default opening process
\[r\] \- Refresh saved directories
\[q\] \- Quit

\> d

Listing Saved Opening Actions\.\.\.
1. \.html \-\> nano
2. \.txt \-\> vim
3. \.potfile \-\> vi

Select Action
\[n\] \- New Launch Command
\[d\] \- Delete Launch Command
\[r\] \- Return to main menu
\[q\] \- Quit

\> n

Please enter a file extension\.\.\.
\> \.md

What command would you like to open '\.md' files with?
\> vim

'\.md' files will now be opened with 'vim' by default, use 'n' from the default opening process menu to change\.

Listing Saved Opening Actions\.\.\.
1. \.html \-\> nano
2. \.txt \-\> vim
3. \.potfile \-\> vi
4. \.md \-\> vim

Select Action
\[n\] \- New Launch Command
\[d\] \- Delete Launch Command
\[r\] \- Return to main menu
\[q\] \- Quit

\> d

Select launch command to delete\.\.\.
\> \.potfile

Deleting '\.potfile' from registry\.\.\. \(fast\_files does not truly delete files\)

Listing Saved Opening Actions\.\.\.
1. \.html \-\> nano
2. \.txt \-\> vim
3. \.md \-\> vim

Select Action
\[n\] \- New Launch Command
\[d\] \- Delete Launch Command
\[r\] \- Return to main menu
\[q\] \- Quit

\> r

Fast Files \- File directory storage UI for fast access and sorting

Select Action
\[l\] \- List saved directories
\[d\] \- List default opening process
\[r\] \- Refresh saved directories
\[q\] \- Quit

\> r

Refreshing directories and launch commands from file\.\.\.

Select Action
\[l\] \- List saved directories
\[d\] \- List default opening process
\[r\] \- Refresh saved directories
\[q\] \- Quit

\> q
