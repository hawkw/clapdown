
# `$ `**`git`**

A fictional versioning CLI


```text
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

 - **[`clone`](#$--git-clone)**: Clones repos
 - **[`diff`](#$--git-diff)**: Compare two commits
 - **[`push`](#$--git-push)**: pushes things
 - **[`add`](#$--git-add)**: adds things
 - **[`stash`](#$--git-stash)**

## `$ git `**`clone`**

Clones repos


```text
Usage: clone <REMOTE>
```


### arguments

 -  `<REMOTE>`: The remote to clone


## `$ git `**`diff`**

Compare two commits


```text
Usage: diff [OPTIONS] [COMMIT] [COMMIT] [-- <PATH>]
```


### options

 - `--color` `[=<WHEN>]`
    - **default:** `auto` 
    - **possible values:** 
      - `always` 
      - `auto` 
      - `never` 



### arguments

 -  `[COMMIT]`
 -  `[COMMIT]`
 -  `[PATH]`


## `$ git `**`push`**

pushes things


```text
Usage: push <REMOTE>
```


### arguments

 -  `<REMOTE>`: The remote to target


## `$ git `**`add`**

adds things


```text
Usage: add <PATH>...
```


### arguments

 -  `<PATH>...`: Stuff to add


## `$ git `**`stash`**


```text
Usage: stash [OPTIONS]
       stash push [OPTIONS]
       stash pop [STASH]
       stash apply [STASH]
```

### subcommands

 - **[`push`](#$-git--stash-push)**
 - **[`pop`](#$-git--stash-pop)**
 - **[`apply`](#$-git--stash-apply)**

### options

 - `-m`, `--message` `<MESSAGE>`


### `$ git stash `**`push`**


```text
Usage: push [OPTIONS]
```


#### options

 - `-m`, `--message` `<MESSAGE>`


### `$ git stash `**`pop`**


```text
Usage: pop [STASH]
```


#### arguments

 -  `[STASH]`


### `$ git stash `**`apply`**


```text
Usage: apply [STASH]
```


#### arguments

 -  `[STASH]`


