# Auditor Setup and Test Guide

This guide provides a repeatable way to build 0-Shell and demonstrate the
required commands without relying on shell redirection or an editor builtin
inside the program.

## Prerequisites

- Rust and Cargo installed (`rustc --version` and `cargo --version`).
- Linux or WSL is recommended for the Unix-style permissions shown by `ls -l`.
- Open a host terminal in the project directory.

## Build and run automated tests

From the project directory:

```sh
cargo test
cargo build
```

The tests cover parsing and file operations; the build creates
`target/debug/0-shell`.

## Start the shell

Build from the project directory, then run the binary from a dedicated
temporary test directory so audit files stay out of the repository:

```sh
cargo build
mkdir -p /tmp/zero-shell-audit
cd /tmp/zero-shell-audit
/mnt/c/Projects/0-shell/target/debug/0-shell
```

The prompt shows the current directory followed by `$`. Enter one command per
line and press Enter. The shell implements its supported commands itself and
does not launch external commands. `mkdir` above is run by the host terminal
before starting 0-Shell; it is not executed by the project.

## Manual audit walkthrough

Run these commands at the 0-Shell prompt:

```text
pwd
echo "something!"
echo something else
mkdir audit_parent/child_one audit_parent/child_two
cd audit_parent
pwd
cd ..
mkdir new_folder1 new_folder2
ls -l -a -F
exit
```

To test `cp`, `cat`, `mv`, and `rm`, create a file named `new_doc.txt` with a
text editor at `/tmp/zero-shell-audit/new_folder1/new_doc.txt`. The editor is
used outside 0-Shell because this project does not implement a text editor or
output redirection. After saving the file, return to the host terminal, change
back to `/tmp/zero-shell-audit`, and launch
`/mnt/c/Projects/0-shell/target/debug/0-shell` again.

Continue at the 0-Shell prompt:

```text
cp new_folder1/new_doc.txt new_folder2
cat new_folder1/new_doc.txt
cat new_folder2/new_doc.txt
mv new_folder2 new_folder1
ls -l -a -F new_folder1
rm -r new_folder1
ls -l -a -F
rm -r audit_parent
cd
pwd
exit
```

Check that both `cat` commands display the text, `ls` shows the moved
`new_folder2` inside `new_folder1`, and the final listing no longer shows
`new_folder1`. The `cd` command without an argument should change to the
directory in `HOME`; `pwd` displays the resulting path. If you need to resume
the audit after restarting the shell, launch it from the same test directory.
All test folders are under `/tmp/zero-shell-audit`; the walkthrough does not
create or remove test folders in the repository.

## Notes about output

- `ls -a` includes `.` and `..` as well as hidden entries.
- `ls -l` shows type and permissions, link count, numeric user/group IDs,
  size, modification time, and name. Timestamps are UTC and IDs are numeric,
  so exact columns can differ from the host's `ls`.
- Ctrl+D exits by sending end-of-file to the shell.
- `exit` returns to the terminal that launched the shell.
