# Git-client
A cli-based git-client created in rust as a part of secure programming course project

Version 1: init, add, commit

version 2: authentication, clone, branch

Compilation instructions and commands:
1) Download and extract the repository
2) Use: **docker build -t gitclient .**
3) Use: **docker images** and copy the gitclient image ID
4) Use: **docker run -it [imageID]**
5) Use: **cargo build**
6) For authentication setup: **cargo run auth**, next **cd .ssh**, add the public key to your github account
7) For authentication verification: **cargo run check-auth**, enter "yes" on the prompt
8) For initialising the git repository: **cargo run init**
9) For adding a file to staging: **cargo run add <file-path/file-name>**
10) For commiting a staged file: **cargo run commit**
11) For creating a clone repository: **cargo run clone <git-repo-url>**
12) For displaying branch information: **cargo run branch <path_of_test-tmp>**
  
To verify the commands:
1) Verify git init:
You should be able to see .GitClient subdirectory inside the container

2) Verify git add:
Before executing the add command, **cd .GitClient/objects** and **ll**, notice that there are no objects created yet
Create a new file "new.txt": **touch new.txt**
**cargo run add new.txt**
Got to objects directory again, and you can see that the blob file is generated for the created file (new.txt)

3) Verify commit:
You will be prompted the blob hashed IDs and file names on command prompt

4) Verify clone:
The clone currently works for open repositories only. You can pass the link of open repository after "cargo run clone" to clone it. Please run **ll** after completion to check for the newly created repo in test-tmp.
  
5) Verify branch:
Currently we are storing cloned repository in test-tmp folder. On running "cargo run branch test-tmp", we get the names of branches created with the "git branch <new-branch-name>" command.
  
6) About Auth:
We have added the support for generating ssh-keys in the ED25519 format.The keys are generated without passphrases currently. We will look into adding this feature in our future version. The ssh keys generated by our client are stored in the default ssh folder ~/.ssh. Please note that the ssh keys should not be visible to other users in the system.

7) About Authorization verfication:
The keys generated by the client must be added to the users account on github.com. Upon successful addition of the ssh keys,the git client allows the user to verify the validity of the ssh keys. This is necessary for safe cloning of git repositories.
