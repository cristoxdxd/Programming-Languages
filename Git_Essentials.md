# Git & GitHub course
## List of **Commands for _Terminal_**

|Command|Function|
|---|---|
|clear|To clear the terminal|
|ls|List information about the **files** (current directory by default)|
|cat |Show files content|
|cd|Change directory|
|history|display the history list|
|touch|Create a new file|
|nano|Access the _'nano'_ editor|

## Git 
> **Git** is a free and open source, fast, scalable and distributed version control system

You could view the latest Git documentation at:
* [Git Manual](https://git.github.io/htmldocs/git.html)
* [Git Documentation](https://git-scm.com/doc)

---
When you `init` a repository you have a hint directory 
## ___.git___
* Directories
    * branches
    * hooks
    * info
    * objects
    * refs
* Files
    * config
    * description
    * HEAD

## Git commands
This section list basic Git commands

|Command|Function|
|---|---|
|git init|Create an empty Git repository or reinitialize an existing one|
|git config|Get an set repository (adding *-local*) or global options|
|git status|Show the working tree status|
|git add|Add file contents to the _Staging Area_/**Index** (there you can give follow to this files)|
|git commit -m `"Comment for the commit"`|Record changes to the repository|
|git log|Show commit logs|
|git log --oneline|Show compact version of commit logs|

## Branching
This section is about branching commands for Git

|Command|Function|
|---|---|
|git branch|To list branches|
|git branch `<branch>`|To create a branch|
|git -d `<branch>`|To delete a branch without commits _(soft)_|
|git -D `<branch>`|To delete a branch _( You could lose information of commits not merged)_|
|git checkout `<branch>`|To navigate from branches|
|git diff `<current_branch> <other_branch>`|Show changes between commits (another branch -> actual branch)|
|git merge `<branch>`|Join two or more development histories together|
|git log --graph|Show commit logs with merge of branches|

## SSH

Steps to create a SSH key for your repositories:

1. Open Git bash Terminal
2. Write the command
```
$ ssh-keygen -t id25519 -C "your@email.com"
```
or 
```
$ ssh-keygen -t rsa -b 4096 -C "your@email.com"
```
- When you're prompted to "Enter a file in which to save the key," press Enter. This accepts the default file location.
- Then you have to type a secure passphrase.

3. Ensure ssh-agent
```
$ eval "$(ssh-agent -s)"
```
4. Add your SSH private key to the ssh-agent
```
$ ssh-add ~/.ssh/id_name
```
5. Add the SSH key to your account on GitHub

## GPG
This section is about GPG keys commands for sign from local `git clone` repository to the `'origin'`

|Command|Function|
|---|---|
|gpg --armor --export `<id_GPGkey>`||
|git commit -S||
|||

## GitHub CLI
|Command|Function|
|---|---|
|gh auth login|Login to your GitHub accout|
|gh repo list `<repository>`|List |
|gh repo create `<new_repository>`||
