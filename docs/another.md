# Anther tools help:

## GSPLIT

_For advanced users only._

![img](https://www.gdgsoft.com/gsplit/help/bullet2.png)  CONTENTS:

1.  [Command line switches supported by GSplit](https://www.gdgsoft.com/gsplit/help/TechnicalDoc.htm#P1)
2.  [Using Batch text files and splitting multiple files](https://www.gdgsoft.com/gsplit/help/TechnicalDoc.htm#P2)
3.  [Command line switches for GUnite](https://www.gdgsoft.com/gsplit/help/TechnicalDoc.htm#P4)
4.  [Adding predefined values for piece size](https://www.gdgsoft.com/gsplit/help/TechnicalDoc.htm#P3)

### 1. Command line switches supported by GSplit

GSplit supports multiple command line switches.

![img](https://www.gdgsoft.com/gsplit/help/bullet1.png)**What is a switch?**  
All switches are specified with a forward slash or a dash and are sometimes followed by a value. An example of a valid switch: "c:\program files\myfile.dat" /C  
Please note that if you are specifying files or folders with spaces in them, you should enclose them in  **quotation marks**.

![img](https://www.gdgsoft.com/gsplit/help/bullet1.png)**The switches handled by GSplit**  
You may enter the switches like the following scheme:  
GSPLIT.EXE "Param1" "Param2" "Param3" "Param 4"

-   Param1: specify the full path to the file you would like to split. Can be left to blank if you have a configuration file or a batch file: just type "".
-   Param2: lets you specify some optional commands.
    -   -go : starts splitting the file specified in "Param1" after having loaded the default configuration file. Optional.
    -   -c: this switch causes GSplit to load the profile file specified in Param3 and then possibly split the associated file. Optional.
    -   -t: closes GSplit after having split the file. Optional.
    -   -b: reads the batch text file specified in "Param3". See below for further information about batch text files.
    -   -i: causes the batch text file to be executed: in fact, -b only causes the batch text file to be read and -i makes GSplit split the source file(s).
    -   -s: activates the silent mode, GSplit works as a daemon, no GUI is displayed. Only if -i or -go are specified too.
    -   -l: saves the output log to the file specified in "Param 4".
-   Param3: (optional) the full path to the configuration or the batch file to be used.
-   Param4: (optional) the full path to the output log file (-l option required).

Note: the switches -c and -b cannot be used in the same time.

![img](https://www.gdgsoft.com/gsplit/help/bullet1.png)  **GSplit exit codes:**

GSplit always returns an exit code when it closes. You can use this exit code in batch files to know whether a file was successfully split or not.

**Exit Code**

**Meaning**

0

No error, file successfully split

1

Original file not found

2

Destination folder not specified

3

Error in settings (see log)

4

Error while splitting (see log)

5

Invalid command line parameters (invalid filenames or paths)

![img](https://www.gdgsoft.com/gsplit/help/bullet1.png)  **Examples:**

-   Split the MYPIC.EXE file:  
    GSPLIT.EXE "c:\my documents\mypic.exe" -go
-   Open the MYCONF.GSC profile to split MYPIC.EXE:  
    GSPLIT.EXE "c:\my documents\mypic.exe" -c "c:\my documents\myconf.gsc"
-   Split the file according to the MYCONF.GSC profile silently (in background) and save the log file:  
    GSPLIT.EXE "" -c-go-s-t-l "c:\my documents\myconf.gsc" "c:\my documents\outputlog.rtf"
-   Execute the specified batch file silently:  
    GSPLIT.EXE "" -b-i-s-t "c:\my documents\mybatch.txt"

### 2. Using Batch text files and splitting multiple files

Batch text files are normal text files (.txt) that contain instructions for GSplit in order to split one or more files. Batch files are useful for external applications which need to split files.  
  
![img](https://www.gdgsoft.com/gsplit/help/bullet1.png)How do batch files work?  
  
Batch text files work like the old Windows configuration (.ini extension) files. They contain two sections "Main" and "FileList". The "Main" section is required.  
Below you can see a simple example:

_; GSplit Batch File  
; Pass these parameters to GSplit in order to run this file:  
; GSplit.exe "no" -b-i-t "[path to this file]"  
  
[Main]  
ConfigFile=  
Source=c:\my programs\mymovie.avi  
DestFolder=d:\distribution\movies  
PieSize=100  
PieSizeUnit=MB  
SplitMethod=0  
TypePieSpanned=0  
Title=My new movie_

![img](https://www.gdgsoft.com/gsplit/help/bullet1.png)  The different entries of [Main] are explained here:

-   **ConfigFile**: GSplit reads the configuration file specified by the value. Example:  _ConfigFile=c:\my documents\GSplit Config\myconfig1.gsf_
-   **MulFiles**: two values 1 or 0. 1 means that GSplit will have to split multiple files. And 0 means one file only. Optional (0 is the default value).
-   **DestFolder**: the destination folder for the piece set.
-   **Source**: if MulFiles is set to 0, then this entry should point to the file to be split. Mandatory if MulFiles is set to 0.
-   **TypePieSpanned**: two values 1 or 0. 1 means that GSplit will create disk-spanned pieces, 0 means blocked pieces. Optional.
-   **SplitMethod**: three values from 0 to 2. Only required if TypePieSpanned is set to 0 (blocked pieces). Should be  **0**  if you set the size of the piece files,  **1**  if you specify the number of pieces you want to create or  **2**  if you want to split using  [occurrences of a pattern](https://www.gdgsoft.com/gsplit/help/SplitByLinesOccurrences.htm).
-   **PieSize**: size of the blocked pieces (TypePieSpanned set to 0 AND SplitMethod set to 0). Enter a real positive number. The unit is specified by PieSizeUnit.
-   **PieSizeUnit**: the unit for PieSize. 4 values possible: Bytes, KB, MB and GB. By default this is "Bytes".
-   **PieNumber**: number of blocked pieces to create (TypePieSpanned set to 0 AND SplitMethod set to 1). Positive integer only.
-   **Pattern**: the pattern used to split the file (TypePieSpanned set to 0 AND SplitMethod set to 2).
-   **NumberOccurrences**: defines the number of occurrences of the pattern by piece. Positive integer only.
-   **SplitBeforePattern**: two values 1 or 0. 0 means GSplit splits after the nth occurrence of the pattern, 1 before.
-   **CustomHeader**: three values from 0 to 2. 0: does nothing, 1:  [copies all data prior to the first occurrence or line](https://www.gdgsoft.com/gsplit/help/Split_Options.htm), 2: insert the text specified by CustomHeaderText.
-   **CustomHeaderText**: lets you enter the header you want, only required if CustomHeader is set to 2.
-   **Title**: the title of the piece set (optional).
-   **Author**: the author of the piece set (optional).
-   **Description**: the description of the piece set (optional).
-   **PieMask**: indicates the  [mask for piece filenames](https://www.gdgsoft.com/gsplit/help/The_mask_for_pieces_filenames.htm). Optional.
-   **NoPieHeader**: two values 1 or 0. 1 means that GSplit won't  [add its tags](https://www.gdgsoft.com/gsplit/help/Split_Options.htm)  to the piece files (useful for text-based files). And 0 means the default behavior (tags enabled). Optional.
-   **LogFile**: full path to the splitting log to save when the operation is done. Optional.

![img](https://www.gdgsoft.com/gsplit/help/bullet1.png)  If MulFiles is set to 1, then GSplit will need a second section called "**FileList**". This section lists all the files that will be split.  
Files should be listed like this sample:

_[FileList]  
0=c:\my documents\my music\aria.mp3  
1=c:\my documents\my music\dream.mp3  
2=c:\program files\my music reader\mp3pack.zip  
...  
_  
Please refer to the "[Splitting multiple files](https://www.gdgsoft.com/gsplit/help/Multiple_Files.htm)" help topic for further explanation about how GSplit can split several files in one time.  
  
Below you can see a simple example working with multiple files:  
  
_; GSplit Batch File  
; Pass these parameters to GSplit in order to run this file:  
; GSplit.exe "no" -b-i-t "[path to this file]"  
  
[Main]  
ConfigFile=  
MulFiles=1  
Source=  
DestFolder=a:\  
TypePieSpanned=1  
Title=My programs  
  
[FileList]  
0=c:\my work\my programs\program1.exe  
1=c:\my work\my files\help1.chm_

### 3. Command line switches for GUnite

![img](https://www.gdgsoft.com/gsplit/help/bullet1.png)**The switches handled by GUnite**  
You may enter the switches like the following scheme:  
GUNITE.EXE [path to first piece] -u-v-m [output file] [buffer size]

-   [path to first piece]: specify the full path to the first piece file. Can be left to blank for memory options.
-   Param2: lets you specify some optional commands.
    -   -u: unite pieces and exits.
    -   -v: opens the first piece and display the "Verify piece set" page. Incompatible with -u.
    -   -m: set the internal memory buffer's size (in kB). Optional.
-   [output file]: (optional) the full path to the future restored file. If not specified, you'll be prompted.
-   [buffer size]: (optional) size for the internal buffer (in kB). Only required if -m is used.

![img](https://www.gdgsoft.com/gsplit/help/bullet1.png)  **GUnite exit codes:**

GUnite always returns an exit code when it closes. You can use this exit code in batch files to know whether a file was successfully restored or not.

**Exit Code**

**Meaning**

0

No error, file successfully restored

1

Uniting not started or failed before completion

2

Uniting completed but file not correctly restored

![img](https://www.gdgsoft.com/gsplit/help/bullet1.png)  **Examples:**

-   Restore the testout.exe file and exit:  
    GUNITE.EXE "D:\GSplitPieces\disk1.gsd" -u "D:\testout.exe"
-   Set the memory buffer's size to 128 kb.  
    GUNITE.EXE "" -m "" 128

### 4. Adding predefined values for piece size

If you create  [blocked pieces](https://www.gdgsoft.com/gsplit/help/Blocked_and_Spanned_Disk_Pieces.htm), you may have to enter a specific size for your pieces or choose a predefined value from the size list by clicking Predefined. GSplit reads all default values from a file called  **GDEFVAL.INF**  and available in the GSplit's main folder. This file is in text format and can be edited with Notepad. But it has its own syntax:

The first line (identification header) must be: "_[GSplit Default Size Values]_". Then follow the default values. One value per line.

![img](https://www.gdgsoft.com/gsplit/help/bullet1.png)  To add a default value, you should use the following format:

NAME;DESCRIPTION;NUMBER OF BYTES  
  
where

-   NAME is a general name like "1.44 MB"
-   DESCRIPTION is an optional description like "3.5''HD". (enter None if no description).
-   NUMBER OF BYTES is the final size that will be used if the end user selects this default value (positive integer, like 1457664).

This becomes: 1.44 MB;3.5''HD;1457664  
  
![img](https://www.gdgsoft.com/gsplit/help/bullet1.png)  You can add or remove default values. Please be sure that the first line remains unmodified. If this file is not found, GSplit will still continue to work but with an empty list.

