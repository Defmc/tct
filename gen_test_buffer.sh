./target/release/tct cat lipsum >> test
for i in $(seq $1); do
    ./target/release/tct cat test >> test
    printf "%s (%s)\n" "$(exa --long test)" "$i"
done
