# JIRA-CLI

This is (or at least is going to be) a CLI client for JIRA. The idea is to let
the user see all informations on tickets and also do searches, directly from the CLI.

## Installing

As of now, using this project requires Rust nightly. There's a chance it might
work on Stable, I just didn't test it yet. You can install it using `cargo
install`:

`cargo install --git https://github.com/cfcosta/jira-cli`

It works on public JIRAs, if having not oAuth support is a deal-breaker for
you, check back in a week or so, its high on the milestones list.

You are going to need a configuration file to use it, something like this:

`$HOME/.config/jira/config.toml`
```toml
[host]
  hostname = "https://domain.for.my.jira.org"
```

## LICENSE

Copyright 2017 Cainã Costa

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
