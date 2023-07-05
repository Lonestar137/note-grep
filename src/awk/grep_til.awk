BEGIN { RS="\n---\n"; FS="\n"; ORS="\n---\n" }
$0 ~ pattern { 
  filename = file                      # Assign the full path to a variable
  gsub(/^.*\//, "", filename)          # Extract the filename without the path
  gsub(/\.[^.]+$/, "", filename)       # Remove the file extension
  print "Filename: " filename
  # Loop through each line in the section
  for (i=1; i<=NF; i++){
    # if the line matches the pattern, print the block
    if ($i ~ pattern) {
      print $0
      break
    }
  }
}
