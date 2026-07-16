from pathlib import Path
import re

p = Path("crates/codegen/xvora-pager/src/i18n/slash.rs")
t = p.read_text(encoding="utf-8")
t = t.replace(
    "pub fn description(name: &str, fallback: &'static str) -> &'static str {",
    "pub fn description(name: &str) -> Option<&'static str> {",
)
t = t.replace("        _ => fallback,", "        _ => None,")
# Wrap string returns in Some(...)
t = re.sub(r'=> "((?:\\.|[^"\\])*)",', r'=> Some("\1"),', t)
p.write_text(t, encoding="utf-8")
print(p.read_text(encoding="utf-8")[:600])
