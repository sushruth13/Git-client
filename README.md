# Git-client
A cli-based git-client created in rust as a part of a class project

Compilation instructions and commands:

For initialising the git repository: cargo run init
For adding a file to staging: cargo run add <file-path/file-name>
For commiting a staged file: cargo run commit
For creating a clone repository: cargo run clone

Please note that before adding the file to the staging area, check that the folder ".GitClient/objects" is empty. To test this:
1. cd .GitClient/objects
2. ls
3. Create a new file "new.txt": touch new.txt
4. cargo run add new.txt
5. A folder containing the blob file will be generated.
6. Check the contents of the blob:

To test commit command:
1.

The clone currently does not support private repository.
