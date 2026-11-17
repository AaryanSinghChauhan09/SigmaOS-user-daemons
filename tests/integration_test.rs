// Integration test for SigmaOS-user-daemons
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use user_daemons as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
