# 0-Shell Audit Answers

## General

### Was the project written in Rust?

**Yes.** The project is a Rust package configured in `Cargo.toml`; the shell entry point is `src/main.rs`.

### Are the commands mentioned in the subject implemented from scratch, without calling any external binaries?

**Yes.** The required commands are implemented in Rust in `src/builtins/` and dispatched by `src/main.rs`. The source does not use process-spawning APIs such as `std::process::Command`, `spawn`, or `execve`.

### What happens when an unrecognized command is entered?

The shell prints the required message, `Command '<name>' not found`, and returns to the prompt.

## Functional

### Open a terminal and run the project. Can you confirm that the project runs and displays a unix shell?

**Yes.** Build and run it using the instructions in [setup.md](./setup.md). The program starts its own command prompt.

### Open a terminal and run the project. Can you confirm that this interpreter displays at least a simple `$` and waits for you to type a command?

**Yes.** It displays the current directory followed by `$` and waits for a line of input, for example `/path/to/0-shell $`.

### Try to run a command of your choice. Can you confirm that the interpreter only validates the command if you press enter?

**Yes.** Input is read one line at a time and parsed after Enter is pressed.

### Try to run the command `exit`. Can you confirm that the interpreter terminates properly and gives back the parent's shell?

**Yes.** `exit` returns from the shell loop, closing the program and returning control to the terminal that launched it.

### Try to run the command `echo "something!"`. Do the same in your computer terminal. Can you confirm that the displayed message of the project is exactly the same as the computer terminal?

**Yes.** It prints `something!` followed by a newline.

### Try to run the command `echo something else` (without double quotes). Do the same in your computer terminal. Can you confirm that the displayed message of the project is exactly the same as the computer terminal?

**Yes.** It prints `something else` followed by a newline.

### Try to run the command `pwd`. Can you confirm that the interpreter displayed the current path?

**Yes.** `pwd` prints the shell's current working directory.

### Try to open the project and create a parent folder with two children folders using the command `mkdir`. Then enter the parent folder and do `pwd`. Can you confirm that the interpreter displayed the current path?

**Yes.** `mkdir` accepts multiple directory arguments. After `cd parent` (or another path to the parent), `pwd` prints the resulting current path.

### Try to enter a directory of your choice by using the command `cd dir/of/your/choice`. Can you confirm that the interpreter took you to the correct path? Use `pwd` to confirm.

**Yes.** `cd` resolves relative paths from the current directory and accepts absolute paths. Confirm the result with `pwd`.

### Try to run only the command `cd`. Can you confirm that the interpreter took you to the users home folder? Use `pwd` to confirm.

**Yes, when the `HOME` environment variable is set.** With no argument, `cd` uses `HOME`; run `pwd` to confirm the destination.

### Try to run the command `ls` in a directory at your choice. Do the same in your computer terminal. Can you confirm that the output is similar in the project and in your computer terminal?

**Yes, for the directory entry names.** The shell lists and sorts visible entries. Its display layout is simpler than a system `ls`.

### Try to run the command `ls -l -a -F` in a directory at your choice. Do the same in your computer terminal. Can you confirm that the output is similar in the project and in your computer terminal?

**Mostly.** The flags are accepted: `-a` includes `.` and `..` and other hidden entries, `-F` marks directories, executable files, and symbolic links, and `-l` shows a total plus file type and permissions, link count, numeric user/group IDs, size, modification time, and name. The listing contains comparable information, but exact columns can differ because timestamps are UTC and ownership uses numeric IDs.

### Try to run the commands `mkdir new_folder1` and `mkdir new_folder2` in a directory of your choice. Can you confirm that the directories `new_folder1` and `new_folder2` were created?

**Yes.** `mkdir` creates each requested directory and reports an error if a requested creation fails.

### Create a document inside the `new_folder1` called `new_doc.txt` with some random text inside. Try to run the command `cp new_doc.txt ../folder2` to copy the document to the folder `new_folder2`. Can you confirm that the document `new_doc.txt` is inside the `new_folder2`?

**Yes, provided the destination directory already exists and the paths match your folder names.** Create the test file with a text editor before starting the shell (the shell does not include a text editor or output redirection). From inside `new_folder1`, run `cp new_doc.txt ../new_folder2`; the copied file should then be at `../new_folder2/new_doc.txt`.

### Try to run the command `cat new_folder1/new_doc`. Do the same in your computer terminal. Can you confirm that the output is the same in the project and in your computer terminal?

**Yes, when the supplied path names an existing text file.** `cat` prints the file contents. For the file created above, use `cat new_folder1/new_doc.txt` from the parent directory; the audit's example omits the `.txt` extension.

### Try to run the commands `mv new_folder2 new_folder1` to move the directory `new_folder2` inside of the directory `new_folder1`. Can you confirm that the directory `new_folder2` is inside of the directory `new_folder1`?

**Yes, when `new_folder1` exists and `new_folder2` is its sibling.** Run `mv new_folder2 new_folder1/`; it should result in `new_folder1/new_folder2`.

### Try to run the command `rm -r new_folder1` to remove what was created above. Can you confirm that the directory `new_folder1` was removed?

**Yes.** `rm -r new_folder1` recursively removes the directory and its contents. Confirm with `ls` or `pwd` from the parent directory.

## Audit note

The `ls -l` output includes the standard long-listing details using UTC timestamps and numeric user and group IDs. Exact formatting and owner/group names can differ from the host system's `ls`.

The shell also exits cleanly on end-of-file (Ctrl+D), in addition to the `exit` command.
