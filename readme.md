# A point of sale and metrics client for the town of Silverplume, Colorado. 
- Easy to use
- Writes all data to csv to make further analysis easy
- tracks months (30 day periods) for you, and copies data to new files at the end of the month

# Usage instructions
1. Download the program from the release tab on the right
2. Run it!
3. Enter data throughout the day. Whenever it is valid to add data, then the button to do so will turn bright blue. 
4. At the end of a day, week, or month copy any files you want from the data directory out for further analysis.

## DO NOT EVER DELETE `database.lock`, the program will forget what a month is.
## DO NOT WORK DIRECTLY ON FILES IN THE `data` DIRECTORY, YOU WILL CORRUPT IT. ALWAYS COPY THEM OUT FIRST.

# Known Bugs
- Metrics reads "-0.00" for a number of fields, this is a qurik of numerical storage, and isn't going to be fixed this version.

# Improvements
This really ought to be using a database backing like Sqlite which can run entirely on one PC...But doing so would 
require more thought.