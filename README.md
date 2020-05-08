# ZPM
[![CircleCI](https://circleci.com/gh/Shard-Labs/zpm.svg?style=shield)](https://circleci.com/gh/Shard-Labs/zpm)

Navigate to official [ZoKrates](https://github.com/Zokrates/ZoKrates) repository and follow the instructions on how to install ZoKrates.

Before running this tool, make sure zokrates is added to PATH:
```bash
export PATH=$PATH:$HOME/.zokrates/bin
export ZOKRATES_HOME=$HOME/.zokrates/stdlib
```

## Commands

### create
Creates new project

Arguments:
* `<name>` - Sets the project name
* `<path>` - Sets the project path \[default: .\]

```
zpm create <name> <path>
```

### compile
Compiles the code into flattened conditions

Artifacts:
* `target/out`
* `target/out.ztf`
```
zpm compile
```

### setup
Performs a trusted setup for a given constraint system

Artifacts:
* `target/verification.key`
* `target/proving.key`
```
zpm setup
```

### compute
Calculates a witness for a given constraint system

Arguments:
* `-a, --arguments` - Arguments for the compiled program

Artifacts:
* `target/witness`

Compute with inline arguments:
```
zpm compute -a 1 2
```

Or pass arguments to the stdin as json:
```
echo [\"1\",\"2\"] | zpm compute --stdin

# inputs.json -> ["1","2"]
cat inputs.json | zpm compute --stdin
```

### export-verifier
Exports a verifier as Solidity smart contract

Artifacts:
* `target/verifier.sol`
```
zpm export-verifier
```

### generate-proof
Calculates a proof for a given constraint system and witness

Artifacts:
* `target/proof.json`
```
zpm generate-proof
```

### clean
Cleans target directory
```
zpm clean
```

## ZoKrates Configuration File (zcf)

```
# config.zcf

[general]
name = 'example' # project name
entry = 'main.zok' # entry point

[crypto]
elliptic_curve = 'bn128' # elliptic curve to use
proving_scheme = 'g16' # verifiable computation scheme
```

## Improvements
1. Instability caused by different versions - By default, zpm uses zokrates binary from the `PATH` env variable to execute the commands. In case zokrates cli changes, zpm needs to be updated accordingly. One way to solve this it to have zpm manage the versions of zokrates instead of using user installed version of zokrates.
3. Add silent flag for silencing zokrates stdin/stderr.
5. `zpm create` should cleanup if something fails.
6. Add a test case if curve or proof system changes halfway through.


## Development
Anyone is welcome to help progress and improve this utility. Tasks and issues can be found in the issues tab. If your problem/task is not in the tasks, feel free to create a new issue explaining your problem/task.
