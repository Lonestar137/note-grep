#!/usr/bin/env -S nu --stdin

# TODO: Tools, read from config toml.
alias editor = helix
alias file_viewer = less



let base_path = $"($env.HOME)/.cache/note-grep"
let filename = date now | format date "%m-%d-%y" | $"($in).yml"
let note_file = $"($base_path)/($filename)"

def "main ntil" [] {
  if ($note_file | path exists) {
    editor $note_file
  } else {
    mkdir $base_path
    touch $note_file
    editor $note_file
  }
}

def "main til" [] {
  ranger $base_path
}

def main [query: string] {
  if ($base_path | path exists) {
    let relevant_note_blocks = ls $base_path | where type == "file" |  where name =~ 'yml$' | each {
      open $in.name --raw | split row "---:" | filter { $query in $in }
    }

    if not ($relevant_note_blocks | is-empty) {
      let note_str = $relevant_note_blocks | str join '---'
      $note_str | file_viewer
      return $note_str
      # filter { $query in $in }
    }
  } else {
    print "No notes to scan."
    return "No notes to scan."
  }
}
