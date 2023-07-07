# subset-prompter

Query unique subsets of large datasets - quickly.

This program is designed to parse through large JSON blobs stored in a CSV file and return unique values based on a user-specified data path. It is built with Rust, and uses multi-threading to perform this task in a memory-efficient and fast manner; even on CSV files that are several GB in size.

## How to Use

Run the program with `cargo build && cargo run`. It will then ask you for the following:

1. **Filepath Input**: The program will first prompt you for a filepath relative to the current working directory. This filepath should point to a CSV file containing one column of large serialized JSON blobs.

2. **Data Path Input**: Next, you will be asked for a data path which serves as an instruction to navigate through the nested JSON data structure. For example: `key.childKey..childKey2.childKey3`.

## Data Path Syntax

The syntax for specifying the data path is important:

- A single period `.` indicates a direct parent-child relationship between keys.
- Two or more periods such as `..` or `...` signify that there is one or more "collection" levels between the keys, which could be either an *array*, or an *object* / *map*. In such cases, all corresponding contents of this level are flattened, and the cursor passes through them to the next level.
- It is probably simpler to think of an "imaginary key" between consecutive periods `.` to denote a collection level for flattening, such that `key1...key2` is actually treated as `key1.<collection>.<collection>.key2`

For instance, given the following JSON object:

```json
{
  "id": 2,
  "shipment": [
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
        "125256": {
          "name": "Pogo Stick"
        }
      }
    }
  ]
}
```

A data query `shipment..items..name` will return:

```
"Gel Blaster"
"Jetpack"
"Pincushion"
"Pogo Stick"
```

## Output

The resulting subset of unique values based on your specified data path will be printed directly to your terminal. Knowing the expected shape of the data you are traversing will allow you to return sets of unique values from multi-layered JSON structures efficiently.

## Cases that have not been handled

- Keys which have a period `.` in them; these will be parsed as separate keys in the structure.
- Paths which end with a period `.`
