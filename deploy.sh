#!/bin/sh

setup_git() {
    git config --global user.email "travis@travis-ci.org"
    git config --global user.name "Travis CI"
}

commit_website_files() {
    git checkout -b egg-herbie-deploy-$TRAVIS_OS_NAME-temp
    git add -u
    git add target/release/* -f
    git add .travis.yml
    git commit --message "Travis build: $TRAVIS_BUILD_NUMBER"
}

upload_files() {
    git remote add origin-pages https://${GITHUB_TOKEN}@github.com/oflatt/egg-herbie > /dev/null 2>&1
    git fetch origin-pages egg-herbie-deploy-$TRAVIS_OS_NAME
    git checkout egg-herbie-deploy-$TRAVIS_OS_NAME
    git merge -s theirs egg-herbie-deploy-$TRAVIS_OS_NAME-temp
    git commit
    git push egg-herbie-deploy-$TRAVIS_OS_NAME
}

setup_git
commit_website_files
upload_files
