#!/usr/bin/env bash

# Author: Arda Sevinç   <ardasevinc.technical@gmail.com>
# 2020-01-09


main () {
    # Shell default parameter expansion
    echo "One for ${1:-you}, one for me."
    exit 0
}

main "$@"
