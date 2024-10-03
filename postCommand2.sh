#!/usr/bin/expect -f

expect -c "
set timeout 3
spawn curl -fsSL https://internetcomputer.org/install.sh

expect 'Proceed with installation'
send '\n'
interact
"

