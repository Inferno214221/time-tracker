# Invoice Generator (Rename Pending)

A CLI tool to log time and generate invoices / timesheets for software development.

This tool is pretty specific to my own workflow and time tracking format, but I'm putting it on GitHub anyway.

## Features

- CLI interface with `clap`
- ORM using `diesel`
- Invoice generation using `typst`
  - (Usage is heavily based on [`typst-as-library`](https://github.com/tfachmann/typst-as-library))
- Timesheet generation using `csv`