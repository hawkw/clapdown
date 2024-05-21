
# git

A fictional versioning CLI


```
Usage: git clone <REMOTE>
       git diff [OPTIONS] [COMMIT] [COMMIT] [-- <PATH>]
       git push <REMOTE>
       git add <PATH>...
       git stash [OPTIONS]
       git stash push [OPTIONS]
       git stash pop [STASH]
       git stash apply [STASH]
```

## subcommands

 - **[clone](#git-clone)**: Clones repos
 - **[diff](#git-diff)**: Compare two commits
 - **[push](#git-push)**: pushes things
 - **[add](#git-add)**: adds things
 - **[stash](#git-stash)**

## git clone

Clones repos


```
Usage: clone <REMOTE>
```


### arguments

 -  `<REMOTE>`: The remote to clone


## git diff

Compare two commits


```
Usage: diff [OPTIONS] [COMMIT] [COMMIT] [-- <PATH>]
```


### arguments

 -  `[COMMIT]`
 -  `[COMMIT]`
 -  `[PATH]`


### options

 - `--color` `[=<WHEN>]`
    - **default:** `auto` 
    - **possible values:** 
      - `always` 
      - `auto` 
      - `never` 



## git push

pushes things


```
Usage: push <REMOTE>
```


### arguments

 -  `<REMOTE>`: The remote to target


## git add

adds things


```
Usage: add <PATH>...
```


### arguments

 -  `<PATH>...`: Stuff to add


## git stash


```
Usage: stash [OPTIONS]
       stash push [OPTIONS]
       stash pop [STASH]
       stash apply [STASH]
```

### subcommands

 - **[push](#stash-push)**
 - **[pop](#stash-pop)**
 - **[apply](#stash-apply)**

### options

 - `-m`, `--message` `<MESSAGE>`


### git stash push


```
Usage: push [OPTIONS]
```


#### options

 - `-m`, `--message` `<MESSAGE>`


### git stash pop


```
Usage: pop [STASH]
```


#### arguments

 -  `[STASH]`


### git stash apply


```
Usage: apply [STASH]
```


#### arguments

 -  `[STASH]`


