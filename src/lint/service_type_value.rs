use lint::*;
use systemd_parser::items::*;

static KNOWN_TYPES: &'static [&'static str] =
    &["simple", "forking", "oneshot", "dbus", "notify", "idle"];

pub fn lint(unit: &SystemdUnit) -> Result<(), LintResult> {
    if let Some(&DirectiveEntry::Solo(ref type_entry)) = unit.lookup_by_key("Type") {
        if let Some(val) = type_entry.value() {
            if !KNOWN_TYPES.contains(&val) {
                return Err(LintResult {
                    severity: LintSeverity::Error,
                    message: format!("Unknown type: {}", val),
                    code: LintCode::ErrorUnknownServiceType,
                });
            }
        } else {
            return Err(LintResult {
                severity: LintSeverity::Error,
                message: format!("No type set"),
                code: LintCode::ErrorNoServiceTypeSet,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
use systemd_parser;

#[test]
fn success_case() {
    let input = "
        [Service]
        Type=simple
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    let res = lint(&unit);
    assert!(res.is_ok());
}

#[test]
fn error_case() {
    let input = "
        [Service]
        Type=Simple
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    let res = lint(&unit);
    assert!(res.is_err());
}

#[test]
fn error2_case() {
    let input = "
        [Service]
        Type=
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    let res = lint(&unit);
    assert!(res.is_err());
}