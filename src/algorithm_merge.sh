# !/bin/bash

folder="../algorithms"

for file in $(ls -1 "$folder" | sort); do
  filename=$(basename -- "$file")
  n=$(echo "$filename" | cut -d'_' -f1)
  i=$(echo "$filename" | cut -d'_' -f2 | cut -d'.' -f1)

  echo $filename, $n, $i

  rm -rf algorithm_test.rs
  cat algorithm_test_prefix.txt >> algorithm_test.rs
  echo "    const N: usize = $n;" >> algorithm_test.rs
  echo "    const I: usize = $i;" >> algorithm_test.rs
  cat ../algorithms/"$filename" >> algorithm_test.rs
  cat algorithm_test_suffix.txt >> algorithm_test.rs

  cargo test algorithm_test --release >

  if [ $? -ne 0 ]; then
    echo "Error: cargo run failed. Exiting."
    exit 1
  fi
done