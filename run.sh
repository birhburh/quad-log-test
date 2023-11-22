#!/bin/bash
xdg-open ./target/android-artifacts/release/apk/quad-log-test.apk
read
am start --user 0 -n rust.quad_log_test/.MainActivity
am start --user 0 -n rust.quad_log_test.MainActivity
