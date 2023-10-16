#!/bin/sh

echo "Bootstrapping container"

export GIT_SSH_COMMAND="ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no"

## Don't ask for confirmation on first operation with git server
export GIT_SERVER_IP=git_server
ssh-keyscan -H $GIT_SERVER_IP >> ~/.ssh/known_hosts


eval $(ssh-agent -s)
ssh-add /root/.ssh/id_rsa
git config --global user.email "app@test"

echo "Container ready for testing"


## git folder without diff
# git test
mkdir /tmp/git-without-diff
cd /tmp/git-without-diff
git init
touch init.txt
echo "test" >> init.txt
git add .
git commit -m "initial commit"
git remote add origin git@git_server:/tmp/git-without-diff.git
git branch -m main
git push origin main
git checkout -b featureA
git push origin featureA



## git folder with diff
mkdir /tmp/git-with-diff
cd /tmp/git-with-diff
git init
echo "hello world" >> init.txt
git add .
git commit -m "initial commit"
git remote add origin git@git_server:/tmp/git-with-diff.git
git branch -m main
git push origin main

git checkout -b featureA
echo "hello git" >> init.txt
git add .
git commit -m "add hello git"
git push origin featureA


# git folder with diff on merge commit
mkdir /tmp/git-with-diff-on-merge
cd /tmp/git-with-diff-on-merge
git init
echo "hello world" >> init.txt
git add .
git commit -m "initial commit"
git remote add upstream git@git_server:/tmp/git-with-diff-on-merge.git
git branch -m main
git push upstream main

git checkout -b featureA
echo "hello git" >> init.txt
git add .
git commit -m "add hello git"
git push upstream featureA

git checkout main
git merge featureA
git branch -D featureA

git checkout -b featureB
echo "hello featureB" >> init.txt
git add .
git commit -m "add featureB"
git push upstream featureB


