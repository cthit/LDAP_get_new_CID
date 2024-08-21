# Three simple scripts to get CID of new students.

## Usage
1. Get a CSV with the format Firstname,Lastname.
2. Run `./ldap\_parser\_names.sh {filewithnames.csv} >> potential\_new\_students`.
3. Run `./ldap\_verify\_cid.sh potential\_new\_students >> verified\_new\_students`.
4. Run `cargo run` and input `verified\_new\_students`.

## Dependencies
- OpenLDAP
- Rust
- Cargo

## Disclaimer
This is not 100% guaranteed to get ALL new students, you need a list provided by e.g. NollKIT or Programledningen. This script also completely gives up when it finds several cid:s for a Firstname Lastname combo.
