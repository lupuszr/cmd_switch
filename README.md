# cmd_switch

## Overview
`cmd_switch` is a versatile tool designed to manage and execute tasks dynamically based on a pre-defined JSON configuration. It allows users to easily switch between different commands, making it ideal for scenarios requiring multiple or conditional command executions.

## Features
- **Load JSON Configurations:** Effortlessly load configurations defined in a JSON file, specifying labels and commands to execute.
- **Grouping Commands:** Organize related tasks into groups for better management and clarity.
- **Simple Command Line Interface:** Use a straightforward CLI to manage and execute tasks quickly.

## Configuration
`cmd_switch` uses a JSON file to define tasks. Each task must specify a `label` and a `cmd`. Tasks can be grouped under a `group` label for better organization.

### Example JSON Configuration
```json
[
    {
        "label": "Display Current Directory",
        "cmd": "pwd"
    },
    {
        "label": "List All Files",
        "cmd": "ls -a"
    },
    {
        "group": "File Operations",
        "config": [
            {
                "label": "Create New File",
                "cmd": "touch newfile.txt"
            },
            {
                "label": "Write to File",
                "cmd": "echo 'Hello, World!' > newfile.txt"
            },
            {
                "label": "Read File",
                "cmd": "cat newfile.txt"
            }
        ]
    },
    {
        "group": "System Info",
        "config": [
            {
                "label": "Check Disk Usage",
                "cmd": "df -h"
            },
            {
                "label": "Memory Usage",
                "cmd": "free -h"
            }
        ]
    },
    {
        "label": "Echo Custom Message",
        "cmd": "echo 'This is a custom message.'"
    }
]
```

This configuration defines commands for displaying directory information, listing files, grouped file operations, system information checks, and custom messaging.

## Installation
To use cmd_switch, clone the repository or download the latest release and build it with cargo build --release.


## Usage
To execute commands with cmd_switch, you need to specify the path to your JSON configuration file using the --config-path argument.

```sh
  cmd_switch --config-path example.json
```
