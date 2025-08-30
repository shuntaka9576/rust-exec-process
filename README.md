
```bash
$ pstree 74699
-+= 74699 shuntaka npm exec @anthropic-ai/claude-code@latest --dangerously-skip-permissions --verbose --output-format=stream-json
 \-+- 74725 shuntaka claude
   |--- 74759 shuntaka <defunct>
   |--- 74783 shuntaka <defunct>
   |-+- 74786 shuntaka /etc/profiles/per-user/shuntaka/bin/zsh -c -l SNAPSHOT_FILE=/Users/shuntaka/.config/claude/shell-snapshots/snapshot-zsh-1756599324687-yiekxs.sh\012      source "/Users/shuntak
   | \--- 74790 shuntaka stty -ixon
   |--- 74788 shuntaka /Users/shuntaka/.npm/_npx/97540b0888a2deac/node_modules/@anthropic-ai/claude-code/vendor/ripgrep/arm64-darwin/rg --version
   \--- 74789 shuntaka /Users/shuntaka/.npm/_npx/97540b0888a2deac/node_modules/@anthropic-ai/claude-code/vendor/ripgrep/arm64-darwin/rg --files --hidden /Users/shuntaka/repos/github.com/shuntaka9576

~/repos/github.com/shuntaka9576/rust-exec-process main*
$  kill -CONT 74725 74699
```
