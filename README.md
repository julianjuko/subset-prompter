# subset-prompter
Query unique subsets of large datasets - quickly.

This program is designed to parse through large JSON blobs stored in a CSV file and return unique values based on a user-specified data path. It is built with Rust, ensuring it performs this task in a memory-efficient and fast manner.

## How to Use

Follow the steps below to use this program:

1. **Filepath Input**: The program will first prompt you for a filepath relative to the current working directory. This filepath should point to a CSV file containing one column of large serialized JSON blobs.

2. **Data Path Input**: Next, you will be asked for a data path which serves as an instruction to navigate through the nested JSON data structure. For example: `key.childKey..childKey2.childKey3`.

## Data Path Syntax

The syntax for specifying the data path is important:

- A single period `.` indicates a direct parent-child relationship between keys.
- Two periods `..` signify an abstraction level which could be either an array or an object (similar to `Record<string, object>` type in TypeScript). In such cases, all corresponding values are consolidated at that level as part of the bottom-level subset.

For instance, given the following JSON object:

```json
{
  "id": 2,
  "cargo": [
    {
      "items": {
        "125252": {
          "name": "Gel Blaster"
        },
        "125253": {
          "name": "Jetpack"
        },
        "125254": {
          "name": "Pincushion"
        }
      }
    },
    {
      "items": {
        "125252": {
          "name": "Gel Blaster"
        },
        "125253": {
          "name": "Pogo Stick"
        },
      }
    }
  ]
}
```
A data query `id..items..name` will return:

```
"Gel Blaster"
"Jetpack"
"Pincushion"
"Pogo Stick"
```

## Output 

The resulting subset of unique values based on your specified data path will be printed directly to your terminal.
