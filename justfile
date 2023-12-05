dev day:
    cargo watch -q -c -w ./day-{{day}}/src -x "test -p day-{{day}} -q"

create day:
    cp -r ./starter ./day-{{day}}
    sed -i "s/day-/day-{{day}}/" ./day-{{day}}/Cargo.toml

solve day part:
    cargo run -p day-{{day}} --release --bin part-{{part}}