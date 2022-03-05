# Git-client
A cli-based git-client created in rust as a part of secure programming course project

Compilation instructions and commands:
1) Download and extract the repositary
2) Use: **docker build -t gitclient .**
3) Use: **docker images** and copy the gitclient image ID
4) Use: **docker run -it [imageID]**
5) Use: **Cargo build**
6) For initialising the git repository: **cargo run init**
7) For adding a file to staging: **cargo run add <file-path/file-name>**
8) For commiting a staged file: **cargo run commit**
9) For creating a clone repository: **cargo run clone**

To verify the commands:
1) Verify git init
You should be able to see .GitClient subdirectory inside the container

2) Verify git add:
Before executing the add command, **cd .GitClient/objects** and **ll**, notice that there are no objects created yet
Create a new file "new.txt": **touch new.txt**
**cargo run add new.txt**
Got to objects directory agaib, and you can see that the blob file is generated for the created file (new.txt)

3) Verify commit:
You will be prompted the blob hashed IDs and file names on command prompt

4) Verify clone:

