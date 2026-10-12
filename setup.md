# Run and Test 0-Shell

Follow these steps in order. There are two prompts:

- **Normal terminal** looks like `s7@S7:/some/path$`.
- **0-Shell** looks like `/some/path $`.

Commands in the first section go in the normal terminal. Commands in the
second section go only after 0-Shell starts.

## 1. Build and prepare test files in the normal terminal

Copy and run this block in your normal WSL terminal:

```sh
cd /mnt/c/Projects/0-shell
cargo test
cargo build
mkdir -p /tmp/zero-shell-audit/audit_parent/child_one
mkdir -p /tmp/zero-shell-audit/audit_parent/child_two
mkdir -p /tmp/zero-shell-audit/new_folder1
mkdir -p /tmp/zero-shell-audit/new_folder2
printf 'Hello from the audit test file!\n' > /tmp/zero-shell-audit/new_folder1/new_doc.txt
cd /tmp/zero-shell-audit
/mnt/c/Projects/0-shell/target/debug/0-shell
```

The `printf` line creates the sample file using the **normal terminal**, before
0-Shell starts. The shell itself does not implement a text editor or output
redirection. Do not type the file path by itself; that attempts to run it as a
program.

## 2. Test commands inside 0-Shell

You should now see a prompt like `/tmp/zero-shell-audit $`. Copy and run these
commands one at a time at that prompt:

```text
pwd
echo "something!"
echo something else
ls
ls -l -a -F
cd audit_parent
pwd
cd ..
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

Expected checks:

- `pwd` initially prints `/tmp/zero-shell-audit`.
- `echo` prints the text entered.
- `audit_parent` contains `child_one` and `child_two`; `pwd` prints its path
  after `cd audit_parent`.
- Both `cat` commands print `Hello from the audit test file!`.
- After `mv`, the listing for `new_folder1` shows `new_doc.txt` and
  `new_folder2/`.
- After `rm -r new_folder1` and `rm -r audit_parent`, the final `ls` no longer
  shows either test folder.
- `cd` with no argument changes to your home directory; `pwd` shows it.
- `exit` returns to the normal terminal.

All test files and folders are created under `/tmp/zero-shell-audit`, outside
the project directory.

## Notes

- If your project is not at `/mnt/c/Projects/0-shell`, change the path to
  `target/debug/0-shell` in the launch command.
- `ls -l` shows permissions, numeric user/group IDs, size, modification time,
  and other long-listing information. Formatting and numeric IDs can differ
  from your system's `ls`.
- Ctrl+D also exits 0-Shell by sending end-of-file.
