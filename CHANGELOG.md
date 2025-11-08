# Changelog

## [1.0.1]
- Update comments and README.md.
- Rearrange code.

## [1.0.0]
- `use_args_for_case_name` is introduced. With this option, you can
  specify `p_test` to generate test names with their arguments.
- Test case name should be specified before the arguments tuple.
- Explicit module name specification is removed.

## [0.1.8]
Support literal string for test name and test case name.

## [0.1.7]
Internal changes:
- Simplified the macro parsing logic.
- Removed unnecessary code.
- Added tests.

## [0.1.6]
Test case name is optional. When case name is skipped, case name will be generated in `case_{n}` format.

## [0.1.5]
Support new format.

## [0.1.4]
Clean up code.

## [0.1.3]
Module name is optional.

## [0.1.2]
Added doc, and small touches in Cargo.toml.

## [0.1.1]
Test module name, which used to be hard-corded as `tests`, can be specified.

## [0.1.0]
Initial release of p-test.
