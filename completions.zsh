#!/usr/bin/env zsh

#compdef _stamp stamp
function _stamp() {
  local line

  _arguments -C \
    "-h[Show help information]" \
    "--h[Show help information]" \
    "1: :(list run)" \
    "*::arg:->args"

  case $line[1] in
  list)
    _stamp_list
    ;;
  run)
    _stamp_run
    ;;
  esac
}

function _stamp_list() {
  _arguments \
    "--silent[Dont output anything]"
}

function _stamp_run() {
  _arguments \
    "--repeat=[Repat the <message> any number of times]"
}
