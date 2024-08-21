# Three simple scripts to get CID of new students.

## Usage
1. Get a CSV with the format Firstname,Lastname.
2. Run `./ldap_parser_names.sh {filewithnames.csv} >> potential_new_students`.
3. Run `./ldap_verify_cid.sh potential_new_students >> verified_new_students`.
4. Run `cargo run` and input `verified_new_students`.

## Dependencies
- OpenLDAP
- Rust
- Cargo

## Disclaimer
This is not 100% guaranteed to get ALL new students, you need a list provided by e.g. NollKIT or Programledningen. This script also completely gives up when it finds several cid:s for a Firstname Lastname combo.
