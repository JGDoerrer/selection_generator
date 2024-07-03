# !/bin/bash

mv src/algorithm_test/algorithm.rs src/algorithm_test/algorithm_old.rs

for file in $(ls -1 "algorithms" | sort); do
  filename=$(basename -- "$file")
  n=$(echo "$filename" | cut -d'_' -f1)
  i=$(echo "$filename" | cut -d'_' -f2 | cut -d'.' -f1)

  echo $filename, $n, $i

  cp algorithms/"$filename" src/algorithm_test/algorithm.rs

  cargo test algorithm_test --release > "$filename"_test.txt

  if [ $? -ne 0 ]; then
    echo "Error: cargo run failed. Exiting."
    break;
  fi
done

mv src/algorithm_test/algorithm_old.rs src/algorithm_test/algorithm.rs