test dir:
  #!/usr/bin/env bash
  dir={{ dir }}
  [[ "all" -ne "$dir" ]] && { just $dir/test; exit;}

  for d in ./day_*; do
    echo "/******************************************/"
    echo $d
    just $d/test
    echo "/******************************************/"
    echo
    echo
  done;
