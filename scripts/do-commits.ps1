cd "D:\Kaiyo\Project\xVora"

function git-commit-simple($files, $msg) {
    foreach ($f in $files) { Remove-Item .git\index.lock -Force -ErrorAction SilentlyContinue }
    git add @files
    Start-Sleep -Milliseconds 500
    git commit -m $msg
    Start-Sleep -Seconds 2
}

# 1. i18n.rs core engine
git-commit-simple @("crates/codegen/xvora-pager/src/i18n.rs") "feat(i18n): rewrite stub into full locale engine with En and Vi support"

# 2. lib.rs pub mod
git-commit-simple @("crates/codegen/xvora-pager/src/lib.rs") "chore: expose i18n module publicly"

# 3. settings/registry.rs
git-commit-simple @("crates/codegen/xvora-pager/src/settings/registry.rs") "fix(settings): wire language setting to live locale"

# 4. actions.rs
git-commit-simple @("crates/codegen/xvora-pager/src/app/actions.rs") "feat(actions): add SetLocale action"

# 5. setters.rs
git-commit-simple @("crates/codegen/xvora-pager/src/app/dispatch/settings/setters.rs") "feat(settings): add set_locale setter"

# 6. ui.rs dispatch
git-commit-simple @("crates/codegen/xvora-pager/src/app/dispatch/settings/ui.rs") "feat(dispatch): wire language to SetLocale"

# 7. router.rs
git-commit-simple @("crates/codegen/xvora-pager/src/app/dispatch/router.rs") "feat(router): route SetLocale to handler"

# 8. settings modal
git-commit-simple @("crates/codegen/xvora-pager/src/views/settings_modal/state.rs","crates/codegen/xvora-pager/src/views/settings_modal/render.rs","crates/codegen/xvora-pager/src/views/settings_modal/mod.rs","crates/codegen/xvora-pager/src/views/settings_modal/tests.rs") "feat(ui): i18n-aware settings modal title"

# 9. modal.rs
git-commit-simple @("crates/codegen/xvora-pager/src/views/modal.rs") "fix(modal): use i18n modal_title function"

# 10. main.rs
git-commit-simple @("crates/codegen/xvora-pager-bin/src/main.rs") "feat(i18n): init locale from env at startup"

# 11. AGENTS.md
git-commit-simple @("AGENTS.md") "docs: add AGENTS.md with project build notes"

# 12. scripts
git-commit-simple @("scripts/replace_grok_in_docs.py") "scripts: add grok-to-xvora rename utility"
git-commit-simple @("scripts/fix_grok_cmds.py") "scripts: add fix for remaining grok references"

# 13. docs rename + new file
git-commit-simple @("crates/codegen/xvora-pager/docs/user-guide/README.md","crates/codegen/xvora-pager/docs/user-guide/27-xvora-clone.md") "docs: add xvora-clone doc and update README"
git rm crates/codegen/xvora-pager/docs/user-guide/27-grok-clone.md 2>$null; git commit -m "docs: remove old grok-clone file"

# 14-29. individual docs
$docs = @("01-getting-started.md","02-authentication.md","03-keyboard-shortcuts.md","04-slash-commands.md","05-configuration.md","06-theming.md","07-mcp-servers.md","08-skills.md","09-plugins.md","10-hooks.md","11-custom-models.md","12-project-rules.md","13-memory.md","14-headless-mode.md","15-agent-mode.md","16-subagents.md","17-sessions.md","18-sandbox.md","19-plan-mode.md","20-background-tasks.md","21-terminal-support.md","22-permissions-and-safety.md","23-dashboard.md","24-monitoring-usage.md","25-status-line.md","26-config-reference.md")
foreach ($d in $docs) {
    git-commit-simple @("crates/codegen/xvora-pager/docs/user-guide/$d") "docs: rename Grok references to xVora in $d"
}

# 30. CHANGELOG
git-commit-simple @("CHANGELOG.md") "chore: update CHANGELOG with i18n and rename entries"

# 31. changelog json
git-commit-simple @("changelogs/CURRENT.external.json") "chore: update changelog JSON"

Write-Host "Done. Total commits:"
git log --oneline | Measure-Object | Select-Object -ExpandProperty Count
